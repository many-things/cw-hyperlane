mod setup;

use cosmwasm_std::{attr, Attribute, HexBinary};
use ethers::{prelude::parse_log, signers::Signer};
use hpl_interface::types::{bech32_encode, bech32_to_h256};
use hpl_tests::mailbox::{DispatchFilter, DispatchIdFilter};
use osmosis_test_tube::{Module, Wasm};
use setup::setup_env;

fn sorted(mut attrs: Vec<Attribute>) -> Vec<Attribute> {
    attrs.sort_by(|a, b| a.key.cmp(&b.key));
    attrs
}

async fn test_mailbox_inner() -> eyre::Result<()> {
    let test_env = setup_env().await?;
    let evm_mailbox = test_env.evm_deployments.mailbox;
    let cw_mailbox = test_env.cw_deployments.addrs.mailbox;
    let cw_receiver = test_env.cw_deployments.addrs.receiver;
    let cw_wasm = Wasm::new(&test_env.osmo_app);

    let sender = bech32_encode("osmo", test_env.eth_owner.address().as_bytes())?;
    let receiver = bech32_to_h256(&cw_receiver)?;
    let msg_body = b"hello world";

    let dispatch_tx_call = evm_mailbox.dispatch(2, receiver, msg_body.into());
    let dispatch_res = dispatch_tx_call.send().await?.await?.unwrap();

    let dispatch: DispatchFilter = parse_log(dispatch_res.logs[0].clone())?;
    let _: DispatchIdFilter = parse_log(dispatch_res.logs[1].clone())?;

    let process_res = cw_wasm.execute(
        &cw_mailbox,
        &hpl_interface::mailbox::ExecuteMsg::Process {
            metadata: HexBinary::default(),
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
            attr("_contract_address", cw_receiver),
            attr("sender", sender),
            attr("origin", evm_mailbox.local_domain().await?.to_string()),
            attr("body", std::str::from_utf8(msg_body)?),
        ]),
    );

    Ok(())
}

#[tokio::test]
async fn test_mailbox() {
    test_mailbox_inner().await.unwrap();
}
