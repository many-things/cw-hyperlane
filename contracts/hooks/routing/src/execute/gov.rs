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

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Storage,
    };

    use super::*;
    const ADDR1_VALUE: &str = "addr1";
    const ADDR2_VALUE: &str = "addr2";

    fn mock_owner(storage: &mut dyn Storage, owner: &str) {
        hpl_ownable::OWNER
            .save(storage, &Addr::unchecked(owner))
            .unwrap();
    }

    #[test]
    fn test_pause() {
        let mut deps = mock_dependencies();
        mock_owner(deps.as_mut().storage, ADDR1_VALUE);

        let unauthorized =
            pause(deps.as_mut(), mock_env(), mock_info(ADDR2_VALUE, &[])).unwrap_err();
        assert!(matches!(unauthorized, ContractError::Unauthorized {}));

        // already paused
        PAUSE.save(deps.as_mut().storage, &true).unwrap();
        let already_paused =
            pause(deps.as_mut(), mock_env(), mock_info(ADDR1_VALUE, &[])).unwrap_err();
        assert!(matches!(already_paused, ContractError::Paused {}));

        // success
        PAUSE.save(deps.as_mut().storage, &false).unwrap();
        let res = pause(deps.as_mut(), mock_env(), mock_info(ADDR1_VALUE, &[])).unwrap();

        assert!(PAUSE.load(deps.as_ref().storage).unwrap());
        assert_eq!(res, Response::new().add_event(Event::new("pause")));
    }

    #[test]
    fn test_unpause() {
        let mut deps = mock_dependencies();
        mock_owner(deps.as_mut().storage, ADDR1_VALUE);

        let unauthorized =
            unpause(deps.as_mut(), mock_env(), mock_info(ADDR2_VALUE, &[])).unwrap_err();
        assert!(matches!(unauthorized, ContractError::Unauthorized {}));

        // not paused
        PAUSE.save(deps.as_mut().storage, &false).unwrap();
        let not_paused =
            unpause(deps.as_mut(), mock_env(), mock_info(ADDR1_VALUE, &[])).unwrap_err();
        assert!(matches!(not_paused, ContractError::NotPaused {}));

        // success
        PAUSE.save(deps.as_mut().storage, &true).unwrap();
        let res = unpause(deps.as_mut(), mock_env(), mock_info(ADDR1_VALUE, &[])).unwrap();

        assert!(!PAUSE.load(deps.as_ref().storage).unwrap());
        assert_eq!(res, Response::new().add_event(Event::new("unpause")));
    }

    #[test]
    fn test_update_mailbox() {
        let mut deps = mock_dependencies();
        mock_owner(deps.as_mut().storage, ADDR1_VALUE);

        let unauthorized =
            pause(deps.as_mut(), mock_env(), mock_info(ADDR2_VALUE, &[])).unwrap_err();
        assert!(matches!(unauthorized, ContractError::Unauthorized {}));

        // already paused
        PAUSE.save(deps.as_mut().storage, &true).unwrap();
        let already_paused =
            pause(deps.as_mut(), mock_env(), mock_info(ADDR1_VALUE, &[])).unwrap_err();
        assert!(matches!(already_paused, ContractError::Paused {}));

        PAUSE.save(deps.as_mut().storage, &false).unwrap();
        let mailbox = Addr::unchecked("input-mailbox");

        let res = update_mailbox(
            deps.as_mut(),
            mock_env(),
            mock_info(ADDR1_VALUE, &[]),
            mailbox.to_string(),
        )
        .unwrap();

        assert_eq!(
            MAILBOX.load(deps.as_ref().storage).unwrap(),
            mailbox.to_string()
        );
        assert_eq!(
            res,
            Response::new()
                .add_event(Event::new("update-mailbox").add_attribute("mailbox", mailbox))
        );
    }
}
