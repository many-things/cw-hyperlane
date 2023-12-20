use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, CustomQuery, Empty, Env, MessageInfo, OwnedDeps, Response, StdError, StdResult,
};
use hpl_interface::pausable::{PausableMsg, PausableQueryMsg, PauseInfoResponse};
use rstest::rstest;
use serde::de::DeserializeOwned;

use crate::{event_to_resp, handle, handle_query, new_event};

pub struct Pausable<C: CustomQuery = Empty> {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, C>,
    pub env: Env,
}

impl<C> Pausable<C>
where
    C: CustomQuery,
{
    pub fn new(deps: OwnedDeps<MockStorage, MockApi, MockQuerier, C>, env: Env) -> Self {
        Self { deps, env }
    }

    fn handle(&mut self, info: MessageInfo, msg: PausableMsg) -> StdResult<Response> {
        handle(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    fn query<T: DeserializeOwned>(&self, msg: PausableQueryMsg) -> StdResult<T> {
        from_json(handle_query(self.deps.as_ref(), self.env.clone(), msg)?)
    }

    pub fn set_owner(&mut self, owner: &Addr) -> StdResult<()> {
        hpl_ownable::initialize(self.deps.as_mut().storage, owner)
    }

    pub fn pause(&mut self, sender: &Addr) -> StdResult<Response> {
        self.handle(mock_info(sender.as_str(), &[]), PausableMsg::Pause {})
    }

    pub fn release(&mut self, sender: &Addr) -> StdResult<Response> {
        self.handle(mock_info(sender.as_str(), &[]), PausableMsg::Release {})
    }

    pub fn pause_info(&self) -> StdResult<bool> {
        let resp: PauseInfoResponse = self.query(PausableQueryMsg::PauseInfo {})?;

        Ok(resp.paused)
    }
}

fn pausable_default() -> Pausable {
    Pausable::new(mock_dependencies(), mock_env())
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_initialize(#[case] paused: bool) -> anyhow::Result<()> {
    let mut pausable = pausable_default();

    crate::initialize(pausable.deps.as_mut().storage, &paused)?;

    let paused_saved = pausable.pause_info()?;

    assert_eq!(paused, paused_saved);

    Ok(())
}

#[rstest]
#[case(false,"owner", Ok(("pause", "owner")))]
#[case(false, "abuser", Err(StdError::generic_err("unauthorized")))]
#[case(true, "owner", Err(StdError::generic_err("already paused")))]
#[case(true, "abuser", Err(StdError::generic_err("unauthorized")))]
fn test_pause(
    #[case] init_state: bool,
    #[case] sender: &str,
    #[case] expected: StdResult<(&str, &str)>,
) -> anyhow::Result<()> {
    let mut pausable = pausable_default();

    // init
    pausable.set_owner(&Addr::unchecked("owner"))?;
    crate::initialize(pausable.deps.as_mut().storage, &init_state)?;

    // call
    let res = pausable.pause(&Addr::unchecked(sender));

    assert_eq!(
        res,
        expected.map(|(event_name, sender)| {
            event_to_resp(new_event(event_name).add_attribute("sender", sender))
        })
    );

    // validate
    if res.is_ok() {
        assert!(pausable.pause_info()?);
    }

    Ok(())
}

#[rstest]
#[case(true,"owner", Ok(("release", "owner")))]
#[case(true, "abuser", Err(StdError::generic_err("unauthorized")))]
#[case(false, "owner", Err(StdError::generic_err("already released")))]
#[case(false, "abuser", Err(StdError::generic_err("unauthorized")))]
fn test_release(
    #[case] init_state: bool,
    #[case] sender: &str,
    #[case] expected: StdResult<(&str, &str)>,
) -> anyhow::Result<()> {
    let mut pausable = pausable_default();

    // init
    pausable.set_owner(&Addr::unchecked("owner"))?;
    crate::initialize(pausable.deps.as_mut().storage, &init_state)?;

    // call
    let res = pausable.release(&Addr::unchecked(sender));

    assert_eq!(
        res,
        expected.map(|(event_name, sender)| {
            event_to_resp(new_event(event_name).add_attribute("sender", sender))
        })
    );

    // validate
    if res.is_ok() {
        assert!(!pausable.pause_info()?);
    }

    Ok(())
}
