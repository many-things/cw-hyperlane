use cosmwasm_std::{
    ensure_eq, to_json_binary, Addr, CustomQuery, Deps, DepsMut, Env, Event, MessageInfo,
    QueryResponse, Response, StdError, StdResult, Storage,
};
use cw_storage_plus::Item;
use hpl_interface::connection::{
    ConnectionMsg, ConnectionQueryMsg, HookResponse, IsmResponse, MailboxResponse,
};

const MAILBOX_KEY: &str = "conn::mailbox";
const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);

const ISM_KEY: &str = "conn::ism";
const ISM: Item<Addr> = Item::new(ISM_KEY);

const HOOK_KEY: &str = "conn::hook";
const HOOK: Item<Addr> = Item::new(HOOK_KEY);

fn event_to_resp(event: Event) -> Response {
    Response::new().add_event(event)
}

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_connection::{}", name))
}

pub fn handle<C: CustomQuery>(
    deps: DepsMut<'_, C>,
    _env: Env,
    info: MessageInfo,
    msg: ConnectionMsg,
) -> StdResult<Response> {
    use ConnectionMsg::*;

    ensure_eq!(
        hpl_ownable::get_owner(deps.storage)?,
        info.sender,
        StdError::generic_err("unauthorized")
    );

    match msg {
        SetMailbox { mailbox } => {
            let mailbox_addr = deps.api.addr_validate(&mailbox)?;

            MAILBOX.save(deps.storage, &mailbox_addr)?;

            Ok(event_to_resp(
                new_event("set_mailbox").add_attribute("mailbox", mailbox),
            ))
        }
        SetIsm { ism } => {
            let ism_addr = deps.api.addr_validate(&ism)?;

            ISM.save(deps.storage, &ism_addr)?;

            Ok(event_to_resp(
                new_event("set_ism").add_attribute("ism", ism),
            ))
        }
        SetHook { hook } => {
            let hook_addr = deps.api.addr_validate(&hook)?;

            HOOK.save(deps.storage, &hook_addr)?;

            Ok(event_to_resp(
                new_event("set_hook").add_attribute("hook", hook),
            ))
        }
    }
}

pub fn handle_query<C: CustomQuery>(
    deps: Deps<'_, C>,
    _env: Env,
    msg: ConnectionQueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        ConnectionQueryMsg::GetMailbox {} => Ok(to_json_binary(&MailboxResponse {
            mailbox: get_mailbox(deps.storage)?.map(|v| v.into()),
        })?),
        ConnectionQueryMsg::GetHook {} => Ok(to_json_binary(&HookResponse {
            hook: get_hook(deps.storage)?.map(|v| v.into()),
        })?),
        ConnectionQueryMsg::GetIsm {} => Ok(to_json_binary(&IsmResponse {
            ism: get_ism(deps.storage)?.map(|v| v.into()),
        })?),
    }
}

pub fn get_mailbox(storage: &dyn Storage) -> StdResult<Option<Addr>> {
    MAILBOX.may_load(storage)
}

pub fn get_ism(storage: &dyn Storage) -> StdResult<Option<Addr>> {
    ISM.may_load(storage)
}

pub fn get_hook(storage: &dyn Storage) -> StdResult<Option<Addr>> {
    HOOK.may_load(storage)
}
