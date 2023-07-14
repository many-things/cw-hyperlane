#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};

use hpl_interface::igp_core;

use crate::{error::ContractError, CONTRACT_NAME, CONTRACT_VERSION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: igp_core::InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    hpl_ownable::OWNER.save(deps.storage, &deps.api.addr_validate(&msg.owner)?)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: igp_core::ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        igp_core::ExecuteMsg::Ownership(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        igp_core::ExecuteMsg::SetGasOracles { configs } => todo!(),
        igp_core::ExecuteMsg::SetBeneficiary { beneficiary } => todo!(),
        igp_core::ExecuteMsg::PayForGas {
            message_id,
            dest_domain,
            gas_amount,
            refund_address,
        } => todo!(),
        igp_core::ExecuteMsg::Claim {} => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: igp_core::QueryMsg,
) -> Result<QueryResponse, ContractError> {
    Ok(Binary::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    env: Env,
    msg: igp_core::MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
