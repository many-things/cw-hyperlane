#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};

use hpl_interface::igp_gas_oracle;

use crate::error::ContractError;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: igp_gas_oracle::InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: igp_gas_oracle::ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: igp_gas_oracle::QueryMsg,
) -> Result<QueryResponse, ContractError> {
    Ok(Binary::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    env: Env,
    msg: igp_gas_oracle::MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
