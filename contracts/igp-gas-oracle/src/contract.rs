#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response};

use hpl_interface::igp_gas_oracle::{
    ExecuteMsg, GetExchangeRateAndGasPriceResponse, InstantiateMsg, MigrateMsg, QueryMsg,
};

use crate::{
    error::ContractError,
    state::{insert_gas_data, OWNER, REMOTE_GAS_DATA},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_event(Event::new("init-igp-gas-oracle").add_attribute("owner", info.sender)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRemoteGasDataConfigs { configs } => {
            if info.sender != OWNER.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }

            let mut domains = vec![];
            for config in configs {
                domains.push(config.remote_domain.to_string());
                insert_gas_data(deps.storage, config)?;
            }

            Ok(Response::new().add_event(
                Event::new("set-gas-configs").add_attribute("domains", domains.join(",")),
            ))
        }
        ExecuteMsg::SetRemoteGasData { config } => {
            if info.sender != OWNER.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }

            let domain = config.remote_domain.to_string();
            insert_gas_data(deps.storage, config)?;

            Ok(Response::new()
                .add_event(Event::new("set-gas-config").add_attribute("domain", domain)))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::GetExchangeRateAndGasPrice { dest_domain } => {
            let gas_data = REMOTE_GAS_DATA.load(deps.storage, dest_domain)?;

            Ok(to_binary(&GetExchangeRateAndGasPriceResponse {
                gas_price: gas_data.gas_price,
                exchange_rate: gas_data.token_exchange_rate,
            })?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
