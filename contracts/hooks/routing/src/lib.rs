#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, wasm_execute, Addr, Deps, DepsMut, Env, Event, HexBinary, MessageInfo,
    QueryResponse, Response, StdError, Storage,
};

use cw_storage_plus::Item;
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

    #[error("unauthorized")]
    Unauthorized {},

    #[error("route not found for {0}")]
    RouteNotFound(u32),
}

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);

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
    let mailbox = deps.api.addr_validate(&msg.mailbox)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    MAILBOX.save(deps.storage, &mailbox)?;

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

fn get_mailbox(deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: MAILBOX.load(deps.storage)?.into(),
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
    info: MessageInfo,
    req: PostDispatchMsg,
) -> Result<Response, ContractError> {
    ensure_eq!(
        MAILBOX.load(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    let (decoded_msg, routed_hook) = route(deps.storage, &req.message)?;

    let hook_msg = wasm_execute(&routed_hook, &req.wrap(), vec![])?;

    Ok(Response::new().add_message(hook_msg).add_event(
        new_event("post_dispatch")
            .add_attribute("domain", decoded_msg.dest_domain.to_string())
            .add_attribute("route", routed_hook)
            .add_attribute("message_id", decoded_msg.id().to_hex()),
    ))
}

#[cfg(test)]
mod test {
    use cosmwasm_schema::serde::{de::DeserializeOwned, Serialize};
    use cosmwasm_std::{
        coin, from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Coin, ContractResult, OwnedDeps, QuerierResult, SystemResult, WasmQuery,
    };
    use hpl_interface::{hook::ExpectedHookQueryMsg, router::DomainRouteSet};
    use hpl_ownable::get_owner;
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::{fixture, rstest};

    use super::*;

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    fn query<S: Serialize, T: DeserializeOwned>(deps: Deps, msg: S) -> T {
        let req: QueryMsg = from_binary(&cosmwasm_std::to_binary(&msg).unwrap()).unwrap();
        let res = crate::query(deps, mock_env(), req)
            .map_err(|e| e.to_string())
            .unwrap();
        from_binary(&res).unwrap()
    }

    #[fixture]
    fn deps(
        #[default(addr("deployer"))] sender: Addr,
        #[default(addr("owner"))] owner: Addr,
        #[default(addr("mailbox"))] mailbox: Addr,
    ) -> TestDeps {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                owner: owner.to_string(),
                mailbox: mailbox.to_string(),
            },
        )
        .unwrap();

        deps
    }

    fn mock_query_handler(req: &WasmQuery) -> QuerierResult {
        let req: ExpectedHookQueryMsg = match req {
            WasmQuery::Smart { msg, .. } => from_binary(msg).unwrap(),
            _ => unreachable!("wrong query type"),
        };

        let req = match req {
            ExpectedHookQueryMsg::Hook(HookQueryMsg::QuoteDispatch(msg)) => msg,
            _ => unreachable!("wrong query type"),
        };

        let mut gas_amount = None;
        if !req.metadata.is_empty() {
            gas_amount = Some(serde_json_wasm::from_slice(&req.metadata).unwrap());
        }

        let res = QuoteDispatchResponse { gas_amount };
        let res = cosmwasm_std::to_binary(&res).unwrap();
        SystemResult::Ok(ContractResult::Ok(res))
    }

    #[rstest]
    fn test_init(deps: TestDeps) {
        assert_eq!("owner", get_owner(deps.as_ref().storage).unwrap().as_str());
        assert_eq!(
            "mailbox",
            MAILBOX.load(deps.as_ref().storage).unwrap().as_str()
        );
    }

    #[rstest]
    #[case("mailbox", 26657, vec![(26657, "route1"), (26658, "route2")])]
    #[should_panic(expected = "route not found for 26657")]
    #[case("mailbox", 26657, vec![])]
    #[should_panic(expected = "unauthorized")]
    #[case("owner", 26657, vec![(26657, "route")])]
    fn test_post_dispatch(
        mut deps: TestDeps,
        #[case] sender: &str,
        #[case] test_domain: u32,
        #[case] routes: Vec<(u32, &str)>,
    ) {
        hpl_router::set_routes(
            deps.as_mut().storage,
            &addr(sender),
            routes
                .iter()
                .map(|(dest_domain, hook)| DomainRouteSet {
                    domain: *dest_domain,
                    route: Some(addr(hook)),
                })
                .collect(),
        )
        .unwrap();

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
    fn test_get_mailbox(deps: TestDeps) {
        let res: MailboxResponse = query(deps.as_ref(), QueryMsg::Hook(HookQueryMsg::Mailbox {}));
        assert_eq!("mailbox", res.mailbox);
    }

    #[rstest]
    #[case(26657, Some(coin(123u128, "utest")), vec![(26657, "route1"), (26658, "route2")])]
    #[should_panic(expected = "route not found for 26657")]
    #[case(26657, None, vec![])]
    #[should_panic(expected = "route not found for 26657")]
    #[case(26657, None, vec![(26658, "route2")])]
    fn test_quote_dispatch(
        mut deps: TestDeps,
        #[case] test_domain: u32,
        #[case] gas_amount: Option<Coin>,
        #[case] routes: Vec<(u32, &str)>,
    ) {
        deps.querier.update_wasm(mock_query_handler);

        hpl_router::set_routes(
            deps.as_mut().storage,
            &addr("owner"),
            routes
                .iter()
                .map(|(dest_domain, hook)| DomainRouteSet {
                    domain: *dest_domain,
                    route: Some(addr(hook)),
                })
                .collect(),
        )
        .unwrap();

        let mut rand_msg: Message = gen_bz(100).into();
        rand_msg.dest_domain = test_domain;

        let res: QuoteDispatchResponse = query(
            deps.as_ref(),
            QueryMsg::Hook(HookQueryMsg::QuoteDispatch(QuoteDispatchMsg {
                metadata: gas_amount
                    .clone()
                    .map(|v| serde_json_wasm::to_vec(&v).unwrap().into())
                    .unwrap_or_default(),
                message: rand_msg.into(),
            })),
        );
        assert_eq!(res.gas_amount, gas_amount);
    }
}
