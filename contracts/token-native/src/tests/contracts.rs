use std::str::FromStr;

use cosmwasm_std::{
    coin, testing::mock_env, to_binary, Addr, BankMsg, Binary, CosmosMsg, Uint256, WasmMsg,
};
use hpl_interface::{
    mailbox,
    token::{self, TokenMode},
    types::bech32_encode,
};

use crate::{
    error::ContractError,
    proto::{self, MsgBurn, MsgMint},
};

use super::TokenNative;

#[test]
fn test_init() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let mailbox = Addr::unchecked("mailbox");
    let owner = Addr::unchecked("owner");

    let mut warp = TokenNative::default();

    warp.init(
        &deployer,
        &owner,
        &mailbox,
        "token-warp",
        None,
        TokenMode::Bridged,
    )?;

    Ok(())
}

#[test]
fn test_router_role() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let mailbox = Addr::unchecked("mailbox");
    let owner = Addr::unchecked("owner");

    let denom = "token-native";
    let domain = 999;
    let router = Binary(b"hello".to_vec());

    let mut warp = TokenNative::default();

    warp.init_hack(&deployer, &owner, &mailbox, denom, TokenMode::Bridged)?;

    // err
    let err = warp
        .router_enroll(&mailbox, domain, router.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized);

    // ok
    warp.router_enroll(&owner, domain, router)?;

    Ok(())
}

#[test]
fn test_outbound_transfer() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let mailbox = Addr::unchecked("mailbox");
    let router = Addr::unchecked("router");
    let owner = Addr::unchecked("owner");

    let denom = "token-native";
    let amount = 100_000;

    let user_remote = Addr::unchecked("user-remote");

    let dest_domain = 1;

    let env = mock_env();

    let burn_msg: CosmosMsg = MsgBurn {
        sender: env.contract.address.to_string(),
        amount: Some(proto::Coin {
            amount: amount.to_string(),
            denom: denom.to_string(),
        }),
    }
    .into();

    let dispatch_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: mailbox.to_string(),
        msg: to_binary(&mailbox::ExecuteMsg::Dispatch {
            dest_domain,
            recipient_addr: Binary(router.as_bytes().to_vec()).into(),
            msg_body: token::Message {
                recipient: Binary(user_remote.as_bytes().to_vec()),
                amount: Uint256::from_str(&amount.to_string())?,
                metadata: Binary::default(),
            }
            .into(),
        })?,
        funds: vec![],
    }
    .into();

    for (mode, routers, expected_resp) in [
        (
            TokenMode::Bridged,
            vec![(dest_domain, router.as_bytes().into())],
            Ok(vec![burn_msg, dispatch_msg.clone()]),
        ),
        (
            TokenMode::Bridged,
            vec![],
            Err(ContractError::NoRouter {
                domain: dest_domain,
            }),
        ),
        (
            TokenMode::Collateral,
            vec![(dest_domain, router.as_bytes().into())],
            Ok(vec![dispatch_msg]),
        ),
        (
            TokenMode::Collateral,
            vec![],
            Err(ContractError::NoRouter {
                domain: dest_domain,
            }),
        ),
    ] {
        let mut warp = TokenNative {
            env: env.clone(),
            ..Default::default()
        };

        warp.init_hack(&deployer, &owner, &mailbox, denom, mode)?;

        for (domain, router) in routers {
            warp.router_enroll(&owner, domain, router)?;
        }

        let resp = warp.transfer_remote(
            &owner,
            coin(amount, denom),
            dest_domain,
            user_remote.as_bytes().into(),
        );

        assert_eq!(
            resp.map(|v| v.messages.into_iter().map(|v| v.msg).collect::<Vec<_>>()),
            expected_resp
        );
    }

    Ok(())
}

#[test]
fn test_inbound_transfer() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let mailbox = Addr::unchecked("mailbox");
    let router = Addr::unchecked("router");
    let owner = Addr::unchecked("owner");
    let errortic = Addr::unchecked("errortic");

    let denom = "token-native";
    let amount = 100_000;

    let user_remote = Addr::unchecked("user-remote____________________1");

    let env = mock_env();

    let origin_domain = 1;

    let mint_msg: CosmosMsg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(proto::Coin {
            amount: amount.to_string(),
            denom: denom.to_string(),
        }),
    }
    .into();

    let send_msg: CosmosMsg = BankMsg::Send {
        to_address: bech32_encode("osmo", user_remote.as_bytes())?.to_string(),
        amount: vec![coin(amount, denom)],
    }
    .into();

    let default_msg = token::Message {
        recipient: user_remote.as_bytes().to_vec().into(),
        amount: Uint256::from_u128(amount),
        metadata: Binary::default(),
    };

    for (mode, sender, origin, origin_sender, token_msg, expected_resp) in [
        // happy
        (
            TokenMode::Bridged,
            &mailbox,
            origin_domain,
            &router,
            default_msg.clone(),
            Ok(vec![mint_msg, send_msg.clone()]),
        ),
        (
            TokenMode::Collateral,
            &mailbox,
            origin_domain,
            &router,
            default_msg.clone(),
            Ok(vec![send_msg]),
        ),
        // errors
        (
            TokenMode::Bridged,
            &errortic,
            origin_domain,
            &router,
            default_msg.clone(),
            Err(ContractError::Unauthorized),
        ),
        (
            TokenMode::Bridged,
            &mailbox,
            origin_domain,
            &errortic,
            default_msg.clone(),
            Err(ContractError::Unauthorized),
        ),
        (
            TokenMode::Collateral,
            &errortic,
            origin_domain,
            &router,
            default_msg.clone(),
            Err(ContractError::Unauthorized),
        ),
        (
            TokenMode::Collateral,
            &mailbox,
            origin_domain,
            &errortic,
            default_msg,
            Err(ContractError::Unauthorized),
        ),
    ] {
        let mut warp = TokenNative {
            env: env.clone(),
            ..Default::default()
        };

        warp.init_hack(&deployer, &owner, &mailbox, denom, mode)?;
        warp.router_enroll(&owner, origin_domain, router.as_bytes().into())?;

        let resp = warp.mailbox_handle(
            sender,
            mailbox::HandleMsg {
                origin,
                sender: origin_sender.as_bytes().to_vec().into(),
                body: token_msg.into(),
            },
        );

        assert_eq!(
            resp.map(|v| v.messages.into_iter().map(|v| v.msg).collect::<Vec<_>>()),
            expected_resp
        );
    }

    Ok(())
}
