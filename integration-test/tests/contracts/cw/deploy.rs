use cosmwasm_schema::{cw_serde, serde::Serialize};
use hpl_interface::core::mailbox;
use test_tube::{Account, Runner, SigningAccount, Wasm};

use crate::contracts::cw::igp::IgpDeployment;

use super::{
    igp::Igp,
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
    test_igp: Igp,
    test_default_ism: Ism,
    test_default_hook: Hook,
    test_required_hook: Hook,
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

    // set default ism, hook, igp
    let IgpDeployment {
        core: igp,
        oracle: igp_oracle,
    } = test_igp.deploy(wasm, codes, mailbox.clone(), deployer)?;

    let default_ism = test_default_ism.deploy(wasm, codes, deployer)?;
    let default_hook = test_default_hook.deploy(wasm, codes, mailbox.clone(), deployer)?;
    let required_hook = test_required_hook.deploy(wasm, codes, mailbox.clone(), deployer)?;

    wasm.execute(
        &mailbox,
        &mailbox::ExecuteMsg::SetDefaultIsm {
            ism: default_ism.clone(),
        },
        &[],
        deployer,
    )?;

    wasm.execute(
        &mailbox,
        &mailbox::ExecuteMsg::SetDefaultHook {
            hook: default_hook.clone(),
        },
        &[],
        deployer,
    )?;

    wasm.execute(
        &mailbox,
        &mailbox::ExecuteMsg::SetRequiredHook {
            hook: required_hook.clone(),
        },
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
        mailbox,
        igp,
        igp_oracle,
        default_ism,
        default_hook,
        required_hook,
        msg_receiver,
    })
}
