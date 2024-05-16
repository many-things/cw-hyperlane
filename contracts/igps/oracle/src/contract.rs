#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, to_json_binary, Deps, DepsMut, Empty, Env, Event, MessageInfo,
    QueryResponse, Response,
};

use hpl_interface::igp::oracle::{
    ExecuteMsg, GetExchangeRateAndGasPriceResponse, IgpGasOracleQueryMsg, InstantiateMsg, QueryMsg,
};
use hpl_ownable::get_owner;

use crate::{
    error::ContractError,
    state::{insert_gas_data, REMOTE_GAS_DATA},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    Ok(Response::new().add_event(Event::new("init-igp-gas-oracle").add_attribute("owner", owner)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ownership(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),

        ExecuteMsg::SetRemoteGasDataConfigs { configs } => {
            ensure_eq!(
                info.sender,
                get_owner(deps.storage)?,
                ContractError::Unauthorized {}
            );

            let mut domains = vec![];
            for config in configs {
                ensure!(
                    !config.token_exchange_rate.is_zero(),
                    ContractError::invalid_config("exchange rate must be non-zero")
                );

                domains.push(config.remote_domain.to_string());
                insert_gas_data(deps.storage, config)?;
            }

            Ok(Response::new().add_event(
                Event::new("set-gas-configs")
                    .add_attribute("owner", info.sender)
                    .add_attribute("domains", domains.join(",")),
            ))
        }
        ExecuteMsg::SetRemoteGasData { config } => {
            ensure_eq!(
                info.sender,
                get_owner(deps.storage)?,
                ContractError::Unauthorized {}
            );

            let domain = config.remote_domain.to_string();
            insert_gas_data(deps.storage, config)?;

            Ok(Response::new()
                .add_event(Event::new("set-gas-config").add_attribute("domain", domain)))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Oracle(msg) => match msg {
            IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { dest_domain } => {
                let gas_data = REMOTE_GAS_DATA.load(deps.storage, dest_domain)?;

                Ok(to_json_binary(&GetExchangeRateAndGasPriceResponse {
                    gas_price: gas_data.gas_price,
                    exchange_rate: gas_data.token_exchange_rate,
                })?)
            }
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
