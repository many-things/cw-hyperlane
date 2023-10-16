#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use hpl_interface::ism::{
    multisig::{ExecuteMsg, InstantiateMsg, MultisigIsmQueryMsg, QueryMsg},
    IsmQueryMsg,
};

use crate::{error::ContractError, execute, state::HRP, CONTRACT_NAME, CONTRACT_VERSION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    HRP.save(deps.storage, &msg.hrp)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        EnrollValidator { set: msg } => execute::enroll_validator(deps, info, msg),
        EnrollValidators { set: validators } => execute::enroll_validators(deps, info, validators),
        UnenrollValidator {
            domain,
            validator: vald,
        } => execute::unenroll_validator(deps, info, domain, vald),
        SetThreshold { set: threshold } => execute::set_threshold(deps, info, threshold),
        SetThresholds { set: thresholds } => execute::set_thresholds(deps, info, thresholds),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use crate::query;
    use IsmQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Ism(msg) => match msg {
            ModuleType {} => query::get_module_type(),
            Verify {
                metadata: raw_metadata,
                message: raw_message,
            } => query::verify_message(deps, raw_metadata, raw_message),
            VerifyInfo {
                message: raw_message,
            } => query::get_verify_info(deps, raw_message),
        },
        QueryMsg::MultisigIsm(msg) => match msg {
            MultisigIsmQueryMsg::EnrolledValidators { .. } => todo!(),
        },
    }
}
