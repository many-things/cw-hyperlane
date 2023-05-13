#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, to_vec, Binary, ContractResult, Deps, DepsMut, Empty, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult, SystemResult,
};
use cw2::set_contract_version;
use hpl_interface::multicall::{
    AggregateResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};

use crate::{
    error::ContractError,
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
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", config.owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
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
        Aggregate(msgs) => {
            let config = CONFIG.load(deps.storage)?;

            assert_eq!(config.owner, info.sender, "not an owner");

            let resp = Response::new().add_messages(msgs);

            Ok(resp)
        }
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use QueryMsg::*;

    match msg {
        AggregateStatic(calls) => {
            let resps = calls
                .into_iter()
                .map(|call| {
                    let raw = to_vec(&QueryRequest::<Empty>::Stargate {
                        path: call.path,
                        data: call.data,
                    })?;

                    match deps.querier.raw_query(&raw) {
                        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
                            "Querier system error: {}",
                            system_err
                        ))),
                        SystemResult::Ok(ContractResult::Err(contract_err)) => {
                            Err(StdError::generic_err(format!(
                                "Querier contract error: {}",
                                contract_err
                            )))
                        }
                        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
                    }
                })
                .collect::<StdResult<Vec<_>>>()?;

            Ok(to_binary(&AggregateResponse(resps))?)
        }
    }
}
