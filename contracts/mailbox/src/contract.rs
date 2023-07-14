#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};
use cw2::set_contract_version;
use hpl_interface::mailbox::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::{
    error::ContractError,
    event::emit_instantiated,
    state::{assert_paused, Config, CONFIG, NONCE, PAUSE},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        factory: info.sender,
        default_ism: deps.api.addr_validate(&msg.default_ism)?,
    };

    CONFIG.save(deps.storage, &config)?;
    PAUSE.save(deps.storage, &false)?;
    NONCE.save(deps.storage, &0)?;

    Ok(Response::new().add_event(emit_instantiated(config.owner)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::core;
    use crate::gov;
    use ExecuteMsg::*;

    assert_paused(deps.storage)?;

    match msg {
        Pause {} => gov::pause(deps, info),
        Unpause {} => gov::unpause(deps, info),
        SetDefaultISM {
            ism: new_default_ism,
        } => gov::set_default_ism(deps, info, new_default_ism),

        Dispatch {
            dest_domain,
            recipient_addr,
            msg_body,
        } => core::dispatch(deps, info, dest_domain, recipient_addr, msg_body),
        Process { metadata, message } => core::process(deps, metadata, message),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;
    use QueryMsg::*;

    match msg {
        Root {} => query::get_root(deps),
        Count {} => query::get_count(deps),
        CheckPoint {} => query::get_checkpoint(deps),
        Paused {} => query::get_paused(deps),
        Nonce {} => query::get_nonce(deps),
        DefaultIsm {} => query::get_default_ism(deps),
        MessageDelivered { id } => query::get_delivered(deps, id),
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    const OWNER: &str = "owner";
    const DEFAULT_ISM: &str = "default_ism";

    #[test]
    fn init() {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", &[]),
            InstantiateMsg {
                owner: OWNER.to_string(),
                default_ism: DEFAULT_ISM.to_string(),
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(
            version,
            cw2::ContractVersion {
                contract: CONTRACT_NAME.to_string(),
                version: CONTRACT_VERSION.to_string()
            }
        );
    }
}
