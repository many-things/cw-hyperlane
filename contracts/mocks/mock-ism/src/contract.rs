use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdResult,
};
use cw2::set_contract_version;
use hpl_interface::ism::{
    ExpectedIsmQueryMsg, ISMType, IsmQueryMsg, VerifyInfoResponse, VerifyResponse,
};

use crate::{CONTRACT_NAME, CONTRACT_VERSION};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ExecuteMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: ExpectedIsmQueryMsg) -> StdResult<QueryResponse> {
    use IsmQueryMsg::*;

    match msg {
        ExpectedIsmQueryMsg::Ism(msg) => match msg {
            ModuleType {} => Ok(to_binary(&ISMType::Null)?),
            Verify { .. } => Ok(to_binary(&VerifyResponse { verified: true })?),
            VerifyInfo { .. } => Ok(to_binary(&VerifyInfoResponse {
                threshold: 1u8,
                validators: vec!["".to_string()],
            })?),
        },
    }
}
