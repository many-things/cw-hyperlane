#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, to_json_binary, CosmosMsg, Deps, DepsMut, Empty, Env, HexBinary,
    MessageInfo, QueryResponse, Reply, Response, StdError, SubMsg, Uint128, Uint256,
};
use hpl_connection::{get_hook, get_ism};
use hpl_interface::{
    core::mailbox,
    ism::{InterchainSecurityModuleResponse, IsmSpecifierQueryMsg},
    to_binary,
    types::bech32_encode,
    warp::{
        self,
        native::{ExecuteMsg, InstantiateMsg, QueryMsg},
    },
    warp::{TokenMode, TokenModeMsg, TokenModeResponse, TokenTypeResponse},
};
use hpl_router::get_route;

use crate::{
    conv,
    error::ContractError,
    new_event,
    proto::{MsgCreateDenom, MsgCreateDenomResponse},
    CONTRACT_NAME, CONTRACT_VERSION, HRP, MAILBOX, MODE, REPLY_ID_CREATE_DENOM, TOKEN,
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

    HRP.save(deps.storage, &msg.hrp)?;
    MODE.save(deps.storage, &mode)?;
    MAILBOX.save(deps.storage, &deps.api.addr_validate(&msg.mailbox)?)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    let (msgs, denom) = match msg.token {
        // create native denom if token is bridged
        TokenModeMsg::Bridged(token) => {
            let mut msgs = vec![];

            msgs.push(SubMsg::reply_on_success(
                MsgCreateDenom {
                    sender: env.contract.address.to_string(),
                    subdenom: token.denom.clone(),
                },
                REPLY_ID_CREATE_DENOM,
            ));

            if let Some(metadata) = token.metadata {
                msgs.push(SubMsg::new(conv::to_set_metadata_msg(
                    &env.contract.address,
                    metadata,
                )));
            }

            (
                msgs,
                format!("factory/{}/{}", env.contract.address, token.denom),
            )
        }
        // use denom directly if token is native
        TokenModeMsg::Collateral(token) => {
            TOKEN.save(deps.storage, &token.denom)?;
            (vec![], token.denom)
        }
    };

    Ok(Response::new().add_submessages(msgs).add_event(
        new_event("instantiate")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("mode", format!("{}", mode))
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
        Handle(msg) => mailbox_handle(deps, env, info, msg),
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
    let reply_data = msg
        .result
        .into_result()
        .map_err(StdError::generic_err)?
        .data
        .ok_or(StdError::generic_err("no reply data"))?;

    match msg.id {
        REPLY_ID_CREATE_DENOM => {
            let reply: MsgCreateDenomResponse = reply_data.try_into()?;

            TOKEN.save(deps.storage, &reply.new_token_denom)?;

            let resp = Response::new()
                .add_event(new_event("reply-init").add_attribute("denom", reply.new_token_denom));

            Ok(resp)
        }

        _ => Err(ContractError::InvalidReplyId),
    }
}

fn mailbox_handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: hpl_interface::core::HandleMsg,
) -> Result<Response, ContractError> {
    // validate mailbox
    ensure_eq!(
        info.sender,
        MAILBOX.load(deps.storage)?,
        ContractError::Unauthorized
    );
    // validate message origin - this should be registered route
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

    let mut msgs: Vec<CosmosMsg> = vec![];

    if mode == TokenMode::Bridged {
        // push token mint msg if token is bridged
        msgs.push(conv::to_mint_msg(&env.contract.address, &token, token_msg.amount).into());
    }

    // push token send msg
    msgs.push(
        conv::to_send_msg(
            &recipient,
            vec![conv::to_coin_u256(token_msg.amount, &token)?],
        )
        .into(),
    );

    Ok(Response::new().add_messages(msgs).add_event(
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

    let mut funds = info.funds.clone();

    let (token_index, token_received) = funds
        .iter()
        .enumerate()
        .find(|(_, v)| v.denom == token)
        .expect("no funds sent");
    ensure!(
        token_received.amount >= transfer_amount,
        ContractError::InsufficientFunds
    );

    funds[token_index].amount -= transfer_amount;

    let dest_router = get_route::<HexBinary>(deps.storage, dest_domain)?
        .route
        .expect("route not found");

    // validate hook if present
    if let Some(ref custom_hook) = hook {
        let _ = deps.api.addr_validate(custom_hook)?;
    }

    let mut msgs: Vec<CosmosMsg> = vec![];

    if mode == TokenMode::Bridged {
        // push token burn msg if token is bridged
        msgs.push(conv::to_burn_msg(&env.contract.address, &token, transfer_amount).into());
    }

    let dispatch_payload = warp::Message {
        recipient: recipient.clone(),
        amount: Uint256::from_uint128(transfer_amount),
        metadata: HexBinary::default(),
    };

    // push mailbox dispatch msg
    msgs.push(mailbox::dispatch(
        mailbox,
        dest_domain,
        dest_router,
        dispatch_payload.into(),
        hook.clone().or(get_hook(deps.storage)?.map(|v| v.into())),
        metadata.clone(),
        funds,
    )?);

    Ok(Response::new().add_messages(msgs).add_event(
        new_event("transfer-remote")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", recipient.to_hex())
            .add_attribute("token", token)
            .add_attribute("amount", transfer_amount.to_string())
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
    let denom = TOKEN.load(deps.storage)?;

    Ok(TokenTypeResponse {
        typ: warp::TokenType::Native(warp::TokenTypeNative::Fungible { denom }),
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
        coin,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Coin, OwnedDeps, Uint128,
    };
    use hpl_interface::{
        build_test_executor, build_test_querier,
        core::HandleMsg,
        router::DomainRouteSet,
        warp::native::{Metadata, NativeModeBriged, NativeModeCollateral},
    };
    use hpl_router::set_route;
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::{fixture, rstest};

    use super::*;

    build_test_executor!(super::execute);
    build_test_querier!(super::query);

    type NativeTokenMode = TokenModeMsg<NativeModeBriged, NativeModeCollateral>;
    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    const DEPLOYER: &str = "deployer";
    const OWNER: &str = "owner";
    const MAILBOX: &str = "mailbox";
    const DENOM: &str = "utest";
    const CUSTOM_HOOK: &str = "custom_hook";

    #[fixture]
    fn metadata(#[default(true)] empty: bool) -> Option<Metadata> {
        if empty {
            None
        } else {
            Some(Metadata {
                description: "testtesttest".into(),
                denom_units: vec![],
                base: "basebasebase".into(),
                display: "displaydisplaydisplay".into(),
                name: DENOM.into(),
                symbol: DENOM.into(),
            })
        }
    }

    #[fixture]
    fn token_mode_bridged(metadata: Option<Metadata>) -> NativeTokenMode {
        TokenModeMsg::Bridged(NativeModeBriged {
            denom: DENOM.into(),
            metadata,
        })
    }

    #[fixture]
    fn token_mode_collateral() -> NativeTokenMode {
        TokenModeMsg::Collateral(NativeModeCollateral {
            denom: DENOM.into(),
        })
    }

    #[fixture]
    fn deps(
        #[default(token_mode_collateral())] token_mode: NativeTokenMode,
        #[default("osmo")] hrp: &str,
    ) -> TestDeps {
        let mut deps = mock_dependencies();

        super::instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(DEPLOYER, &[]),
            super::InstantiateMsg {
                token: token_mode,
                hrp: hrp.into(),
                owner: OWNER.into(),
                mailbox: MAILBOX.into(),
            },
        )
        .unwrap();

        deps
    }

    #[rstest]
    #[case(token_mode_bridged(metadata(true)))]
    #[case(token_mode_bridged(metadata(false)))]
    #[case(token_mode_collateral())]
    fn test_queries(#[values("osmo", "neutron")] hrp: &str, #[case] token_mode: NativeTokenMode) {
        let mut deps = deps(token_mode.clone(), hrp);

        if TokenMode::from(token_mode.clone()) == TokenMode::Bridged {
            super::TOKEN
                .save(deps.as_mut().storage, &DENOM.into())
                .unwrap();
        }

        let res: warp::TokenTypeResponse = test_query(
            deps.as_ref(),
            QueryMsg::TokenDefault(warp::TokenWarpDefaultQueryMsg::TokenType {}),
        );
        assert_eq!(
            res.typ,
            warp::TokenType::Native(warp::TokenTypeNative::Fungible {
                denom: DENOM.into()
            })
        );

        let res: warp::TokenModeResponse = test_query(
            deps.as_ref(),
            QueryMsg::TokenDefault(warp::TokenWarpDefaultQueryMsg::TokenMode {}),
        );
        assert_eq!(res.mode, token_mode.into());
    }

    #[rstest]
    #[case(token_mode_bridged(metadata(true)))]
    #[case(token_mode_bridged(metadata(false)))]
    #[case(token_mode_collateral())]
    fn test_init(#[values("osmo", "neutron")] hrp: &str, #[case] token_mode: NativeTokenMode) {
        let mut deps = mock_dependencies();

        let res = super::instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(DEPLOYER, &[]),
            super::InstantiateMsg {
                token: token_mode.clone(),
                hrp: hrp.into(),
                owner: OWNER.into(),
                mailbox: MAILBOX.into(),
            },
        )
        .unwrap();

        let storage = deps.as_ref().storage;
        assert_eq!(super::HRP.load(storage).unwrap(), hrp);
        assert_eq!(
            super::MODE.load(storage).unwrap(),
            token_mode.clone().into()
        );
        assert_eq!(super::MAILBOX.load(storage).unwrap(), MAILBOX);

        match token_mode {
            TokenModeMsg::Bridged(v) => {
                if v.metadata.is_some() {
                    assert_eq!(res.messages.len(), 2);
                } else {
                    assert_eq!(res.messages.len(), 1);
                }
            }
            TokenModeMsg::Collateral(_) => {
                assert_eq!(res.messages.len(), 0);
                assert_eq!(super::TOKEN.load(storage).unwrap(), DENOM);
            }
        }
    }

    #[rstest]
    #[case(MAILBOX, 1, gen_bz(32))]
    #[should_panic(expected = "unauthorized")]
    #[case(OWNER, 1, gen_bz(32))]
    #[should_panic(expected = "route not found")]
    #[case(MAILBOX, 2, gen_bz(32))]
    fn test_mailbox_handle(
        mut deps: TestDeps,
        #[case] sender: &str,
        #[case] origin_domain: u32,
        #[case] origin_sender: HexBinary,
    ) {
        let recipient = gen_bz(32);

        let handle_msg = HandleMsg {
            origin: origin_domain,
            sender: origin_sender.clone(),
            body: warp::Message {
                recipient: recipient.clone(),
                amount: Uint256::from_u128(100),
                metadata: HexBinary::default(),
            }
            .into(),
        };

        set_route(
            deps.as_mut().storage,
            &addr(OWNER),
            DomainRouteSet {
                domain: 1,
                route: Some(origin_sender),
            },
        )
        .unwrap();

        let res = test_execute(
            deps.as_mut(),
            &addr(sender),
            ExecuteMsg::Handle(handle_msg),
            vec![],
        );
        let mut msgs: Vec<_> = res.messages.into_iter().map(|v| v.msg).collect();

        let mode = MODE.load(deps.as_ref().storage).unwrap();

        assert_eq!(
            msgs.pop().unwrap(),
            conv::to_send_msg(
                &bech32_encode("osmo", recipient.as_slice()).unwrap(),
                vec![coin(100, DENOM)]
            )
            .into()
        );

        if mode == TokenMode::Bridged {
            assert_eq!(
                msgs.pop().unwrap(),
                conv::to_mint_msg(&mock_env().contract.address, DENOM, "100").into()
            );
        } else {
            assert!(msgs.is_empty());
        }
    }

    #[rstest]
    #[case(1, gen_bz(32), gen_bz(32), vec![coin(100, DENOM)], Some(CUSTOM_HOOK), None)]
    #[case(1, gen_bz(32), gen_bz(32), vec![coin(100, DENOM), coin(100, "uatom")], None, Some(gen_bz(100)))]
    #[should_panic(expected = "route not found")]
    #[case(2, gen_bz(32), gen_bz(32), vec![coin(100, DENOM)], None, None)]
    #[should_panic(expected = "no funds sent")]
    #[case(1, gen_bz(32), gen_bz(32), vec![], None, None)]
    #[should_panic(expected = "no funds sent")]
    #[case(1, gen_bz(32), gen_bz(32), vec![coin(100, "uatom")], None, None)]
    fn test_transfer_remote(
        mut deps: TestDeps,
        #[case] dest_domain: u32,
        #[case] dest_router: HexBinary,
        #[case] dest_recipient: HexBinary,
        #[case] funds: Vec<Coin>,
        #[case] custom_hook: Option<&str>,
        #[case] custom_metadata: Option<HexBinary>,
    ) {
        set_route(
            deps.as_mut().storage,
            &addr(OWNER),
            DomainRouteSet {
                domain: 1,
                route: Some(dest_router.clone()),
            },
        )
        .unwrap();

        let res = test_execute(
            deps.as_mut(),
            &addr("sender"),
            ExecuteMsg::TransferRemote {
                dest_domain,
                recipient: dest_recipient.clone(),
                amount: Uint128::new(50),
                hook: custom_hook.map(|h| h.to_string()),
                metadata: custom_metadata.clone(),
            },
            funds.clone(),
        );
        let mut msgs: Vec<_> = res.messages.into_iter().map(|v| v.msg).collect();

        let mode = MODE.load(deps.as_ref().storage).unwrap();

        assert_eq!(
            msgs.last().unwrap(),
            &mailbox::dispatch(
                MAILBOX,
                dest_domain,
                dest_router,
                warp::Message {
                    recipient: dest_recipient,
                    amount: Uint256::from_u128(50),
                    metadata: HexBinary::default(),
                }
                .into(),
                custom_hook.map(|h| h.to_string()),
                custom_metadata,
                [
                    vec![coin(50, DENOM)],
                    funds.into_iter().filter(|v| v.denom != DENOM).collect()
                ]
                .concat()
            )
            .unwrap()
        );
        msgs.remove(msgs.len() - 1); // remove last (dispatch) msg

        if mode == TokenMode::Bridged {
            assert_eq!(
                msgs.pop().unwrap(),
                conv::to_burn_msg(&mock_env().contract.address, DENOM, "100").into()
            );
        } else {
            assert!(msgs.is_empty());
        }
    }
}
