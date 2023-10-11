#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ensure_eq, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use hpl_interface::{
    ism::{
        routing::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, RouteResponse},
        ISMQueryMsg, ModuleTypeResponse, VerifyResponse,
    },
    types::message::Message,
};

use crate::{error::ContractError, state::MODULES, CONTRACT_NAME, CONTRACT_VERSION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    hpl_ownable::OWNER.save(deps.storage, &deps.api.addr_validate(&msg.owner)?)?;

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
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
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
        Ownership(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        Set { ism } => {
            ensure_eq!(
                info.sender,
                hpl_ownable::OWNER.load(deps.storage)?,
                ContractError::Unauthorized {}
            );

            MODULES.save(
                deps.storage,
                ism.domain,
                &deps.api.addr_validate(&ism.address)?,
            )?;

            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use QueryMsg::*;

    match msg {
        ModuleType {} => Ok(to_binary(&ModuleTypeResponse {
            typ: hpl_interface::ism::ISMType::Routing,
        })?),
        Verify { metadata, message } => {
            let decoded = Message::from(message.clone());

            let ism = MODULES
                .may_load(deps.storage, decoded.origin_domain)?
                .ok_or(ContractError::RouteNotFound {})?;

            let verify_resp: VerifyResponse = deps
                .querier
                .query_wasm_smart(ism, &ISMQueryMsg::Verify { metadata, message })?;

            Ok(to_binary(&verify_resp)?)
        }
        Route { message } => {
            let decoded = Message::from(message);

            let ism = MODULES
                .may_load(deps.storage, decoded.origin_domain)?
                .ok_or(ContractError::RouteNotFound {})?
                .to_string();

            Ok(to_binary(&RouteResponse { ism })?)
        }
    }
}
