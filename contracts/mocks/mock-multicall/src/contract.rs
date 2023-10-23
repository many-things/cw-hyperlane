use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, QueryResponse, Response, StdResult,
};
use cw2::set_contract_version;

use crate::{CONTRACT_NAME, CONTRACT_VERSION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: CosmosMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_message(msg))
}

#[cfg(not(feature = "library"))]
#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: cosmwasm_std::QueryRequest<cosmwasm_std::Empty>,
) -> StdResult<QueryResponse> {
    use cosmwasm_std::{to_vec, ContractResult, StdError, SystemResult};

    let req_bin = to_vec(&msg).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {serialize_err}"))
    })?;

    match deps.querier.raw_query(&req_bin) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {system_err}"
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {contract_err}"
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }
}
