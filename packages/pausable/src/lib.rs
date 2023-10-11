use cosmwasm_std::{
    ensure_eq, to_binary, Addr, CustomQuery, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse,
    Response, StdError, StdResult, Storage,
};
use cw_storage_plus::Item;
use hpl_interface::pausable::{PausableMsg, PausableQueryMsg, PauseInfoResponse};

pub const PAUSE_KEY: &str = "pause";
pub const PAUSE: Item<bool> = Item::new(PAUSE_KEY);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_pausable::{}", name))
}

pub fn handle<C: CustomQuery>(
    deps: DepsMut<'_, C>,
    _env: Env,
    info: MessageInfo,
    msg: PausableMsg,
) -> StdResult<Response> {
    ensure_eq!(
        hpl_ownable::get_owner(deps.storage)?,
        info.sender,
        StdError::generic_err("unauthorized")
    );

    match msg {
        PausableMsg::Pause {} => Ok(Response::new().add_event(pause(deps.storage, &info.sender)?)),
        PausableMsg::Release {} => {
            Ok(Response::new().add_event(release(deps.storage, &info.sender)?))
        }
    }
}

fn pause(storage: &mut dyn Storage, sender: &Addr) -> StdResult<Event> {
    PAUSE.save(storage, &true)?;
    Ok(new_event("pause").add_attribute("sender", sender))
}

fn release(storage: &mut dyn Storage, sender: &Addr) -> StdResult<Event> {
    PAUSE.save(storage, &false)?;
    Ok(new_event("release").add_attribute("sender", sender))
}

pub fn handle_query<C: CustomQuery>(
    deps: Deps<'_, C>,
    _env: Env,
    msg: PausableQueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        PausableQueryMsg::PauseInfo {} => to_binary(&PauseInfoResponse {
            paused: get_pause_info(deps.storage)?,
        }),
    }
}

pub fn get_pause_info(storage: &dyn Storage) -> StdResult<bool> {
    PAUSE.load(storage)
}
