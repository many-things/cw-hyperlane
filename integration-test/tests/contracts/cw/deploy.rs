use std::collections::BTreeMap;

use cosmwasm_schema::{cw_serde, serde::Serialize};
use cosmwasm_std::HexBinary;
use hpl_interface::{
    core::mailbox,
    router::{DomainRouteSet, RouterMsg},
    warp::{self, cw20::Cw20ModeBridged, native::NativeModeBriged},
};
use osmosis_test_tube::osmosis_std::types::cosmwasm::wasm::v1::MsgInstantiateContractResponse;
use test_tube::{Account, ExecuteResponse, Runner, SigningAccount, Wasm};

use super::{
    types::{Codes, CoreDeployments},
    Hook, Ism,
};

pub fn instantiate<'a, M: Serialize, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    code: u64,
    deployer: &SigningAccount,
    name: &str,
    msg: &M,
) -> ExecuteResponse<MsgInstantiateContractResponse> {
    wasm.instantiate(
        code,
        msg,
        Some(&deployer.address()),
        Some(name),
        &[],
        deployer,
    )
    .unwrap()
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_core<'a, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    owner: &SigningAccount,
    deployer: &SigningAccount,
    codes: &Codes,
    origin_domain: u32,
    hrp: &str,
    default_ism: Ism,
    default_hook: Hook,
    required_hook: Hook,
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
    )
    .data
    .address;

    // set default ism, hook, igp

    let default_ism = default_ism.deploy(wasm, codes, owner, deployer)?;
    let default_hook = default_hook.deploy(wasm, codes, mailbox.clone(), owner, deployer)?;
    let required_hook = required_hook.deploy(wasm, codes, mailbox.clone(), owner, deployer)?;

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

    let msg_receiver = instantiate(
        wasm,
        codes.test_mock_msg_receiver,
        deployer,
        "test_mock_msg_receiver",
        &ReceiverInitMsg {
            hrp: hrp.to_string(),
        },
    )
    .data
    .address;

    Ok(CoreDeployments {
        mailbox,
        default_ism,
        default_hook,
        required_hook,
        msg_receiver,
    })
}

#[allow(dead_code)]
pub fn deploy_warp_route_bridged<'a, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    owner: &SigningAccount,
    deployer: &SigningAccount,
    mailbox: &str,
    hrp: &str,
    codes: &Codes,
    denom: String,
    token_type: warp::TokenType,
) -> ExecuteResponse<MsgInstantiateContractResponse> {
    match token_type {
        warp::TokenType::Native(_) => instantiate(
            wasm,
            codes.warp_native,
            deployer,
            "warp-native",
            &warp::native::InstantiateMsg {
                token: warp::TokenModeMsg::Bridged(NativeModeBriged {
                    denom,
                    metadata: None,
                }),
                hrp: hrp.to_string(),
                owner: owner.address(),
                mailbox: mailbox.to_string(),
            },
        ),
        warp::TokenType::CW20 { .. } => instantiate(
            wasm,
            codes.warp_cw20,
            deployer,
            "warp-cw20",
            &warp::cw20::InstantiateMsg {
                token: warp::TokenModeMsg::Bridged(Cw20ModeBridged {
                    code_id: codes.cw20_base,
                    init_msg: Box::new(warp::cw20::Cw20InitMsg {
                        name: denom.clone(),
                        symbol: denom,
                        decimals: 6,
                        initial_balances: vec![],
                        mint: None,
                        marketing: None,
                    }),
                }),
                hrp: hrp.to_string(),
                owner: owner.address(),
                mailbox: mailbox.to_string(),
            },
        ),
        warp::TokenType::CW721 { .. } => todo!(),
    }
}

#[allow(dead_code)]
pub fn deploy_warp_route_collateral<'a, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    owner: &SigningAccount,
    deployer: &SigningAccount,
    mailbox: &str,
    hrp: &str,
    codes: &Codes,
    denom: String,
) -> ExecuteResponse<MsgInstantiateContractResponse> {
    if denom.starts_with(format!("{hrp}1").as_str()) {
        // cw20
        instantiate(
            wasm,
            codes.warp_cw20,
            deployer,
            &format!("warp-cw20-{denom}"),
            &warp::cw20::InstantiateMsg {
                token: warp::TokenModeMsg::Collateral(warp::cw20::Cw20ModeCollateral {
                    address: denom,
                }),
                hrp: hrp.to_string(),
                owner: owner.address(),
                mailbox: mailbox.to_string(),
            },
        )
    } else {
        // native
        instantiate(
            wasm,
            codes.warp_native,
            deployer,
            &format!("warp-native-{denom}"),
            &warp::native::InstantiateMsg {
                token: warp::TokenModeMsg::Collateral(warp::native::NativeModeCollateral { denom }),
                hrp: hrp.to_string(),
                owner: owner.address(),
                mailbox: mailbox.to_string(),
            },
        )
    }
}

#[allow(dead_code)]
pub fn link_warp_route<'a, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    owner: &SigningAccount,
    origin: String,
    remotes: BTreeMap<u32, HexBinary>,
) -> eyre::Result<()> {
    #[cw_serde]
    pub enum ExpectedExecuteMsg {
        Router(RouterMsg<HexBinary>),
    }

    wasm.execute(
        &origin,
        &ExpectedExecuteMsg::Router(RouterMsg::SetRoutes {
            set: remotes
                .into_iter()
                .map(|(domain, route)| DomainRouteSet {
                    domain,
                    route: Some(route),
                })
                .collect(),
        }),
        &[],
        owner,
    )?;

    Ok(())
}
