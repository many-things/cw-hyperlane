use std::{ops::Deref, env};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    wasm_execute, Event, MessageInfo, QueryResponse, Response, DepsMut, Deps, Addr,
    StdError, Env, Binary, StdResult,
};
use cw_storage_plus::Item;
use hpl_interface::{
    hook::{
        wormhole::{ExecuteMsg, InstantiateMsg, QueryMsg, WormholeInfoResponse, WormholeQueryMsg},
        HookQueryMsg, MailboxResponse, QuoteDispatchResponse, PostDispatchMsg,
    },
    to_binary,
    types::Message,
};
mod wormhole;
use wormhole::WormholeExecuteMsg;



// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const WORMHOLE_CORE_KEY: &str = "wormhole_core";
const WORMHOLE_CORE: Item<Addr> = Item::new(WORMHOLE_CORE_KEY);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_hook_pausable::{}", name))
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("hook paused")]
    Paused {},

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

    let wormhole_core = deps.api.addr_validate(&msg.wormhole_core)?;
    WORMHOLE_CORE.save(deps.storage, &wormhole_core)?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("wormhole_core", wormhole_core)

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
        // TODO: maybe add SetWomrholeCore Msg
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::PostDispatch(msg) => post_dispatch(deps, info, msg)
    }
}

fn post_dispatch(
    deps: DepsMut,
    _info: MessageInfo,
    req: PostDispatchMsg,
) -> Result<Response, ContractError> {
    let wormhole_core = WORMHOLE_CORE.load(deps.storage)?;
    let decoded_msg: Message = req.message.clone().into();
    let binary_message = Binary::from(req.message);
    let wormhole_message: WormholeExecuteMsg = WormholeExecuteMsg::PostMessage {nonce: decoded_msg.nonce, message: binary_message};

    let wormhole_msg = wasm_execute(&wormhole_core, &wormhole_message, vec![])?;

    Ok(Response::new().add_message(wormhole_msg).add_event(
        new_event("post_dispatch")
            .add_attribute("message_id", decoded_msg.id().to_hex()),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::Wormhole(msg) => Ok(handle_query(deps, env, msg)?),
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::Mailbox {} => to_binary(get_mailbox(deps)),
            HookQueryMsg::QuoteDispatch(_) => to_binary(quote_dispatch()),
        },
    }
}

pub fn handle_query(
    deps: Deps,
    _env: Env,
    msg: WormholeQueryMsg,
) -> StdResult<QueryResponse> {
    cosmwasm_std::to_binary(&WormholeInfoResponse {
            wormhole_core: WORMHOLE_CORE.load(deps.storage)?.into_string(),
        })
}

fn get_mailbox(_deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: "unrestricted".to_string(),
    })
}

fn quote_dispatch() -> Result<QuoteDispatchResponse, ContractError> {
    Ok(QuoteDispatchResponse { gas_amount: None })
}