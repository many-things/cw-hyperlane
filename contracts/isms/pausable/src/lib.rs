#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, to_json_binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, QueryResponse, Response,
    StdError,
};
use hpl_interface::ism::{
    pausable::{ExecuteMsg, InstantiateMsg, QueryMsg},
    IsmQueryMsg, IsmType, ModuleTypeResponse, VerifyResponse,
};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("hook paused")]
    Paused {},
}

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_hook_pausable::{}", name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;

    hpl_ownable::initialize(deps.storage, &owner)?;
    hpl_pausable::initialize(deps.storage, &msg.paused)?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::Pausable(msg) => Ok(hpl_pausable::handle(deps, env, info, msg)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use IsmQueryMsg::*;

    match msg {
        QueryMsg::Pausable(msg) => Ok(hpl_pausable::handle_query(deps, env, msg)?),
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Ism(msg) => match msg {
            ModuleType {} => Ok(to_json_binary(&ModuleTypeResponse { typ: IsmType::Null })?),
            Verify {
                metadata: _,
                message: _,
            } => {
                ensure!(
                    !hpl_pausable::get_pause_info(deps.storage)?,
                    ContractError::Paused {}
                );
                Ok(to_json_binary(&VerifyResponse { verified: true })?)
            }
            _ => unimplemented!(),
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        from_json,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_json_binary, Addr, OwnedDeps,
    };
    use hpl_ownable::get_owner;
    use hpl_pausable::get_pause_info;
    use ibcx_test_utils::{addr, hex};
    use rstest::{fixture, rstest};

    use super::*;

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    fn query(deps: Deps, msg: crate::QueryMsg) -> Result<QueryResponse, ContractError> {
        let req: QueryMsg = from_json(to_json_binary(&msg).unwrap()).unwrap();
        crate::query(deps, mock_env(), req)
    }

    #[fixture]
    fn deps(
        #[default(addr("deployer"))] sender: Addr,
        #[default(addr("owner"))] owner: Addr,
        #[default(false)] paused: bool,
    ) -> TestDeps {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                owner: owner.to_string(),
                paused,
            },
        )
        .unwrap();

        deps
    }

    #[rstest]
    fn test_init(deps: TestDeps) {
        assert!(!get_pause_info(deps.as_ref().storage).unwrap());
        assert_eq!("owner", get_owner(deps.as_ref().storage).unwrap().as_str());
    }

    #[rstest]
    #[case(false)]
    #[should_panic(expected = "hook paused")]
    #[case(true)]
    fn test_query(mut deps: TestDeps, #[case] paused: bool) {
        if paused {
            hpl_pausable::pause(deps.as_mut().storage, &addr("owner")).unwrap();
        }

        let raw_message = hex("0000000000000068220000000000000000000000000d1255b09d94659bb0888e0aa9fca60245ce402a0000682155208cd518cffaac1b5d8df216a9bd050c9a03f0d4f3ba88e5268ac4cd12ee2d68656c6c6f");
        let raw_metadata = raw_message.clone();

        query(
            deps.as_ref(),
            QueryMsg::Ism(IsmQueryMsg::Verify {
                metadata: raw_metadata,
                message: raw_message,
            }),
        )
        .map_err(|e| e.to_string())
        .unwrap();
    }
}
