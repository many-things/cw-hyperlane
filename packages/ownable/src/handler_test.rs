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
