use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{
    event::{
        emit_finish_transfer_ownership, emit_init_transfer_ownership,
        emit_revoke_transfer_ownership,
    },
    state::{
        assert_owned, assert_pending_owner, assert_pending_owner_empty, assert_pending_owner_exist,
        Config, CONFIG, PENDING_OWNER,
    },
    ContractError,
};

pub fn init_transfer_ownership(
    deps: DepsMut,
    info: MessageInfo,
    next_owner: String,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender)?;
    assert_pending_owner_empty(deps.storage)?;

    PENDING_OWNER.save(deps.storage, &deps.api.addr_validate(&next_owner)?)?;

    Ok(Response::new().add_event(emit_init_transfer_ownership(next_owner)))
}

pub fn finish_transfer_ownership(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    assert_pending_owner(deps.storage, info.sender.clone())?;

    let config = CONFIG.load(deps.storage)?;
    let pending_owner = PENDING_OWNER.load(deps.storage)?;

    CONFIG.save(
        deps.storage,
        &Config {
            owner: pending_owner,
            ..config
        },
    )?;

    // FIXME: define event
    Ok(Response::new().add_event(emit_finish_transfer_ownership(info.sender)))
}

pub fn revoke_transfer_ownership(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender)?;
    assert_pending_owner_exist(deps.storage)?;

    PENDING_OWNER.remove(deps.storage);

    Ok(Response::new().add_event(emit_revoke_transfer_ownership()))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr, Storage,
    };

    use super::*;
    const ADDR1_VAULE: &str = "addr1";
    const ADDR2_VAULE: &str = "addr2";

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        let config = Config {
            owner,
            chain_hpl: String::new(),
        };

        CONFIG.save(storage, &config).unwrap();
    }

    #[test]
    fn test_init_transfer_ownership_failed() {
        let mut deps = mock_dependencies();

        let owner = Addr::unchecked(ADDR1_VAULE);
        let abuser = Addr::unchecked(ADDR2_VAULE);

        mock_owner(deps.as_mut().storage, owner.clone());

        // NOT OWNED ASSERT
        let not_owned_info = mock_info(abuser.as_str(), &[]);
        let not_owned_assert =
            init_transfer_ownership(deps.as_mut(), not_owned_info, "ADDR3".to_string())
                .unwrap_err();

        assert!(matches!(not_owned_assert, ContractError::Unauthorized {}));

        // Transfer Already Started
        PENDING_OWNER.save(deps.as_mut().storage, &owner).unwrap();
        let duplicated_info = mock_info(owner.as_str(), &[]);

        let already_started_assert =
            init_transfer_ownership(deps.as_mut(), duplicated_info, "ADDR3".to_string())
                .unwrap_err();

        assert!(matches!(
            already_started_assert,
            ContractError::OwnershipTransferAlreadyStarted {}
        ));
    }

    #[test]
    fn test_init_transfer_ownership_success() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked(ADDR1_VAULE);
        let next_owner = "osmo1pe6jpke2wvufly7y6k3xuhlfy7n2knpts222g4".to_string();

        mock_owner(deps.as_mut().storage, owner.clone());

        let info = mock_info(owner.as_str(), &[]);
        let result = init_transfer_ownership(deps.as_mut(), info, next_owner.clone()).unwrap();

        assert_eq!(
            result.events,
            vec![emit_init_transfer_ownership(next_owner)]
        )
    }
}
