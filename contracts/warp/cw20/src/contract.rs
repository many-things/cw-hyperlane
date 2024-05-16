#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, to_json_binary, wasm_execute, CosmosMsg, Deps, DepsMut, Empty, Env, HexBinary,
    MessageInfo, QueryResponse, Reply, Response, StdError, SubMsg, Uint128, Uint256, WasmMsg,
};

use cw20::Cw20ExecuteMsg;
use hpl_connection::{get_hook, get_ism};
use hpl_interface::{
    core::mailbox,
    ism::{InterchainSecurityModuleResponse, IsmSpecifierQueryMsg},
    to_binary,
    types::bech32_encode,
    warp::{
        self,
        cw20::{ExecuteMsg, InstantiateMsg, QueryMsg},
        TokenMode, TokenModeMsg, TokenModeResponse, TokenTypeResponse,
    },
};
use hpl_router::get_route;

use crate::{
    conv, error::ContractError, new_event, CONTRACT_NAME, CONTRACT_VERSION, HRP, MAILBOX, MODE,
    REPLY_ID_CREATE_DENOM, TOKEN,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mode: TokenMode = msg.token.clone().into();
    let owner = deps.api.addr_validate(&msg.owner)?;
    let mailbox = deps.api.addr_validate(&msg.mailbox)?;

    HRP.save(deps.storage, &msg.hrp)?;
    MODE.save(deps.storage, &mode)?;
    MAILBOX.save(deps.storage, &mailbox)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    let (msgs, denom) = match msg.token {
        TokenModeMsg::Bridged(token) => {
            let mut token_init_msg = token.init_msg;
            token_init_msg.mint = Some(cw20::MinterResponse {
                minter: env.contract.address.to_string(),
                cap: None,
            });

            let msgs = vec![SubMsg::reply_on_success(
                WasmMsg::Instantiate {
                    admin: Some(env.contract.address.to_string()),
                    code_id: token.code_id,
                    msg: to_json_binary(&token_init_msg)?,
                    funds: vec![],
                    label: "token warp cw20".to_string(),
                },
                REPLY_ID_CREATE_DENOM,
            )];

            (msgs, token_init_msg.name)
        }
        TokenModeMsg::Collateral(token) => {
            let token_addr = deps.api.addr_validate(&token.address)?;
            TOKEN.save(deps.storage, &token_addr)?;
            (vec![], token_addr.into())
        }
    };

    Ok(Response::new().add_submessages(msgs).add_event(
        new_event("instantiate")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("mode", format!("{mode}"))
            .add_attribute("denom", denom),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        Router(msg) => Ok(hpl_router::handle(deps, env, info, msg)?),
        Connection(msg) => Ok(hpl_connection::handle(deps, env, info, msg)?),
        Handle(msg) => mailbox_handle(deps, info, msg),
        TransferRemote {
            dest_domain,
            recipient,
            amount,
            hook,
            metadata,
        } => transfer_remote(
            deps,
            env,
            info,
            dest_domain,
            recipient,
            amount,
            hook,
            metadata,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_CREATE_DENOM => {
            let reply_data = msg
                .result
                .into_result()
                .map_err(StdError::generic_err)?
                .data
                .ok_or(StdError::generic_err("no reply data"))?;
            let init_resp = cw_utils::parse_instantiate_response_data(&reply_data)?;
            let init_addr = deps.api.addr_validate(&init_resp.contract_address)?;

            TOKEN.save(deps.storage, &init_addr)?;

            let resp = Response::new()
                .add_event(new_event("reply-init").add_attribute("new_token", init_addr));

            Ok(resp)
        }

        _ => Err(ContractError::InvalidReplyId),
    }
}

fn mailbox_handle(
    deps: DepsMut,
    info: MessageInfo,
    msg: hpl_interface::core::HandleMsg,
) -> Result<Response, ContractError> {
    // validate mailbox
    ensure_eq!(
        info.sender,
        MAILBOX.load(deps.storage)?,
        ContractError::Unauthorized
    );
    // validate origin chain router
    ensure_eq!(
        msg.sender,
        get_route::<HexBinary>(deps.storage, msg.origin)?
            .route
            .expect("route not found"),
        ContractError::Unauthorized
    );

    let token_msg: warp::Message = msg.body.into();
    let recipient = bech32_encode(&HRP.load(deps.storage)?, &token_msg.recipient)?;

    let token = TOKEN.load(deps.storage)?;
    let mode = MODE.load(deps.storage)?;

    let msg = match mode {
        // make token mint msg if token mode is bridged
        TokenMode::Bridged => conv::to_mint_msg(&token, &recipient, token_msg.amount)?,
        // make token transfer msg if token mode is collateral
        // we can consider to use MsgSend for further utility
        TokenMode::Collateral => conv::to_send_msg(&token, &recipient, token_msg.amount)?,
    };

    Ok(Response::new().add_message(msg).add_event(
        new_event("handle")
            .add_attribute("recipient", recipient)
            .add_attribute("token", token)
            .add_attribute("amount", token_msg.amount),
    ))
}

#[allow(clippy::too_many_arguments)]
fn transfer_remote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    dest_domain: u32,
    recipient: HexBinary,
    transfer_amount: Uint128,
    hook: Option<String>,
    metadata: Option<HexBinary>,
) -> Result<Response, ContractError> {
    let token = TOKEN.load(deps.storage)?;
    let mode = MODE.load(deps.storage)?;
    let mailbox = MAILBOX.load(deps.storage)?;

    let dest_router = get_route::<HexBinary>(deps.storage, dest_domain)?
        .route
        .expect("route not found");

    // validate hook if present
    if let Some(ref custom_hook) = hook {
        let _ = deps.api.addr_validate(custom_hook)?;
    }

    let mut msgs: Vec<CosmosMsg> = vec![];

    // push token transfer msg
    msgs.push(
        wasm_execute(
            &token,
            &Cw20ExecuteMsg::TransferFrom {
                owner: info.sender.to_string(),
                recipient: env.contract.address.to_string(),
                amount: transfer_amount,
            },
            vec![],
        )?
        .into(),
    );

    if mode == TokenMode::Bridged {
        // push token burn msg if token is bridged
        msgs.push(conv::to_burn_msg(&token, transfer_amount)?.into());
    }

    // push mailbox dispatch msg
    msgs.push(mailbox::dispatch(
        mailbox,
        dest_domain,
        dest_router,
        warp::Message {
            recipient: recipient.clone(),
            amount: Uint256::from_uint128(transfer_amount),
            metadata: HexBinary::default(),
        }
        .into(),
        hook.clone().or(get_hook(deps.storage)?.map(|v| v.into())),
        metadata.clone(),
        info.funds,
    )?);

    Ok(Response::new().add_messages(msgs).add_event(
        new_event("transfer-remote")
            .add_attribute("sender", info.sender)
            .add_attribute("dest_domain", dest_domain.to_string())
            .add_attribute("recipient", recipient.to_hex())
            .add_attribute("token", token)
            .add_attribute("amount", transfer_amount)
            .add_attribute("hook", hook.unwrap_or_default())
            .add_attribute("metadata", metadata.unwrap_or_default().to_string()),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use warp::TokenWarpDefaultQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Router(msg) => Ok(hpl_router::handle_query(deps, env, msg)?),
        QueryMsg::Connection(msg) => Ok(hpl_connection::handle_query(deps, env, msg)?),
        QueryMsg::TokenDefault(msg) => match msg {
            TokenType {} => to_binary(get_token_type(deps)),
            TokenMode {} => to_binary(get_token_mode(deps)),
        },
        QueryMsg::IsmSpecifier(IsmSpecifierQueryMsg::InterchainSecurityModule()) => {
            Ok(to_json_binary(&InterchainSecurityModuleResponse {
                ism: get_ism(deps.storage)?,
            })?)
        }
    }
}

fn get_token_type(deps: Deps) -> Result<TokenTypeResponse, ContractError> {
    let contract = TOKEN.load(deps.storage)?.into_string();

    Ok(TokenTypeResponse {
        typ: warp::TokenType::CW20 { contract },
    })
}

fn get_token_mode(deps: Deps) -> Result<TokenModeResponse, ContractError> {
    let mode = MODE.load(deps.storage)?;

    Ok(TokenModeResponse { mode })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps, Uint128,
    };
    use hpl_interface::{
        build_test_executor, build_test_querier,
        core::HandleMsg,
        router::DomainRouteSet,
        warp::cw20::{Cw20ModeBridged, Cw20ModeCollateral},
    };
    use hpl_router::set_routes;
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::{fixture, rstest};

    use super::*;

    build_test_querier!(super::query);
    build_test_executor!(super::execute);

    const DEPLOYER: &str = "sender";
    const OWNER: &str = "owner";
    const MAILBOX: &str = "mailbox";
    const TOKEN: &str = "token";
    const CUSTOM_HOOK: &str = "custom_hook";

    const CW20_BRIDGED_CODE_ID: u64 = 1;
    const CW20_BRIDGED_NAME: &str = "cw20-created";
    const CW20_COLLATERAL_ADDRESS: &str = "cw20-exisiting";

    type Cw20TokenMode = TokenModeMsg<Cw20ModeBridged, Cw20ModeCollateral>;
    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    #[fixture]
    fn token_mode_bridged() -> Cw20TokenMode {
        TokenModeMsg::Bridged(Cw20ModeBridged {
            code_id: CW20_BRIDGED_CODE_ID,
            init_msg: cw20_base::msg::InstantiateMsg {
                name: CW20_BRIDGED_NAME.to_string(),
                symbol: CW20_BRIDGED_NAME.to_string(),
                decimals: 1,
                initial_balances: vec![],
                mint: None,
                marketing: None,
            }
            .into(),
        })
    }

    #[fixture]
    fn token_mode_collateral() -> Cw20TokenMode {
        TokenModeMsg::Collateral(Cw20ModeCollateral {
            address: CW20_COLLATERAL_ADDRESS.to_string(),
        })
    }

    #[fixture]
    fn deps(
        #[default(vec![])] routes: Vec<(u32, HexBinary)>,
        #[default("osmo")] hrp: &str,
        #[default(Some(TOKEN))] token: Option<&str>,
        token_mode_collateral: Cw20TokenMode,
    ) -> (TestDeps, Response) {
        let mut deps = mock_dependencies();

        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(DEPLOYER, &[]),
            InstantiateMsg {
                token: token_mode_collateral,
                hrp: hrp.to_string(),
                owner: OWNER.to_string(),
                mailbox: MAILBOX.to_string(),
            },
        )
        .unwrap();

        if let Some(token) = token {
            super::TOKEN
                .save(deps.as_mut().storage, &addr(token))
                .unwrap();
        }

        if !routes.is_empty() {
            set_routes(
                deps.as_mut().storage,
                &addr(OWNER),
                routes
                    .into_iter()
                    .map(|v| DomainRouteSet {
                        domain: v.0,
                        route: Some(v.1),
                    })
                    .collect(),
            )
            .unwrap();
        }

        (deps, res)
    }

    #[rstest]
    #[case(token_mode_bridged())]
    #[case(token_mode_collateral())]
    fn test_queries(#[values("osmo", "neutron")] hrp: &str, #[case] token_mode: Cw20TokenMode) {
        let (deps, _) = deps(vec![], hrp, Some(TOKEN), token_mode.clone());

        let res: warp::TokenTypeResponse = test_query(
            deps.as_ref(),
            QueryMsg::TokenDefault(warp::TokenWarpDefaultQueryMsg::TokenType {}),
        );
        assert_eq!(
            res.typ,
            warp::TokenType::CW20 {
                contract: TOKEN.into()
            }
        );

        let res: warp::TokenModeResponse = test_query(
            deps.as_ref(),
            QueryMsg::TokenDefault(warp::TokenWarpDefaultQueryMsg::TokenMode {}),
        );
        assert_eq!(res.mode, token_mode.into());
    }

    #[rstest]
    #[case(token_mode_bridged())]
    #[case(token_mode_collateral())]
    fn test_init(#[values("osmo", "neutron")] hrp: &str, #[case] token_mode: Cw20TokenMode) {
        let (deps, res) = deps(vec![], hrp, None, token_mode.clone());

        let storage = deps.as_ref().storage;
        let mode = token_mode.clone().into();

        assert_eq!(super::HRP.load(storage).unwrap(), hrp);
        assert_eq!(super::MODE.load(storage).unwrap(), mode);
        assert_eq!(super::MAILBOX.load(storage).unwrap(), MAILBOX);

        match token_mode {
            TokenModeMsg::Bridged(mut v) => {
                v.init_msg.mint = Some(cw20::MinterResponse {
                    minter: mock_env().contract.address.into(),
                    cap: None,
                });

                assert!(!super::TOKEN.exists(storage));

                let reply = res.messages.get(0).unwrap();
                assert_eq!(reply.id, REPLY_ID_CREATE_DENOM);
                assert_eq!(
                    reply.msg,
                    CosmosMsg::Wasm(WasmMsg::Instantiate {
                        admin: Some(mock_env().contract.address.to_string()),
                        code_id: v.code_id,
                        msg: to_json_binary(&v.init_msg).unwrap(),
                        funds: vec![],
                        label: "token warp cw20".to_string()
                    })
                )
            }
            TokenModeMsg::Collateral(v) => {
                assert_eq!(super::TOKEN.load(storage).unwrap(), v.address);
                assert!(res.messages.is_empty())
            }
        }
    }

    #[rstest]
    #[case(MAILBOX, 1, gen_bz(32), token_mode_bridged())]
    #[case(MAILBOX, 1, gen_bz(32), token_mode_collateral())]
    #[should_panic(expected = "unauthorized")]
    #[case(TOKEN, 1, gen_bz(32), token_mode_collateral())]
    #[should_panic(expected = "route not found")]
    #[case(MAILBOX, 2, gen_bz(32), token_mode_collateral())]
    fn test_mailbox_handle(
        #[values("osmo", "neutron")] hrp: &str,
        #[case] sender: &str,
        #[case] domain: u32,
        #[case] route: HexBinary,
        #[case] token_mode: Cw20TokenMode,
    ) {
        let (mut deps, _) = deps(
            vec![(1, route.clone())],
            hrp,
            Some(TOKEN),
            token_mode.clone(),
        );

        let warp_msg = warp::Message {
            recipient: gen_bz(32),
            amount: Uint256::from_u128(100),
            metadata: HexBinary::default(),
        };

        let handle_msg = HandleMsg {
            origin: domain,
            sender: route,
            body: warp_msg.clone().into(),
        };

        let res = test_execute(
            deps.as_mut(),
            &addr(sender),
            ExecuteMsg::Handle(handle_msg),
            vec![],
        );
        let msg = &res.messages.get(0).unwrap().msg;

        match token_mode {
            TokenModeMsg::Bridged(_) => {
                assert_eq!(
                    to_json_binary(msg).unwrap(),
                    to_json_binary(&CosmosMsg::<Empty>::Wasm(
                        conv::to_mint_msg(
                            TOKEN,
                            bech32_encode(hrp, warp_msg.recipient.as_slice()).unwrap(),
                            warp_msg.amount
                        )
                        .unwrap()
                    ))
                    .unwrap()
                )
            }
            TokenModeMsg::Collateral(_) => {
                assert_eq!(
                    to_json_binary(msg).unwrap(),
                    to_json_binary(&CosmosMsg::<Empty>::Wasm(
                        conv::to_send_msg(
                            TOKEN,
                            bech32_encode(hrp, warp_msg.recipient.as_slice()).unwrap(),
                            warp_msg.amount
                        )
                        .unwrap()
                    ))
                    .unwrap()
                );
            }
        }
    }

    #[rstest]
    #[case(1, gen_bz(32), token_mode_bridged(), Some(CUSTOM_HOOK), None)]
    #[case(1, gen_bz(32), token_mode_collateral(), None, Some(gen_bz(100)))]
    #[should_panic(expected = "route not found")]
    #[case(2, gen_bz(32), token_mode_collateral(), None, None)]
    fn test_transfer_remote(
        #[values("osmo", "neutron")] hrp: &str,
        #[case] domain: u32,
        #[case] route: HexBinary,
        #[case] token_mode: Cw20TokenMode,
        #[case] custom_hook: Option<&str>,
        #[case] custom_metadata: Option<HexBinary>,
    ) {
        let (mut deps, _) = deps(
            vec![(1, route.clone())],
            hrp,
            Some(TOKEN),
            token_mode.clone(),
        );

        let sender = addr("sender");
        let recipient = gen_bz(32);

        let res = test_execute(
            deps.as_mut(),
            &sender,
            ExecuteMsg::TransferRemote {
                dest_domain: domain,
                recipient: recipient.clone(),
                amount: Uint128::new(100),
                hook: custom_hook.map(|h| h.to_string()),
                metadata: custom_metadata.clone(),
            },
            vec![],
        );
        let msgs = res.messages.into_iter().map(|v| v.msg).collect::<Vec<_>>();

        let transfer_from_msg = wasm_execute(
            TOKEN,
            &Cw20ExecuteMsg::TransferFrom {
                owner: sender.to_string(),
                recipient: mock_env().contract.address.to_string(),
                amount: Uint128::new(100),
            },
            vec![],
        )
        .unwrap();

        let warp_msg = warp::Message {
            recipient,
            amount: Uint256::from_u128(100),
            metadata: HexBinary::default(),
        };

        let dispatch_msg = mailbox::dispatch(
            MAILBOX,
            domain,
            route,
            warp_msg.into(),
            custom_hook.map(|h| h.to_string()),
            custom_metadata,
            vec![],
        )
        .unwrap();

        match token_mode {
            TokenModeMsg::Bridged(_) => {
                assert_eq!(
                    to_json_binary(&msgs).unwrap(),
                    to_json_binary(&vec![
                        transfer_from_msg.into(),
                        CosmosMsg::from(conv::to_burn_msg(TOKEN, Uint128::new(100)).unwrap()),
                        dispatch_msg,
                    ])
                    .unwrap(),
                );
            }
            TokenModeMsg::Collateral(_) => {
                assert_eq!(
                    to_json_binary(&msgs).unwrap(),
                    to_json_binary(&vec![transfer_from_msg.into(), dispatch_msg]).unwrap(),
                );
            }
        }
    }
}
