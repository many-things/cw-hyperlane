#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, Deps, DepsMut, Empty, Env, Event, MessageInfo, QueryResponse, Response,
};
use cw2::set_contract_version;
use hpl_interface::{
    ism::{
        multisig::{
            EnrolledValidatorsResponse, ExecuteMsg, InstantiateMsg, MultisigIsmQueryMsg, QueryMsg,
        },
        IsmQueryMsg,
    },
    to_binary,
};
use hpl_ownable::get_owner;

use crate::{
    error::ContractError,
    state::{THRESHOLD, VALIDATORS},
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

    let owner = deps.api.addr_validate(&msg.owner)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

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
        SetValidators {
            domain,
            threshold,
            validators,
        } => {
            ensure_eq!(
                info.sender,
                get_owner(deps.storage)?,
                ContractError::Unauthorized {}
            );
            ensure!(
                validators.iter().all(|v| v.len() == 20),
                ContractError::invalid_addr("length should be 20")
            );
            ensure!(
                validators.len() > threshold as usize && threshold > 0,
                ContractError::invalid_args(&format!(
                    "threshold not in range. 0 <  <= {}",
                    validators.len(),
                ))
            );

            VALIDATORS.save(deps.storage, domain, &validators)?;
            THRESHOLD.save(deps.storage, domain, &threshold)?;

            Ok(Response::new().add_event(
                Event::new("ism_multisig_set_validators")
                    .add_attribute("sender", info.sender)
                    .add_attribute("domain", domain.to_string())
                    .add_attribute("validators", validators.len().to_string())
                    .add_attribute("threshold", threshold.to_string()),
            ))
        }
        UnsetDomain { domain } => {
            ensure_eq!(
                info.sender,
                get_owner(deps.storage)?,
                ContractError::Unauthorized {}
            );

            VALIDATORS.remove(deps.storage, domain);
            THRESHOLD.remove(deps.storage, domain);

            Ok(Response::new().add_event(
                Event::new("ism_multisig_unset_domain")
                    .add_attribute("sender", info.sender)
                    .add_attribute("domain", domain.to_string()),
            ))
        }

        SimulateVerify { metadata, message } => {
            crate::query::verify_message(deps.as_ref(), metadata, message)?;

            Ok(Response::new())
        }
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;
    use IsmQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Ism(msg) => match msg {
            ModuleType {} => to_binary(query::get_module_type()),
            Verify {
                metadata: raw_metadata,
                message: raw_message,
            } => to_binary(query::verify_message(deps, raw_metadata, raw_message)),
            ModulesAndThreshold {
                message: raw_message,
            } => to_binary(query::modules_and_threshold(deps, raw_message)),
        },
        QueryMsg::MultisigIsm(msg) => match msg {
            MultisigIsmQueryMsg::EnrolledValidators { domain } => to_binary({
                let validators = VALIDATORS.load(deps.storage, domain)?;
                let threshold = THRESHOLD.load(deps.storage, domain)?;

                Ok::<_, ContractError>(EnrolledValidatorsResponse {
                    validators,
                    threshold,
                })
            }),
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    Ok(Response::new())
}
