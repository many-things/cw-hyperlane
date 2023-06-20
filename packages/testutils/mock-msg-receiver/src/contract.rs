use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response, StdResult,
};
use cw2::set_contract_version;
use hpl_interface::{ism, mailbox, types::bech32_encode};

use crate::{CONTRACT_NAME, CONTRACT_VERSION};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ExecuteMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: mailbox::ExpectedHandlerMsg,
) -> StdResult<Response> {
    match msg {
        mailbox::ExpectedHandlerMsg::Handle(msg) => Ok(Response::default().add_event(
            Event::new("mailbox_msg_received").add_attributes(vec![
                attr("sender", bech32_encode("osmo", &msg.sender)?),
                attr("origin", msg.origin.to_string()),
                attr("body", std::str::from_utf8(&msg.body)?),
            ]),
        )),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: ism::ISMSpecifierQueryMsg) -> StdResult<QueryResponse> {
    match msg {
        ism::ISMSpecifierQueryMsg::InterchainSecurityModule() => {
            Ok(to_binary(&ism::InterchainSecurityModuleResponse {
                ism: None,
            })?)
        }
    }
}
