use cosmwasm_schema::{cw_serde, serde::Serialize};
use hpl_interface::core::mailbox;
use test_tube::{Account, Runner, SigningAccount, Wasm};

use super::{
    types::{Codes, CoreDeployments},
    Hook, Ism,
};

fn instantiate<'a, M: Serialize, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    code: u64,
    deployer: &SigningAccount,
    name: &str,
    msg: &M,
) -> eyre::Result<String> {
    Ok(wasm
        .instantiate(
            code,
            msg,
            Some(&deployer.address()),
            Some(name),
            &[],
            deployer,
        )?
        .data
        .address)
}

pub fn deploy_core<'a, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    deployer: &SigningAccount,
    codes: &Codes,
    origin_domain: u32,
    hrp: &str,
    test_ism: Ism,
    test_hook: Hook,
) -> eyre::Result<CoreDeployments> {
    // Deploy mailbox
    let mailbox = instantiate(
        wasm,
        codes.mailbox,
        deployer,
        "mailbox",
        &mailbox::InstantiateMsg {
            hrp: hrp.to_string(),
            owner: deployer.address(),
            domain: origin_domain,
        },
    )?;

    // set default ism, hook
    let ism = test_ism.deploy(wasm, codes, deployer)?;
    let hook = test_hook.deploy(wasm, codes, mailbox.clone(), deployer)?;

    wasm.execute(
        &mailbox,
        &mailbox::ExecuteMsg::SetDefaultIsm { ism: ism.clone() },
        &[],
        deployer,
    )?;

    wasm.execute(
        &mailbox,
        &mailbox::ExecuteMsg::SetDefaultHook { hook: hook.clone() },
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
        ism,
        hook,
        mailbox,
        msg_receiver,
    })
}
