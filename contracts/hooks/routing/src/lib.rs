#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    wasm_execute, Addr, Deps, DepsMut, Env, Event, HexBinary, MessageInfo, QueryResponse, Response,
    StdError, Storage,
};

use hpl_interface::{
    hook::{
        routing::{ExecuteMsg, InstantiateMsg, QueryMsg},
        HookQueryMsg, PostDispatchMsg, QuoteDispatchMsg, QuoteDispatchResponse,
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

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Hook not registered for dest: {0}")]
    HookNotRegistered(u32),
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
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    Ok(Response::new())
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
        ExecuteMsg::PostDispatch(PostDispatchMsg { metadata, message }) => {
            post_dispatch(deps, metadata, message)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Router(msg) => Ok(hpl_router::handle_query(deps, env, msg)?),
        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::QuoteDispatch(QuoteDispatchMsg { metadata, message }) => {
                to_binary(quote_dispatch(deps, metadata, message))
            }
        },
    }
}

fn route(storage: &dyn Storage, message: &HexBinary) -> Result<(Message, Addr), ContractError> {
    let decoded_msg: Message = message.clone().into();
    let dest_domain = decoded_msg.dest_domain;

    let routed_hook_set = hpl_router::get_route::<Addr>(storage, dest_domain)?;
    let routed_hook = routed_hook_set
        .route
        .ok_or(ContractError::HookNotRegistered(dest_domain))?;

    Ok((decoded_msg, routed_hook))
}

pub fn post_dispatch(
    deps: DepsMut,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<Response, ContractError> {
    let (decoded_msg, routed_hook) = route(deps.storage, &message)?;

    let hook_msg = wasm_execute(
        &routed_hook,
        &PostDispatchMsg { metadata, message }.wrap(),
        vec![],
    )?;

    Ok(Response::new().add_message(hook_msg).add_event(
        new_event("routed")
            .add_attribute("domain", decoded_msg.dest_domain.to_string())
            .add_attribute("route", routed_hook)
            .add_attribute("message_id", decoded_msg.id().to_hex()),
    ))
}

pub fn quote_dispatch(
    deps: Deps,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<QuoteDispatchResponse, ContractError> {
    let (_, routed_hook) = route(deps.storage, &message)?;

    let hook_resp: QuoteDispatchResponse = deps
        .querier
        .query_wasm_smart(&routed_hook, &QuoteDispatchMsg { metadata, message }.wrap())?;

    Ok(hook_resp)
}
