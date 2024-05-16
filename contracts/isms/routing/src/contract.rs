#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, to_json_binary, Deps, DepsMut, Empty, Env, MessageInfo, QueryResponse, Response,
};
use cw2::set_contract_version;
use hpl_interface::{
    ism::{
        routing::{ExecuteMsg, InstantiateMsg, QueryMsg, RouteResponse, RoutingIsmQueryMsg},
        IsmQueryMsg, ModuleTypeResponse, VerifyResponse,
    },
    types::Message,
};
use hpl_ownable::get_owner;

use crate::{error::ContractError, new_event, state::MODULES, CONTRACT_NAME, CONTRACT_VERSION};

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

    for ism in msg.isms {
        MODULES.save(
            deps.storage,
            ism.domain,
            &deps.api.addr_validate(&ism.address)?,
        )?;
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner))
}

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
        Set { ism } => {
            ensure_eq!(
                get_owner(deps.storage)?,
                info.sender,
                ContractError::Unauthorized {}
            );

            MODULES.save(
                deps.storage,
                ism.domain,
                &deps.api.addr_validate(&ism.address)?,
            )?;

            Ok(Response::default().add_event(
                new_event("set")
                    .add_attribute("sender", info.sender)
                    .add_attribute("domain", ism.domain.to_string())
                    .add_attribute("ism", ism.address),
            ))
        }
        Unset { domains } => {
            ensure_eq!(
                get_owner(deps.storage)?,
                info.sender,
                ContractError::Unauthorized {}
            );

            for domain in domains.clone() {
                MODULES.remove(deps.storage, domain);
            }

            Ok(Response::default().add_event(
                new_event("unset")
                    .add_attribute("sender", info.sender)
                    .add_attribute(
                        "domains",
                        domains
                            .into_iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use IsmQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Ism(msg) => match msg {
            ModuleType {} => Ok(to_json_binary(&ModuleTypeResponse {
                typ: hpl_interface::ism::IsmType::Routing,
            })?),
            Verify { metadata, message } => {
                let decoded = Message::from(message.clone());

                let ism = MODULES
                    .may_load(deps.storage, decoded.origin_domain)?
                    .ok_or(ContractError::RouteNotFound {})?;

                let verify_resp: VerifyResponse = deps
                    .querier
                    .query_wasm_smart(ism, &IsmQueryMsg::Verify { metadata, message }.wrap())?;

                Ok(to_json_binary(&verify_resp)?)
            }
            VerifyInfo { message } => {
                let decoded = Message::from(message.clone());

                let ism = MODULES
                    .may_load(deps.storage, decoded.origin_domain)?
                    .ok_or(ContractError::RouteNotFound {})?;

                let verify_resp: VerifyResponse = deps
                    .querier
                    .query_wasm_smart(ism, &IsmQueryMsg::VerifyInfo { message })?;

                Ok(to_json_binary(&verify_resp)?)
            }
        },
        QueryMsg::RoutingIsm(msg) => match msg {
            RoutingIsmQueryMsg::Route { message } => {
                let decoded = Message::from(message);

                let ism = MODULES
                    .may_load(deps.storage, decoded.origin_domain)?
                    .ok_or(ContractError::RouteNotFound {})?
                    .to_string();

                Ok(to_json_binary(&RouteResponse { ism })?)
            }
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
