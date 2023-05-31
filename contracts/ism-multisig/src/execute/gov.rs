use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{
    state::{Config, CONFIG, PENDING_OWNER},
    ContractError,
};

pub fn init_transfer_ownership(
    deps: DepsMut,
    info: MessageInfo,
    next_owner: String,
) -> Result<Response, ContractError> {
    assert_eq!(info.sender, CONFIG.load(deps.storage)?.owner);
    assert!(PENDING_OWNER.may_load(deps.storage)?.is_none());

    PENDING_OWNER.save(deps.storage, &deps.api.addr_validate(&next_owner)?)?;

    // TODO: define event
    Ok(Response::new())
}

pub fn finish_transfer_ownership(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let pending_owner = PENDING_OWNER.may_load(deps.storage)?;

    assert!(pending_owner.is_some());
    assert_eq!(info.sender, PENDING_OWNER.load(deps.storage)?);

    let config = CONFIG.load(deps.storage)?;

    CONFIG.save(
        deps.storage,
        &Config {
            owner: pending_owner.unwrap(),
            ..config
        },
    )?;

    // FIXME: define event
    Ok(Response::new())
}

pub fn revoke_transfer_ownership(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // FIXME: define event
    Ok(Response::new())
}
