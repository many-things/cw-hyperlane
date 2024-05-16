use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_json_binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, QueryResponse, Response,
    StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use hpl_interface::{core::ExpectedHandleMsg, ism, types::bech32_encode};

use crate::{CONTRACT_NAME, CONTRACT_VERSION};

#[cw_serde]
pub struct InstantiateMsg {
    pub hrp: String,
}

#[cw_serde]
pub struct ExecuteMsg {}

pub const HRP_KEY: &str = "hrp";
pub const HRP: Item<String> = Item::new(HRP_KEY);

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    HRP.save(deps.storage, &msg.hrp)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExpectedHandleMsg,
) -> StdResult<Response> {
    match msg {
        ExpectedHandleMsg::Handle(msg) => Ok(Response::default().add_event(
            Event::new("mailbox_msg_received").add_attributes(vec![
                attr(
                    "sender",
                    bech32_encode(&HRP.load(deps.storage)?, &msg.sender)?,
                ),
                attr("origin", msg.origin.to_string()),
                attr("body", std::str::from_utf8(&msg.body)?),
            ]),
        )),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    _deps: Deps,
    _env: Env,
    msg: ism::ExpectedIsmSpecifierQueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        ism::ExpectedIsmSpecifierQueryMsg::IsmSpecifier(
            ism::IsmSpecifierQueryMsg::InterchainSecurityModule(),
        ) => Ok(to_json_binary(&ism::InterchainSecurityModuleResponse {
            ism: None,
        })?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    Ok(Response::default())
}
