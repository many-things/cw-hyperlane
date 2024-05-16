#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    wasm_execute, Addr, Deps, DepsMut, Empty, Env, Event, HexBinary, MessageInfo, QueryResponse,
    Response, StdError, Storage,
};

use hpl_interface::{
    hook::{
        self,
        routing::{ExecuteMsg, InstantiateMsg, QueryMsg},
        HookQueryMsg, MailboxResponse, PostDispatchMsg, QuoteDispatchMsg, QuoteDispatchResponse,
    },
    to_binary,
    types::Message,
};

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
}

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_hook_routing::{}", name))
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
    }
}

fn get_mailbox(_deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: "unrestricted".to_string(),
    })
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

fn route(storage: &dyn Storage, message: &HexBinary) -> Result<(Message, Addr), ContractError> {
    let decoded_msg: Message = message.clone().into();
    let dest_domain = decoded_msg.dest_domain;

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
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::{fixture, rstest};

    use super::*;

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;
    type Route = (u32, &'static str);
    type Routes = Vec<Route>;

    const ROUTE1: Route = (26657, "route1");
    const ROUTE2: Route = (26658, "route2");

    const OWNER: &str = "owner";
    const DEPLOYER: &str = "deployer";

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
    #[case(OWNER, 26657)]
    #[should_panic(expected = "route not found for 12345")]
    #[case(OWNER, 12345)]
    fn test_post_dispatch(
        deps_routes: (TestDeps, Routes),
        #[case] sender: &str,
        #[case] test_domain: u32,
    ) {
        let (mut deps, routes) = deps_routes;

        let mut rand_msg: Message = gen_bz(100).into();
        rand_msg.dest_domain = test_domain;

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
        assert_eq!(
            routes.iter().find(|v| v.0 == test_domain).unwrap().1,
            event.attributes[1].value
        );
    }

    #[rstest]
    #[case(26657, Some(26657))]
    #[should_panic(expected = "route not found for 12345")]
    #[case(12345, None)]
    fn test_quote_dispatch(
        deps_routes: (TestDeps, Routes),
        #[case] test_domain: u32,
        #[case] expected_fee: Option<u32>,
    ) {
        let (mut deps, _) = deps_routes;

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
