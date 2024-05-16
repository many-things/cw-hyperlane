#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, wasm_execute, Addr, Deps, DepsMut, Empty, Env, Event, HexBinary, MessageInfo,
    QueryResponse, Response, StdError, StdResult, Storage,
};

use cw_storage_plus::Map;
use hpl_interface::{
    hook::{
        self,
        routing_custom::{
            ClearCustomHookMsg, CustomHookResponse, CustomHooksResponse, CustomRoutingHookQueryMsg,
            ExecuteMsg, InstantiateMsg, QueryMsg, RegisterCustomHookMsg,
        },
        HookQueryMsg, MailboxResponse, PostDispatchMsg, QuoteDispatchMsg, QuoteDispatchResponse,
    },
    range_option, to_binary,
    types::Message,
    Order,
};
use hpl_ownable::get_owner;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("route not found for {0}")]
    RouteNotFound(u32),

    #[error("invalid arguments. reason: {reason:?}")]
    InvalidArguments { reason: String },
}

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CUSTOM_HOOKS_PREFIX: &str = "custom_hooks";
pub const CUSTOM_HOOKS: Map<(u32, Vec<u8>), Addr> = Map::new(CUSTOM_HOOKS_PREFIX);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_hook_routing_custom::{}", name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::Router(msg) => Ok(hpl_router::handle(deps, env, info, msg)?),
        ExecuteMsg::PostDispatch(msg) => post_dispatch(deps, info, msg),

        ExecuteMsg::RegisterCustomHook(msg) => register(deps, info, vec![msg]),
        ExecuteMsg::RegisterCustomHooks(msgs) => register(deps, info, msgs),

        ExecuteMsg::ClearCustomHook(msg) => clear(deps, info, vec![msg]),
        ExecuteMsg::ClearCustomHooks(msgs) => clear(deps, info, msgs),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Router(msg) => Ok(hpl_router::handle_query(deps, env, msg)?),
        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::Mailbox {} => to_binary(get_mailbox(deps)),
            HookQueryMsg::QuoteDispatch(msg) => to_binary(quote_dispatch(deps, msg)),
        },
        QueryMsg::CustomRoutingHook(msg) => match msg {
            CustomRoutingHookQueryMsg::CustomHook {
                dest_domain,
                recipient,
            } => to_binary(get_custom_hook(deps, dest_domain, recipient)),
            CustomRoutingHookQueryMsg::CustomHooks {
                dest_domain,
                offset,
                limit,
                order,
            } => to_binary(list_custom_hooks(deps, dest_domain, offset, limit, order)),
        },
    }
}

fn get_mailbox(_deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: "unrestricted".to_string(),
    })
}

fn get_custom_hook(
    deps: Deps,
    dest_domain: u32,
    recipient: String,
) -> Result<CustomHookResponse, ContractError> {
    let recipient = HexBinary::from_hex(&recipient)?;

    Ok(CustomHookResponse {
        dest_domain,
        recipient: recipient.to_hex(),
        hook: CUSTOM_HOOKS
            .load(deps.storage, (dest_domain, recipient.to_vec()))?
            .into(),
    })
}

fn list_custom_hooks(
    deps: Deps,
    dest_domain: u32,
    offset: Option<String>,
    limit: Option<u32>,
    order: Option<Order>,
) -> Result<CustomHooksResponse, ContractError> {
    let offset = offset
        .as_deref()
        .map(HexBinary::from_hex)
        .transpose()?
        .map(|v| v.to_vec());

    let ((min, max), limit, order) = range_option(offset, limit, order)?;

    let custom_hooks = CUSTOM_HOOKS
        .prefix(dest_domain)
        .range(deps.storage, min, max, order.into())
        .take(limit)
        .map(|item| {
            let (recipient, hook) = item?;

            Ok(CustomHookResponse {
                dest_domain,
                recipient: HexBinary::from(recipient).to_hex(),
                hook: hook.into(),
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(CustomHooksResponse { custom_hooks })
}

fn register(
    deps: DepsMut,
    info: MessageInfo,
    msgs: Vec<RegisterCustomHookMsg>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    for msg in msgs.clone() {
        let recipient = HexBinary::from_hex(&msg.recipient)?;
        ensure_eq!(
            recipient.len(),
            32,
            ContractError::InvalidArguments {
                reason: "recipient must be 32 bytes long".into()
            }
        );

        CUSTOM_HOOKS.save(
            deps.storage,
            (
                msg.dest_domain,
                HexBinary::from_hex(&msg.recipient)?.to_vec(),
            ),
            &deps.api.addr_validate(&msg.hook)?,
        )?;
    }

    Ok(Response::new().add_event(
        new_event("register")
            .add_attribute("sender", info.sender)
            .add_attribute(
                "keys",
                serde_json_wasm::to_string(
                    &msgs
                        .into_iter()
                        .map(|v| format!("{}:{}", v.dest_domain, v.recipient))
                        .collect::<Vec<_>>(),
                )
                .map_err(|e| {
                    ContractError::Std(StdError::generic_err(format!(
                        "failed to marshal keys. reason: {e}",
                    )))
                })?,
            ),
    ))
}

fn clear(
    deps: DepsMut,
    info: MessageInfo,
    msgs: Vec<ClearCustomHookMsg>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    for msg in msgs.clone() {
        CUSTOM_HOOKS.remove(
            deps.storage,
            (
                msg.dest_domain,
                HexBinary::from_hex(&msg.recipient)?.to_vec(),
            ),
        );
    }

    Ok(Response::new().add_event(
        new_event("clear")
            .add_attribute("sender", info.sender)
            .add_attribute(
                "keys",
                serde_json_wasm::to_string(
                    &msgs
                        .into_iter()
                        .map(|v| format!("{}:{}", v.dest_domain, v.recipient))
                        .collect::<Vec<_>>(),
                )
                .map_err(|e| {
                    ContractError::Std(StdError::generic_err(format!(
                        "failed to marshal keys. reason: {e}",
                    )))
                })?,
            ),
    ))
}

fn route(storage: &dyn Storage, message: &HexBinary) -> Result<(Message, Addr), ContractError> {
    let decoded_msg: Message = message.clone().into();
    let dest_domain = decoded_msg.dest_domain;

    let custom_hook =
        CUSTOM_HOOKS.may_load(storage, (dest_domain, decoded_msg.recipient.to_vec()))?;
    if let Some(hook) = custom_hook {
        return Ok((decoded_msg, hook));
    }

    let routed_hook_set = hpl_router::get_route::<Addr>(storage, dest_domain)?;
    let routed_hook = routed_hook_set
        .route
        .ok_or(ContractError::RouteNotFound(dest_domain))?;

    Ok((decoded_msg, routed_hook))
}

fn post_dispatch(
    deps: DepsMut,
    _info: MessageInfo,
    req: PostDispatchMsg,
) -> Result<Response, ContractError> {
    let (decoded_msg, routed_hook) = route(deps.storage, &req.message)?;

    let hook_msg = wasm_execute(&routed_hook, &req.wrap(), vec![])?;

    Ok(Response::new().add_message(hook_msg).add_event(
        new_event("post_dispatch")
            .add_attribute("domain", decoded_msg.dest_domain.to_string())
            .add_attribute("route", routed_hook)
            .add_attribute("message_id", decoded_msg.id().to_hex()),
    ))
}

fn quote_dispatch(
    deps: Deps,
    req: QuoteDispatchMsg,
) -> Result<QuoteDispatchResponse, ContractError> {
    let (_, routed_hook) = route(deps.storage, &req.message)?;

    let resp = hook::quote_dispatch(
        &deps.querier,
        routed_hook.as_str(),
        req.metadata,
        req.message,
    )?;

    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        coin, from_json,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_json_binary, Coins, ContractResult, OwnedDeps, QuerierResult, SystemResult, WasmQuery,
    };
    use hpl_interface::{build_test_querier, hook::ExpectedHookQueryMsg, router::DomainRouteSet};
    use hpl_ownable::get_owner;
    use ibcx_test_utils::{addr, gen_bz, hex};
    use rstest::{fixture, rstest};

    use super::*;

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    type Route = (u32, &'static str);
    type Routes = Vec<Route>;

    type CustomRoute = (u32, &'static str, &'static str);
    type CustomRoutes = Vec<CustomRoute>;

    const OWNER: &str = "owner";
    const DEPLOYER: &str = "deployer";
    const MAILBOX: &str = "mailbox";
    const CUSTOM_USER: &str = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

    const ROUTE1: Route = (26657, "route1");
    const ROUTE2: Route = (26658, "route2");

    const CUSTOM_ROUTE1: CustomRoute = (26657, CUSTOM_USER, "custom_route1");
    const CUSTOM_ROUTE2: CustomRoute = (111333, CUSTOM_USER, "custom_route2");

    build_test_querier!(crate::query);

    fn mock_query_handler(req: &WasmQuery) -> QuerierResult {
        let (req, _addr) = match req {
            WasmQuery::Smart { msg, contract_addr } => (from_json(msg).unwrap(), contract_addr),
            _ => unreachable!("wrong query type"),
        };

        let req = match req {
            ExpectedHookQueryMsg::Hook(HookQueryMsg::QuoteDispatch(msg)) => msg,
            _ => unreachable!("wrong query type"),
        };

        let mut fees = Coins::default();

        if !req.metadata.is_empty() {
            let parsed_fee = u32::from_be_bytes(req.metadata.as_slice().try_into().unwrap());

            fees = Coins::from(coin(parsed_fee as u128, "utest"));
        }

        let res = QuoteDispatchResponse {
            fees: fees.into_vec(),
        };
        let res = to_json_binary(&res).unwrap();

        SystemResult::Ok(ContractResult::Ok(res))
    }

    #[fixture]
    fn deps(
        #[default(addr(DEPLOYER))] sender: Addr,
        #[default(addr(OWNER))] owner: Addr,
    ) -> TestDeps {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                owner: owner.to_string(),
            },
        )
        .unwrap();

        deps
    }

    #[fixture]
    fn deps_routes(
        mut deps: TestDeps,
        #[default(vec![ROUTE1, ROUTE2])] routes: Routes,
        #[default(addr("owner"))] sender: Addr,
    ) -> (TestDeps, Routes) {
        hpl_router::set_routes(
            deps.as_mut().storage,
            &sender,
            routes
                .iter()
                .map(|(dest_domain, hook)| DomainRouteSet {
                    domain: *dest_domain,
                    route: Some(addr(hook)),
                })
                .collect(),
        )
        .unwrap();

        (deps, routes)
    }

    #[fixture]
    fn deps_custom_routes(
        deps_routes: (TestDeps, Routes),
        #[default(vec![CUSTOM_ROUTE1, CUSTOM_ROUTE2])] custom_routes: CustomRoutes,
    ) -> (TestDeps, Routes, CustomRoutes) {
        let (mut deps, routes) = deps_routes;

        register(
            deps.as_mut(),
            mock_info(OWNER, &[]),
            custom_routes
                .iter()
                .map(|v| RegisterCustomHookMsg {
                    dest_domain: v.0,
                    recipient: v.1.to_string(),
                    hook: v.2.to_string(),
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();

        (deps, routes, custom_routes)
    }

    #[rstest]
    fn test_init(deps: TestDeps) {
        assert_eq!(OWNER, get_owner(deps.as_ref().storage).unwrap());
    }

    #[rstest]
    fn test_get_mailbox(deps: TestDeps) {
        let res: MailboxResponse =
            test_query(deps.as_ref(), QueryMsg::Hook(HookQueryMsg::Mailbox {}));
        assert_eq!("unrestricted", res.mailbox);
    }

    #[rstest]
    fn test_get_custom_hook(deps_custom_routes: (TestDeps, Routes, CustomRoutes)) {
        let (deps, _, custom_routes) = deps_custom_routes;

        for (dest_domain, recipient, hook) in custom_routes {
            let res: CustomHookResponse = test_query(
                deps.as_ref(),
                QueryMsg::CustomRoutingHook(CustomRoutingHookQueryMsg::CustomHook {
                    dest_domain,
                    recipient: recipient.to_string(),
                }),
            );

            assert_eq!(hook, res.hook);
        }
    }

    #[rstest]
    fn test_list_custom_hooks(deps_custom_routes: (TestDeps, Routes, CustomRoutes)) {
        let (deps, _, custom_routes) = deps_custom_routes;

        for route in custom_routes {
            let res: CustomHooksResponse = test_query(
                deps.as_ref(),
                QueryMsg::CustomRoutingHook(CustomRoutingHookQueryMsg::CustomHooks {
                    dest_domain: route.0,
                    offset: None,
                    limit: None,
                    order: None,
                }),
            );

            assert_eq!(
                vec![route],
                res.custom_hooks
                    .iter()
                    .map(|v| (v.dest_domain, v.recipient.as_str(), v.hook.as_str()))
                    .collect::<Vec<_>>()
            );
        }
    }

    #[rstest]
    #[case(OWNER)]
    #[should_panic(expected = "unauthorized")]
    #[case(MAILBOX)]
    fn test_register(mut deps: TestDeps, #[case] sender: &str) {
        let custom_routes = vec![CUSTOM_ROUTE1, CUSTOM_ROUTE2];

        register(
            deps.as_mut(),
            mock_info(sender, &[]),
            custom_routes
                .iter()
                .map(|v| RegisterCustomHookMsg {
                    dest_domain: v.0,
                    recipient: v.1.to_string(),
                    hook: v.2.to_string(),
                })
                .collect::<Vec<_>>(),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        for (dest_domain, recipient, hook) in custom_routes {
            let hook_loaded = CUSTOM_HOOKS
                .load(
                    deps.as_ref().storage,
                    (
                        dest_domain,
                        HexBinary::from_hex(recipient).unwrap().to_vec(),
                    ),
                )
                .unwrap();

            assert_eq!(hook, hook_loaded.as_str());
        }
    }

    #[rstest]
    #[case(OWNER)]
    #[should_panic(expected = "unauthorized")]
    #[case(MAILBOX)]
    fn test_clear(deps_custom_routes: (TestDeps, Routes, CustomRoutes), #[case] sender: &str) {
        let (mut deps, _, custom_routes) = deps_custom_routes;

        clear(
            deps.as_mut(),
            mock_info(sender, &[]),
            custom_routes
                .iter()
                .map(|v| ClearCustomHookMsg {
                    dest_domain: v.0,
                    recipient: v.1.to_string(),
                })
                .collect::<Vec<_>>(),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        for (dest_domain, recipient, _) in custom_routes {
            let hook_exists = CUSTOM_HOOKS.has(
                deps.as_ref().storage,
                (
                    dest_domain,
                    HexBinary::from_hex(recipient).unwrap().to_vec(),
                ),
            );

            assert!(!hook_exists);
        }
    }

    #[rstest]
    #[case(OWNER, 26657, gen_bz(20), ROUTE1.1)]
    #[case(OWNER, 26657, hex(CUSTOM_USER), CUSTOM_ROUTE1.2)]
    #[case(OWNER, 111333, hex(CUSTOM_USER), CUSTOM_ROUTE2.2)]
    #[should_panic(expected = "route not found for 111333")]
    #[case(OWNER, 111333, gen_bz(20), CUSTOM_ROUTE2.2)]
    #[should_panic(expected = "route not found for 12345")]
    #[case(OWNER, 12345, gen_bz(20), ROUTE1.1)]
    fn test_post_dispatch(
        deps_custom_routes: (TestDeps, Routes, CustomRoutes),
        #[case] sender: &str,
        #[case] test_domain: u32,
        #[case] recipient: HexBinary,
        #[case] expected_hook: &str,
    ) {
        let (mut deps, _, _) = deps_custom_routes;

        let mut rand_msg: Message = gen_bz(200).into();
        rand_msg.dest_domain = test_domain;
        rand_msg.recipient = recipient;

        let res = post_dispatch(
            deps.as_mut(),
            mock_info(sender, &[]),
            PostDispatchMsg {
                metadata: HexBinary::default(),
                message: rand_msg.into(),
            },
        )
        .map_err(|e| e.to_string())
        .unwrap();

        let event = res
            .events
            .iter()
            .find(|v| v.ty == new_event("post_dispatch").ty)
            .unwrap();

        assert_eq!(
            test_domain,
            event.attributes[0].value.parse::<u32>().unwrap()
        );
        assert_eq!(expected_hook, event.attributes[1].value);
    }

    #[rstest]
    #[case(26657, Some(26657))]
    #[should_panic(expected = "route not found for 12345")]
    #[case(12345, None)]
    fn test_quote_dispatch(
        deps_custom_routes: (TestDeps, Routes, CustomRoutes),
        #[case] test_domain: u32,
        #[case] expected_fee: Option<u32>,
    ) {
        let (mut deps, _, _) = deps_custom_routes;

        deps.querier.update_wasm(mock_query_handler);

        let mut rand_msg: Message = gen_bz(100).into();
        rand_msg.dest_domain = test_domain;

        let res: QuoteDispatchResponse = test_query(
            deps.as_ref(),
            QueryMsg::Hook(HookQueryMsg::QuoteDispatch(QuoteDispatchMsg {
                metadata: test_domain.to_be_bytes().to_vec().into(),
                message: rand_msg.into(),
            })),
        );
        assert_eq!(
            res.fees.first().map(|v| v.amount.u128() as u32),
            expected_fee
        );
    }
}
