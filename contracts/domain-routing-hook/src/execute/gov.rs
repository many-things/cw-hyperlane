use cosmwasm_std::{ensure, ensure_eq, DepsMut, Env, Event, MessageInfo, Response};

use crate::{
    state::{MAILBOX, PAUSE},
    ContractError,
};

pub fn pause(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::OWNER.load(deps.storage)?,
        ContractError::Unauthorized {}
    );
    ensure!(!PAUSE.load(deps.storage)?, ContractError::Paused {});

    PAUSE.save(deps.storage, &true)?;
    Ok(Response::new().add_event(Event::new("pause")))
}

pub fn unpause(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::OWNER.load(deps.storage)?,
        ContractError::Unauthorized {}
    );
    ensure!(PAUSE.load(deps.storage)?, ContractError::NotPaused {});

    PAUSE.save(deps.storage, &false)?;
    Ok(Response::new().add_event(Event::new("unpause")))
}

pub fn update_mailbox(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mailbox: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::OWNER.load(deps.storage)?,
        ContractError::Unauthorized {}
    );
    ensure!(!PAUSE.load(deps.storage)?, ContractError::Paused {});

    MAILBOX.save(deps.storage, &deps.api.addr_validate(&mailbox)?)?;
    Ok(Response::new().add_event(Event::new("update-mailbox").add_attribute("mailbox", mailbox)))
}


