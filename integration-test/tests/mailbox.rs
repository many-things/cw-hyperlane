#[allow(dead_code)]
mod constants;
mod contracts;
mod event;
mod validator;

use cosmwasm_std::{attr, coin, Attribute, Binary, Uint128};
use ethers::{
    prelude::parse_log, providers::Middleware, signers::Signer, types::TransactionReceipt,
};
use osmosis_test_tube::{Account, Module, OsmosisTestApp, Wasm};

use hpl_interface::{
    core::mailbox::{self, DispatchMsg},
    igp::oracle::RemoteGasDataConfig,
    types::{bech32_decode, bech32_encode, bech32_to_h256},
};
use test_tube::Runner;

use crate::{
    constants::*,
    contracts::{
        cw,
        eth::{
            self,
            mailbox::{DispatchFilter, DispatchIdFilter},
        },
    },
    event::{parse_dispatch_from_res, parse_dispatch_id_from_res},
    validator::TestValidators,
};

fn sorted(mut attrs: Vec<Attribute>) -> Vec<Attribute> {
    attrs.sort_by(|a, b| a.key.cmp(&b.key));
    attrs
}

async fn send_msg<'a, M, S, R>(
    anvil: &eth::Env<M, S>,
    cosmos: &cw::Env<'a, R>,
) -> eyre::Result<TransactionReceipt>
where
    M: Middleware + 'static,
    S: Signer + 'static,
    R: Runner<'a>,
{
    let mut receiver = [0u8; 32];
    receiver[12..].copy_from_slice(&anvil.core.msg_receiver.address().0);
    let _sender = bech32_decode(cosmos.acc_tester.address().as_str())?;
    let msg_body = b"hello world";

    // dispatch
    let dispatch_res = Wasm::new(cosmos.app).execute(
        &cosmos.core.mailbox,
        &mailbox::ExecuteMsg::Dispatch(DispatchMsg {
            dest_domain: DOMAIN_EVM,
            recipient_addr: receiver.into(),
            msg_body: msg_body.into(),
            hook: None,
            metadata: None,
        }),
        &[coin(56_000_000, "uosmo")],
        &cosmos.acc_tester,
    )?;

    let dispatch = parse_dispatch_from_res(&dispatch_res.events);
    let _dispatch_id = parse_dispatch_id_from_res(&dispatch_res.events);

    let process_tx = anvil
        .core
        .mailbox
        .process(vec![].into(), Binary::from(dispatch.message).0.into());
    let process_tx_res = process_tx.send().await?.await?.unwrap();

    Ok(process_tx_res)
}

#[tokio::test]
async fn test_mailbox_cw_to_evm() -> eyre::Result<()> {
    // init Osmosis env
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        None::<&str>,
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    // init Anvil env
    let anvil1 = eth::setup_env(DOMAIN_EVM).await?;

    let _ = send_msg(&anvil1, &osmo).await?;

    Ok(())
}

#[tokio::test]
async fn test_mailbox_evm_to_cw() -> eyre::Result<()> {
    // init Osmosis env
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        None::<&str>,
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    // init Anvil env
    let anvil1 = eth::setup_env(DOMAIN_EVM).await?;

    // prepare message arguments
    let sender = bech32_encode("osmo", anvil1.acc_owner.address().as_bytes())?;
    let receiver = bech32_to_h256(&osmo.core.msg_receiver)?;
    let msg_body = b"hello world";

    // dispatch
    let dispatch_tx_call = anvil1
        .core
        .mailbox
        .dispatch(DOMAIN_OSMO, receiver, msg_body.into());
    let dispatch_res = dispatch_tx_call.send().await?.await?.unwrap();

    let dispatch: DispatchFilter = parse_log(dispatch_res.logs[0].clone())?;
    let dispatch_id: DispatchIdFilter = parse_log(dispatch_res.logs[1].clone())?;

    // generate ism metadata
    let ism_metadata = osmo.get_validator_set(DOMAIN_EVM)?.make_metadata(
        anvil1.core.mailbox.address(),
        anvil1.core.mailbox.root().await?,
        anvil1.core.mailbox.count().await?,
        dispatch_id.message_id,
        true,
    )?;

    // process
    let process_res = Wasm::new(osmo.app).execute(
        &osmo.core.mailbox,
        &mailbox::ExecuteMsg::Process {
            metadata: ism_metadata.into(),
            message: dispatch.message.to_vec().into(),
        },
        &[],
        &osmo.acc_owner,
    )?;
    let process_recv_evt = process_res
        .events
        .iter()
        .find(|v| v.ty == "wasm-mailbox_msg_received")
        .unwrap();

    assert_eq!(
        process_recv_evt.attributes,
        sorted(vec![
            Attribute {
                key: "_contract_address".to_string(),
                value: osmo.core.msg_receiver,
            },
            attr("sender", sender),
            attr(
                "origin",
                anvil1.core.mailbox.local_domain().await?.to_string()
            ),
            attr("body", std::str::from_utf8(msg_body)?),
        ]),
    );

    Ok(())
}
