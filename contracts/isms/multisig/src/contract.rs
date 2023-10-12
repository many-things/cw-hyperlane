#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response};
use cw2::set_contract_version;
use hpl_interface::ism::multisig::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::{
    error::ContractError,
    execute,
    state::{Config, CONFIG},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        addr_prefix: msg.addr_prefix,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", config.owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    Ok(Response::default())
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        EnrollValidator { set: msg } => execute::enroll_validator(deps, info, msg),
        EnrollValidators { set: validators } => execute::enroll_validators(deps, info, validators),
        UnenrollValidator {
            domain,
            validator: vald,
        } => execute::unenroll_validator(deps, info, domain, vald),
        SetThreshold { set: threshold } => execute::set_threshold(deps, info, threshold),
        SetThresholds { set: thresholds } => execute::set_thresholds(deps, info, thresholds),
        InitTransferOwnership { owner: next_owner } => {
            execute::init_transfer_ownership(deps, info, next_owner)
        }
        FinishTransferOwnership() => execute::finish_transfer_ownership(deps, info),
        RevokeTransferOwnership() => execute::revoke_transfer_ownership(deps, info),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use crate::query;
    use QueryMsg::*;

    match msg {
        ModuleType {} => query::get_module_type(),
        Verify {
            metadata: raw_metadata,
            message: raw_message,
        } => query::verify_message(deps, raw_metadata, raw_message),
        VerifyInfo {
            message: raw_message,
        } => query::get_verify_info(deps, raw_message),
    }
}
