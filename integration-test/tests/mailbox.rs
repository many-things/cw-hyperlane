mod contracts;
mod event;
mod validator;

use cosmwasm_std::{attr, Attribute, Binary, HexBinary};
use ethers::{prelude::parse_log, signers::Signer};
use osmosis_test_tube::{Account, Module, OsmosisTestApp, Wasm};

use hpl_interface::types::{bech32_decode, bech32_encode, bech32_to_h256};

use crate::{
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

const DOMAIN_EVM: u32 = 1;

const DOMAIN_OSMO: u32 = 2;
const PREFIX_OSMO: &str = "osmo";

const DOMAIN_NTRN: u32 = 3;
const PREFIX_NTRN: &str = "neutron";

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
    )?;

    // init Anvil env
    let anvil1 = eth::setup_env(DOMAIN_EVM).await?;

    let mut receiver = [0u8; 32];
    receiver[12..].copy_from_slice(&anvil1.core.msg_receiver.address().0);
    let _sender = bech32_decode(osmo.acc_tester.address().as_str())?;
    let msg_body = b"hello world";

    // dispatch
    let dispatch_res = Wasm::new(osmo.app).execute(
        &osmo.core.mailbox,
        &hpl_interface::mailbox::ExecuteMsg::Dispatch {
            dest_domain: DOMAIN_EVM,
            recipient_addr: receiver.into(),
            msg_body: msg_body.into(),
        },
        &[],
        &osmo.acc_tester,
    )?;

    let dispatch = parse_dispatch_from_res(&dispatch_res.events);
    let _dispatch_id = parse_dispatch_id_from_res(&dispatch_res.events);

    let process_tx = anvil1
        .core
        .mailbox
        .process(vec![].into(), Binary::from(dispatch.message).0.into());
    let _process_tx_res = process_tx.send().await?.await?.unwrap();

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
        dispatch_id.message_id,
        true,
    )?;

    // process
    let process_res = Wasm::new(osmo.app).execute(
        &osmo.core.mailbox,
        &hpl_interface::mailbox::ExecuteMsg::Process {
            metadata: ism_metadata.into(),
            message: HexBinary::from(dispatch.message.to_vec()),
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
