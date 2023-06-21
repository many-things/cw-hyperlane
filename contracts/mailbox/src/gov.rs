use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{
    event::{emit_default_ism_changed, emit_paused, emit_unpaused},
    state::{assert_owner, Config, CONFIG, PAUSE},
    ContractError,
};

pub fn pause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    assert_owner(&config.owner, &info.sender)?;

    PAUSE.save(deps.storage, &true)?;

    Ok(Response::new().add_event(emit_paused(info.sender)))
}

pub fn unpause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_owner(&config.owner, &info.sender)?;

    PAUSE.save(deps.storage, &false)?;

    Ok(Response::new().add_event(emit_unpaused(info.sender)))
}

pub fn set_default_ism(
    deps: DepsMut,
    info: MessageInfo,
    new_default_ism: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_owner(&config.owner, &info.sender)?;

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
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr, Storage,
    };

    use super::*;

    const OWNER: &str = "owner";
    const FACTORY: &str = "factory";
    const DEFAULT_ISM: &str = "default_ism";
    const NOT_OWNER: &str = "not_owner";

    fn mock_owner(storage: &mut dyn Storage, owner: Addr, factory: Addr, default_ism: Addr) {
        let config = Config {
            owner,
            factory,
            default_ism,
        };

        CONFIG.save(storage, &config).unwrap();
    }

    #[test]
    fn test_pause() {
        let mut deps = mock_dependencies();

        let owner = Addr::unchecked(OWNER);
        let factory = Addr::unchecked(FACTORY);
        let default_ism = Addr::unchecked(DEFAULT_ISM);

        mock_owner(
            deps.as_mut().storage,
            owner.clone(),
            factory.clone(),
            default_ism.clone(),
        );

        // Sender is not authorized
        let sender = NOT_OWNER;
        let info = mock_info(sender, &[]);

        let owner_assert = pause(deps.as_mut(), info).unwrap_err();

        assert!(matches!(owner_assert, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_unpause() {
        let mut deps = mock_dependencies();

        let owner = Addr::unchecked(OWNER);
        let factory = Addr::unchecked(FACTORY);
        let default_ism = Addr::unchecked(DEFAULT_ISM);

        mock_owner(
            deps.as_mut().storage,
            owner.clone(),
            factory.clone(),
            default_ism.clone(),
        );

        // Sender is not authorized
        let sender = NOT_OWNER;
        let info = mock_info(sender, &[]);

        let owner_assert = unpause(deps.as_mut(), info).unwrap_err();

        assert!(matches!(owner_assert, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_set_default_ism() {
        let mut deps = mock_dependencies();

        let owner = Addr::unchecked(OWNER);
        let factory = Addr::unchecked(FACTORY);
        let default_ism = Addr::unchecked(DEFAULT_ISM);

        mock_owner(
            deps.as_mut().storage,
            owner.clone(),
            factory.clone(),
            default_ism.clone(),
        );

        // Sender is not authorized
        let sender = NOT_OWNER;
        let info = mock_info(sender, &[]);
        let new_default_ism = "new_default_ism".to_string();

        let owner_assert = set_default_ism(deps.as_mut(), info, new_default_ism).unwrap_err();

        assert!(matches!(owner_assert, ContractError::Unauthorized {}));
    }
}
