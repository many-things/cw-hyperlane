use cosmwasm_std::{
    Addr, CustomQuery, DepsMut, Env, Event, MessageInfo, Response, StdError, StdResult, Storage,
};
use hpl_interface::ownable::OwnableMsg;

use crate::state::{OWNER, PENDING_OWNER};

pub fn handle<C: CustomQuery>(
    deps: DepsMut<'_, C>,
    _env: Env,
    info: MessageInfo,
    msg: OwnableMsg,
) -> StdResult<Response> {
    use OwnableMsg::*;

    match msg {
        InitOwnershipTransfer { next_owner } => {
            Ok(Response::new().add_event(init_ownership_transfer(
                deps.storage,
                &info.sender,
                &deps.api.addr_validate(&next_owner)?,
            )?))
        }
        RevokeOwnershipTransfer {} => {
            Ok(Response::new().add_event(revoke_ownership_transfer(deps.storage, &info.sender)?))
        }
        ClaimOwnership {} => {
            Ok(Response::new().add_event(claim_ownership(deps.storage, &info.sender)?))
        }
    }
}

pub fn init_ownership_transfer(
    storage: &mut dyn Storage,
    sender: &Addr,
    next_owner: &Addr,
) -> StdResult<Event> {
    if sender != OWNER.load(storage)? {
        return Err(StdError::generic_err("unauthorized"));
    }
    if PENDING_OWNER.exists(storage) {
        return Err(StdError::generic_err("ownership is transferring"));
    }

    PENDING_OWNER.save(storage, next_owner)?;

    Ok(Event::new("init-ownership-transfer")
        .add_attribute("owner", sender)
        .add_attribute("next_owner", next_owner))
}

pub fn revoke_ownership_transfer(storage: &mut dyn Storage, sender: &Addr) -> StdResult<Event> {
    if sender != OWNER.load(storage)? {
        return Err(StdError::generic_err("unauthorized"));
    }
    if !PENDING_OWNER.exists(storage) {
        return Err(StdError::generic_err("ownership is not transferring"));
    }

    PENDING_OWNER.remove(storage);

    Ok(Event::new("revoke-ownership-transfer").add_attribute("owner", sender))
}

pub fn claim_ownership(storage: &mut dyn Storage, sender: &Addr) -> StdResult<Event> {
    if !PENDING_OWNER.exists(storage) {
        return Err(StdError::generic_err("ownership is not transferring"));
    }
    if sender != PENDING_OWNER.load(storage)? {
        return Err(StdError::generic_err("unauthorized"));
    }

    OWNER.save(storage, sender)?;
    PENDING_OWNER.remove(storage);

    Ok(Event::new("claim-ownership").add_attribute("owner", sender))
}
