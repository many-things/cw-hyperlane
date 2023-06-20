mod setup;

use cosmwasm_std::HexBinary;
use ethers::prelude::parse_log;
use hpl_interface::types::bech32_to_h256;
use hpl_tests::mailbox::{DispatchFilter, DispatchIdFilter};
use osmosis_test_tube::{Module, Wasm};
use setup::setup_env;

async fn test_mailbox_inner() -> eyre::Result<()> {
    let test_env = setup_env().await?;
    let evm_mailbox = test_env.evm_deployments.mailbox;
    let cw_mailbox = test_env.cw_deployments.addrs.mailbox;
    let cw_receiver = test_env.cw_deployments.addrs.receiver;
    let cw_wasm = Wasm::new(&test_env.osmo_app);

    let receiver = bech32_to_h256(&cw_receiver)?;
    let msg_body = b"hello world";

    let dispatch_tx_call = evm_mailbox.dispatch(2, receiver, msg_body.into());
    let dispatch_res = dispatch_tx_call.send().await?.await?.unwrap();

    let dispatch: DispatchFilter = parse_log(dispatch_res.logs[0].clone())?;
    let dispatch_id: DispatchIdFilter = parse_log(dispatch_res.logs[1].clone())?;

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

    let process_recv_evt_attrs = process_recv_evt
        .attributes
        .iter()
        .map(|v| format!("{}: {}", v.key, v.value))
        .collect::<Vec<_>>();

    assert_eq!(
        process_recv_evt_attrs,
        vec![
            format!("_contract_address: {cw_receiver}"),
            format!("body: {}", std::str::from_utf8(msg_body)?),
            format!("origin: {}", evm_mailbox.local_domain().await?),
            format!("sender")
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_mailbox() {
    test_mailbox_inner().await.unwrap();
}
