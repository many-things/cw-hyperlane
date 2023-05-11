#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use hpl_interface::{
    mailbox,
    mailbox_factory::{ExecuteMsg, InstantiateMsg, MigrateMsg, OriginDomainResponse, QueryMsg},
};

use crate::state::{MAILBOX_CODE, ORIGIN_DOMAIN};
use crate::{error::ContractError, CONTRACT_NAME, CONTRACT_VERSION};

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ORIGIN_DOMAIN.save(deps.storage, &msg.origin_domain)?;
    MAILBOX_CODE.save(deps.storage, &msg.mailbox_code)?;

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Instantiate { owner, default_ism } => {
            deps.api.addr_validate(&owner)?;
            deps.api.addr_validate(&default_ism)?;

            let mailbox_code = MAILBOX_CODE.load(deps.storage)?;

            let resp = Response::new().add_message(WasmMsg::Instantiate {
                admin: Some(env.contract.address.into_string()),
                code_id: mailbox_code,
                msg: to_binary(&mailbox::InstantiateMsg { owner, default_ism })?,
                funds: vec![],
                label: "hyperlane-mailbox".to_string(),
            });

            Ok(resp)
        }
        ExecuteMsg::Migrate {} => todo!(),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::OriginDomain => {
            to_binary(&OriginDomainResponse(ORIGIN_DOMAIN.load(deps.storage)?))
        }
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
