use std::str::FromStr;

use cosmwasm_std::{testing::mock_env, to_binary, Addr, Binary, CosmosMsg, Uint256, WasmMsg};
use hpl_interface::{
    types::bech32_encode,
    warp::{self, cw20::TokenOption},
};
use rstest::rstest;

use crate::{error::ContractError, tests::TokenCW20, TOKEN};

#[rstest]
#[case("osmo")]
#[case("neutron")]
fn test_router_role(#[case] hrp: &str) -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let mailbox = Addr::unchecked("mailbox");
    let owner = Addr::unchecked("owner");

    let token = Addr::unchecked("token-native");
    let domain = 999;
    let router = Binary(b"hello".to_vec());

    let mut warp = TokenCW20::default();

    warp.init(
        &deployer,
        &owner,
        &mailbox,
        Some(TokenOption::Reuse {
            contract: token.to_string(),
        }),
        TokenMode::Bridged,
        hrp,
    )?;

    // err
    let err = warp
        .router_enroll(&mailbox, domain, router.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized);

    // ok
    warp.router_enroll(&owner, domain, router)?;

    Ok(())
}

#[rstest]
#[case("osmo")]
#[case("neutron")]
fn test_outbound_transfer(#[case] hrp: &str) -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let mailbox = Addr::unchecked("mailbox");
    let router = Addr::unchecked("router");
    let owner = Addr::unchecked("owner");

    let token = Addr::unchecked("token-cw20");
    let amount: u64 = 100_000;

    let user_remote = Addr::unchecked("user-remote____________________1");

    let dest_domain = 1;

    let env = mock_env();

    let burn_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&cw20::Cw20ExecuteMsg::Burn {
            amount: amount.into(),
        })?,
        funds: vec![],
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
            vec![(dest_domain, Binary(router.as_bytes().to_vec()))],
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
            vec![(dest_domain, Binary(router.as_bytes().to_vec()))],
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
        let mut warp = TokenCW20 {
            env: env.clone(),
            ..Default::default()
        };

        warp.init(
            &deployer,
            &owner,
            &mailbox,
            Some(TokenOption::Reuse {
                contract: token.to_string(),
            }),
            mode.clone(),
            hrp,
        )?;
        if mode == TokenMode::Collateral {
            TOKEN.save(&mut warp.deps.storage, &token)?;
        }

        for (domain, router) in routers {
            warp.router_enroll(&owner, domain, router)?;
        }

        let resp = warp.transfer_remote(
            &deployer,
            &token,
            amount.into(),
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

#[rstest]
#[case("osmo")]
#[case("neutron")]
fn test_inbound_transfer(#[case] hrp: &str) -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let mailbox = Addr::unchecked("mailbox");
    let router = Addr::unchecked("router");
    let owner = Addr::unchecked("owner");
    let errortic = Addr::unchecked("errortic");

    let token = Addr::unchecked("token-cw20");
    let amount = 100_000;

    let user_remote = Addr::unchecked("user-remote____________________1");

    let env = mock_env();

    let origin_domain = 1;

    let mint_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&cw20::Cw20ExecuteMsg::Mint {
            recipient: bech32_encode(hrp, user_remote.as_bytes())?.to_string(),
            amount: amount.into(),
        })?,
        funds: vec![],
    }
    .into();

    let send_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&cw20::Cw20ExecuteMsg::Transfer {
            recipient: bech32_encode(hrp, user_remote.as_bytes())?.to_string(),
            amount: amount.into(),
        })?,
        funds: vec![],
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
            Ok(vec![mint_msg]),
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
        let mut warp = TokenCW20 {
            env: env.clone(),
            ..Default::default()
        };

        warp.init(
            &deployer,
            &owner,
            &mailbox,
            Some(TokenOption::Reuse {
                contract: token.to_string(),
            }),
            mode.clone(),
            hrp,
        )?;
        if mode == TokenMode::Collateral {
            TOKEN.save(&mut warp.deps.storage, &token)?;
        }

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
