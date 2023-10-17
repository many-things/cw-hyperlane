use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, CustomQuery, Empty, Env, MessageInfo, OwnedDeps, Response, StdError, StdResult,
};
use hpl_interface::ownable::{OwnableMsg, OwnableQueryMsg, OwnerResponse, PendingOwnerResponse};
use rstest::rstest;
use serde::de::DeserializeOwned;

use crate::{handle, handle_query};

pub struct Ownable<C: CustomQuery = Empty> {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, C>,
    pub env: Env,
}

impl<C> Ownable<C>
where
    C: CustomQuery,
{
    pub fn new(deps: OwnedDeps<MockStorage, MockApi, MockQuerier, C>, env: Env) -> Self {
        Self { deps, env }
    }

    fn handle(&mut self, info: MessageInfo, msg: OwnableMsg) -> StdResult<Response> {
        handle(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    fn query<T: DeserializeOwned>(&self, msg: OwnableQueryMsg) -> StdResult<T> {
        from_binary(&handle_query(self.deps.as_ref(), self.env.clone(), msg)?)
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

    pub fn owner(&self) -> StdResult<Addr> {
        let resp: OwnerResponse = self.query(OwnableQueryMsg::GetOwner {})?;

        Ok(resp.owner)
    }

    pub fn pending_owner(&self) -> StdResult<Option<Addr>> {
        let resp: PendingOwnerResponse = self.query(OwnableQueryMsg::GetPendingOwner {})?;

        Ok(resp.pending_owner)
    }
}

fn ownable_default() -> Ownable {
    Ownable::new(mock_dependencies(), mock_env())
}

#[rstest]
#[case(Addr::unchecked("hello"))]
#[case(Addr::unchecked("world"))]
fn test_initialize(#[case] owner: Addr) -> anyhow::Result<()> {
    let mut ownable = ownable_default();

    crate::initialize(ownable.deps.as_mut().storage, &owner)?;

    let owner_saved = ownable.owner()?;

    assert_eq!(owner, owner_saved);

    Ok(())
}

#[test]
fn test_handle() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let next_owner = Addr::unchecked("next_owner");

    let mut ownable = ownable_default();

    // initial setup
    crate::initialize(ownable.deps.as_mut().storage, &deployer)?;

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
    crate::initialize(ownable.deps.as_mut().storage, &deployer)?;

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
    crate::initialize(ownable.deps.as_mut().storage, &deployer)?;

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
    crate::initialize(ownable.deps.as_mut().storage, &deployer)?;

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
