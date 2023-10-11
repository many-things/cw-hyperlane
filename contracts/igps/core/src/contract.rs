#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response};

use hpl_interface::igp_core::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::state::{Config, CONFIG};
use crate::{
    error::ContractError,
    state::{BENEFICIARY, GAS_TOKEN},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    hpl_ownable::OWNER.save(deps.storage, &deps.api.addr_validate(&msg.owner)?)?;
    BENEFICIARY.save(deps.storage, &deps.api.addr_validate(&msg.beneficiary)?)?;
    GAS_TOKEN.save(deps.storage, &msg.gas_token)?;
    CONFIG.save(deps.storage, &Config { prefix: msg.prefix })?;

    Ok(Response::new().add_event(
        Event::new("init-igp-core")
            .add_attribute("owner", msg.owner)
            .add_attribute("creator", info.sender)
            .add_attribute("beneficiary", msg.beneficiary),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute;

    match msg {
        ExecuteMsg::Ownership(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::SetGasOracles { configs } => execute::set_gas_oracle(deps, info, configs),
        ExecuteMsg::SetBeneficiary { beneficiary } => {
            execute::set_beneficiary(deps, info, beneficiary)
        }
        ExecuteMsg::Claim {} => execute::claim(deps, env, info),
        ExecuteMsg::PostDispatch { metadata, message } => {
            execute::post_dispatch(deps, info, metadata, message)
        }
        ExecuteMsg::PayForGas {
            message_id,
            dest_domain,
            gas_amount,
            refund_address,
        } => execute::pay_for_gas(
            deps,
            info,
            message_id,
            dest_domain,
            gas_amount,
            refund_address,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query::*;

    match msg {
        QueryMsg::QuoteGasPayment {
            dest_domain,
            gas_amount,
        } => quote_gas_payment(deps, dest_domain, gas_amount),
        QueryMsg::GetExchangeRateAndGasPrice { dest_domain } => {
            get_exchange_rate_and_gas_price(deps, dest_domain)
        }
        QueryMsg::QuoteDispatch(msg) => quote_dispatch(deps, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
