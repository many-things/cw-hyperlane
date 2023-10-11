use cosmwasm_std::{
    to_binary, Addr, CustomQuery, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response,
    StdError, StdResult, Storage,
};
use cw_storage_plus::Item;
use hpl_interface::ownable::{OwnableMsg, OwnableQueryMsg, OwnerResponse, PendingOwnerResponse};

pub const OWNER_KEY: &str = "owner";
pub const OWNER: Item<Addr> = Item::new(OWNER_KEY);

pub const PENDING_OWNER_KEY: &str = "pending_owner";
pub const PENDING_OWNER: Item<Addr> = Item::new(PENDING_OWNER_KEY);

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

fn init_ownership_transfer(
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

fn revoke_ownership_transfer(storage: &mut dyn Storage, sender: &Addr) -> StdResult<Event> {
    if sender != OWNER.load(storage)? {
        return Err(StdError::generic_err("unauthorized"));
    }
    if !PENDING_OWNER.exists(storage) {
        return Err(StdError::generic_err("ownership is not transferring"));
    }

    PENDING_OWNER.remove(storage);

    Ok(Event::new("revoke-ownership-transfer").add_attribute("owner", sender))
}

fn claim_ownership(storage: &mut dyn Storage, sender: &Addr) -> StdResult<Event> {
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

pub fn handle_query<C: CustomQuery>(
    deps: Deps<'_, C>,
    _env: Env,
    msg: OwnableQueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        OwnableQueryMsg::GetOwner {} => to_binary(&OwnerResponse {
            owner: get_owner(deps.storage)?,
        }),
        OwnableQueryMsg::GetPendingOwner {} => to_binary(&PendingOwnerResponse {
            pending_owner: get_pending_owner(deps.storage)?,
        }),
    }
}

pub fn get_owner(storage: &dyn Storage) -> StdResult<Addr> {
    let owner = OWNER.load(storage)?;

    Ok(owner)
}

pub fn get_pending_owner(storage: &dyn Storage) -> StdResult<Option<Addr>> {
    let pending_owner = PENDING_OWNER.may_load(storage)?;

    Ok(pending_owner)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, Api, CustomQuery, Empty, Env, MessageInfo, OwnedDeps, Querier, Response, StdError,
        StdResult, Storage,
    };
    use hpl_interface::ownable::OwnableMsg;

    use crate::{handle, OWNER, PENDING_OWNER};

    pub struct Ownable<S: Storage, A: Api, Q: Querier, C: CustomQuery = Empty> {
        pub deps: OwnedDeps<S, A, Q, C>,
        pub env: Env,
    }

    impl<S, A, Q, C> Ownable<S, A, Q, C>
    where
        S: Storage,
        A: Api,
        Q: Querier,
        C: CustomQuery,
    {
        pub fn new(deps: OwnedDeps<S, A, Q, C>, env: Env) -> Self {
            Self { deps, env }
        }

        fn handle(&mut self, info: MessageInfo, msg: OwnableMsg) -> StdResult<Response> {
            handle(self.deps.as_mut(), self.env.clone(), info, msg)
        }

        pub fn init(&mut self, sender: &Addr, next_owner: &Addr) -> StdResult<Response> {
            self.handle(
                mock_info(sender.as_str(), &[]),
                OwnableMsg::InitOwnershipTransfer {
                    next_owner: next_owner.to_string(),
                },
            )
        }

        pub fn revoke(&mut self, sender: &Addr) -> StdResult<Response> {
            self.handle(
                mock_info(sender.as_str(), &[]),
                OwnableMsg::RevokeOwnershipTransfer {},
            )
        }

        pub fn claim(&mut self, sender: &Addr) -> StdResult<Response> {
            self.handle(
                mock_info(sender.as_str(), &[]),
                OwnableMsg::ClaimOwnership {},
            )
        }

        pub fn set_owner(&mut self, owner: &Addr) -> StdResult<()> {
            OWNER.save(self.deps.as_mut().storage, owner)
        }

        pub fn owner(&self) -> StdResult<Addr> {
            OWNER.load(self.deps.as_ref().storage)
        }

        pub fn pending_owner(&self) -> StdResult<Option<Addr>> {
            PENDING_OWNER.may_load(self.deps.as_ref().storage)
        }
    }

    fn ownable_default() -> Ownable<MockStorage, MockApi, MockQuerier, Empty> {
        Ownable::new(mock_dependencies(), mock_env())
    }

    #[test]
    fn test_handle() -> anyhow::Result<()> {
        let deployer = Addr::unchecked("deployer");
        let next_owner = Addr::unchecked("next_owner");

        let mut ownable = ownable_default();

        // initial setup
        ownable.set_owner(&deployer)?;

        // test init transfer ownership
        ownable.init(&deployer, &next_owner)?;
        assert_eq!(ownable.owner()?, deployer.to_string());
        assert_eq!(ownable.pending_owner()?, Some(next_owner.clone()));

        // test revoke transfer ownership
        ownable.revoke(&deployer)?;
        assert_eq!(ownable.owner()?, deployer.to_string());
        assert_eq!(ownable.pending_owner()?, None);

        // test claim transfer ownership
        ownable.init(&deployer, &next_owner)?;
        ownable.claim(&next_owner)?;

        assert_eq!(ownable.owner()?, next_owner.to_string());
        assert_eq!(ownable.pending_owner()?, None);

        Ok(())
    }

    #[test]
    fn test_init() -> anyhow::Result<()> {
        let deployer = Addr::unchecked("deployer");
        let next_owner = Addr::unchecked("next_owner");

        let mut ownable = Ownable::new(mock_dependencies(), mock_env());

        // initial setup
        ownable.set_owner(&deployer)?;

        // ok
        ownable.init(&deployer, &next_owner)?;

        // fail - sender is not owner
        let err = ownable.init(&next_owner, &deployer).unwrap_err();
        assert_eq!(err, StdError::generic_err("unauthorized"));

        // fail - pending_owner is not empty
        let err = ownable.init(&deployer, &next_owner).unwrap_err();
        assert_eq!(err, StdError::generic_err("ownership is transferring"));

        Ok(())
    }

    #[test]
    fn test_revoke() -> anyhow::Result<()> {
        let deployer = Addr::unchecked("deployer");
        let next_owner = Addr::unchecked("next_owner");

        let mut ownable = Ownable::new(mock_dependencies(), mock_env());

        // initial setup
        ownable.set_owner(&deployer)?;

        // fail - pending_owner is empty
        let err = ownable.revoke(&deployer).unwrap_err();
        assert_eq!(err, StdError::generic_err("ownership is not transferring"));

        // initiate ownership transfer
        ownable.init(&deployer, &next_owner)?;

        // fail - sender is not owner
        let err = ownable.revoke(&next_owner).unwrap_err();
        assert_eq!(err, StdError::generic_err("unauthorized"));

        // ok
        ownable.revoke(&deployer)?;

        Ok(())
    }

    #[test]
    fn test_claim() -> anyhow::Result<()> {
        let deployer = Addr::unchecked("deployer");
        let next_owner = Addr::unchecked("next_owner");

        let mut ownable = Ownable::new(mock_dependencies(), mock_env());

        // initial setup
        ownable.set_owner(&deployer)?;

        // fail - pending_owner is empty
        let err = ownable.claim(&next_owner).unwrap_err();
        assert_eq!(err, StdError::generic_err("ownership is not transferring"));

        // initiate ownership transfer
        ownable.init(&deployer, &next_owner)?;

        // fail - sender is not pending_owner
        let err = ownable.claim(&deployer).unwrap_err();
        assert_eq!(err, StdError::generic_err("unauthorized"));

        // ok
        ownable.claim(&next_owner)?;

        Ok(())
    }
}
