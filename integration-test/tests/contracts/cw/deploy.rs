use cosmwasm_schema::cw_serde;
use test_tube::{Account, Runner, SigningAccount, Wasm};

use super::{
    types::{Codes, CoreDeployments},
    Hook, Ism,
};

pub fn deploy_core<'a, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    deployer: &SigningAccount,
    codes: &Codes,
    origin_domain: u32,
    hrp: &str,
    test_ism: Ism,
    test_hook: Hook,
) -> eyre::Result<CoreDeployments> {
    // deploy hub
    let hub = wasm
        .instantiate(
            codes.hub,
            &hpl_interface::hub::InstantiateMsg {
                origin_domain,
                mailbox_code: codes.mailbox,
            },
            Some(deployer.address().as_str()),
            Some("cw-hpl-hub"),
            &[],
            deployer,
        )?
        .data
        .address;

    // deploy mailbox through hub
    let mailbox_deploy_res = wasm.execute(
        &hub,
        &hpl_interface::hub::ExecuteMsg::Instantiate {
            owner: deployer.address(),
            default_ism: deployer.address(),  // temporary
            default_hook: deployer.address(), // temporary
        },
        &[],
        deployer,
    )?;

    let mailbox_deploy_evt = mailbox_deploy_res
        .events
        .into_iter()
        .find(|v| v.ty == "wasm-mailbox_instantiated")
        .unwrap();

    let mailbox = mailbox_deploy_evt.attributes.get(0).unwrap().value.clone();

    // set default ism, hook
    let ism = test_ism.deploy(wasm, codes, deployer)?;
    let hook = test_hook.deploy(wasm, codes, mailbox.clone(), deployer)?;

    wasm.execute(
        &mailbox,
        &hpl_interface::mailbox::ExecuteMsg::SetDefaultISM { ism: ism.clone() },
        &[],
        deployer,
    )?;

    wasm.execute(
        &mailbox,
        &hpl_interface::mailbox::ExecuteMsg::SetDefaultHook { hook: hook.clone() },
        &[],
        deployer,
    )?;

    // deploy test message receiver
    #[cw_serde]
    struct ReceiverInitMsg {
        pub hrp: String,
    }

    let msg_receiver = wasm
        .instantiate(
            codes.test_mock_msg_receiver,
            &ReceiverInitMsg {
                hrp: hrp.to_string(),
            },
            None,
            None,
            &[],
            deployer,
        )
        .unwrap()
        .data
        .address;

    Ok(CoreDeployments {
        hub,
        ism,
        hook,
        mailbox,
        msg_receiver,
    })
}
