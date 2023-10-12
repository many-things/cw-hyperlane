#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, wasm_execute, Addr, Deps, DepsMut, Env, Event, HexBinary, MessageInfo,
    QueryResponse, Response, StdError, Storage,
};

use cw_storage_plus::{Item, Map};
use hpl_interface::{
    hook::{
        routing_custom::{
            ClearCustomHookMsg, ExecuteMsg, InstantiateMsg, QueryMsg, RegisterCustomHookMsg,
        },
        HookQueryMsg, MailboxResponse, PostDispatchMsg, QuoteDispatchMsg, QuoteDispatchResponse,
    },
    to_binary,
    types::{bech32_decode, Message},
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

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);

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
    }
}

fn get_mailbox(deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: MAILBOX.load(deps.storage)?.into(),
    })
}

fn register(
    deps: DepsMut,
    info: MessageInfo,
    msgs: Vec<RegisterCustomHookMsg>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        hpl_ownable::get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    for msg in msgs.clone() {
        CUSTOM_HOOKS.save(
            deps.storage,
            (msg.dest_domain, bech32_decode(&msg.recipient)?),
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
                .unwrap(),
            ),
    ))
}

fn clear(
    deps: DepsMut,
    info: MessageInfo,
    msgs: Vec<ClearCustomHookMsg>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        hpl_ownable::get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    for msg in msgs.clone() {
        CUSTOM_HOOKS.remove(
            deps.storage,
            (msg.dest_domain, bech32_decode(&msg.recipient)?),
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
                .unwrap(),
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
        .ok_or(ContractError::HookNotRegistered(dest_domain))?;

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

fn quote_dispatch(
    deps: Deps,
    req: QuoteDispatchMsg,
) -> Result<QuoteDispatchResponse, ContractError> {
    let (_, routed_hook) = route(deps.storage, &req.message)?;

    let resp: QuoteDispatchResponse = deps.querier.query_wasm_smart(routed_hook, &req.wrap())?;

    Ok(resp)
}
