use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{
    event::{emit_default_ism_changed, emit_paused, emit_unpaused},
    state::{Config, CONFIG, PAUSE},
    ContractError,
};

pub fn pause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_eq!(config.owner, info.sender, "not an owner");

    PAUSE.save(deps.storage, &true)?;

    Ok(Response::new().add_event(emit_paused(info.sender)))
}

pub fn unpause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_eq!(config.owner, info.sender, "not an owner");

    PAUSE.save(deps.storage, &false)?;

    Ok(Response::new().add_event(emit_unpaused(info.sender)))
}

pub fn set_default_ism(
    deps: DepsMut,
    info: MessageInfo,
    new_default_ism: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_eq!(config.owner, info.sender, "not an owner");

    // FIXME: clone
    let new_default_ism = deps.api.addr_validate(&new_default_ism)?;
    CONFIG.save(
        deps.storage,
        &Config {
            default_ism: new_default_ism.clone(),
            ..config
        },
    )?;

    Ok(Response::new().add_event(emit_default_ism_changed(info.sender, new_default_ism)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pause() {}

    fn test_unpause() {}

    fn test_set_default_ism() {}
}
