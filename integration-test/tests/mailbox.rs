mod event;
mod setup;
mod validator;

use cosmwasm_std::{attr, Attribute, Binary, HexBinary};
use ethers::{prelude::parse_log, signers::Signer};
use osmosis_test_tube::{Account, Module, Wasm};
use setup::setup_env;

use hpl_interface::types::{bech32_decode, bech32_encode, bech32_to_h256};
use hpl_tests::mailbox::{DispatchFilter, DispatchIdFilter};

use crate::{
    event::{parse_dispatch_from_res, parse_dispatch_id_from_res},
    setup::HplCwIsmType,
    validator::TestValidators,
};

fn sorted(mut attrs: Vec<Attribute>) -> Vec<Attribute> {
    attrs.sort_by(|a, b| a.key.cmp(&b.key));
    attrs
}

#[tokio::test]
async fn test_mailbox_cw_to_evm() -> eyre::Result<()> {
    let evm_domain = 1;
    let cw_domain = 2;

    let validators = TestValidators::new(evm_domain, 5, 3);

    let hrp = "osmo";
    let cw_ism_type = HplCwIsmType::Multisig {
        hrp,
        validators: validators.clone(),
    };
    let cw_routing_ism_type = HplCwIsmType::Routing(vec![(cw_domain, cw_ism_type)]);

    let test_env = setup_env(evm_domain, cw_domain, cw_routing_ism_type).await?;
    let cw_mailbox = test_env.cw_deployments.mailbox;
    let cw_wasm = Wasm::new(&test_env.osmo_app);
    let evm_mailbox = test_env.evm_deployments.mailbox;
    let evm_receiver = test_env.evm_deployments.msg_receiver;

    let mut receiver = [0u8; 32];
    receiver[12..].copy_from_slice(&evm_receiver.address().0);
    let _sender = bech32_decode(test_env.osmo_owner.address().as_str())?;
    let msg_body = b"hello world";

    // dispatch
    let dispatch_res = cw_wasm.execute(
        &cw_mailbox,
        &hpl_interface::mailbox::ExecuteMsg::Dispatch {
            dest_domain: evm_domain,
            recipient_addr: receiver.into(),
            msg_body: msg_body.into(),
        },
        &[],
        &test_env.osmo_owner,
    )?;

    let dispatch = parse_dispatch_from_res(&dispatch_res.events);
    let _dispatch_id = parse_dispatch_id_from_res(&dispatch_res.events);

    let process_tx = evm_mailbox.process(vec![].into(), Binary::from(dispatch.message).0.into());
    let _process_tx_res = process_tx.send().await?.await?.unwrap();

    Ok(())
}

#[tokio::test]
async fn test_mailbox_evm_to_cw() -> eyre::Result<()> {
    let evm_domain = 1;
    let cw_domain = 2;

    let validators = TestValidators::new(evm_domain, 5, 3);

    let hrp = "osmo";
    let cw_ism_type = HplCwIsmType::Multisig {
        hrp,
        validators: validators.clone(),
    };
    let cw_routing_ism_type = HplCwIsmType::Routing(vec![(evm_domain, cw_ism_type)]);

    let test_env = setup_env(evm_domain, cw_domain, cw_routing_ism_type).await?;
    let evm_mailbox = test_env.evm_deployments.mailbox;
    let cw_mailbox = test_env.cw_deployments.mailbox;
    let cw_receiver = test_env.cw_deployments.msg_receiver;
    let cw_wasm = Wasm::new(&test_env.osmo_app);

    let sender = bech32_encode("osmo", test_env.eth_owner.address().as_bytes())?;
    let receiver = bech32_to_h256(&cw_receiver)?;
    let msg_body = b"hello world";

    // dispatch
    let dispatch_tx_call = evm_mailbox.dispatch(cw_domain, receiver, msg_body.into());
    let dispatch_res = dispatch_tx_call.send().await?.await?.unwrap();

    let dispatch: DispatchFilter = parse_log(dispatch_res.logs[0].clone())?;
    let dispatch_id: DispatchIdFilter = parse_log(dispatch_res.logs[1].clone())?;

    // generate ism metadata
    let ism_metadata = validators.make_metadata(
        evm_mailbox.address(),
        evm_mailbox.root().await?,
        dispatch_id.message_id,
        true,
    );

    // process
    let process_res = cw_wasm.execute(
        &cw_mailbox,
        &hpl_interface::mailbox::ExecuteMsg::Process {
            metadata: ism_metadata.into(),
            message: HexBinary::from(dispatch.message.to_vec()),
        },
        &[],
        &test_env.osmo_owner,
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
                value: cw_receiver,
            },
            attr("sender", sender),
            attr("origin", evm_mailbox.local_domain().await?.to_string()),
            attr("body", std::str::from_utf8(msg_body)?),
        ]),
    );

    Ok(())
}
