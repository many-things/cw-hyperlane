use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response, StdResult, Uint256,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use hpl_interface::hook::{HookQueryMsg, PostDispatchMsg, QuoteDispatchResponse};

use crate::{CONTRACT_NAME, CONTRACT_VERSION};

const DEFAULT_GAS: u128 = 500_000;
const GAS_KEY: &str = "gas";
const GAS: Item<Uint256> = Item::new(GAS_KEY);

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SetGasAmount { gas: Uint256 },
    PostDispatch(PostDispatchMsg),
}

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_mock_hook::{name}"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    GAS.save(deps.storage, &Uint256::from_u128(DEFAULT_GAS))?;

    Ok(Response::new().add_event(new_event("instantiate")))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::PostDispatch(PostDispatchMsg { metadata, message }) => Ok(Response::new()
            .add_event(
                new_event("post-dispatch")
                    .add_attribute("gas", GAS.load(deps.storage)?)
                    .add_attribute("sender", info.sender)
                    .add_attribute("message", message.to_string())
                    .add_attribute(
                        "metadata",
                        if metadata.is_empty() {
                            "0x".to_string()
                        } else {
                            metadata.to_string()
                        },
                    ),
            )),
        ExecuteMsg::SetGasAmount { gas } => {
            GAS.save(deps.storage, &gas)?;

            Ok(Response::new())
        }
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: HookQueryMsg) -> StdResult<QueryResponse> {
    match msg {
        HookQueryMsg::QuoteDispatch(_) => {
            let gas = GAS.load(_deps.storage)?;
            Ok(to_binary(&QuoteDispatchResponse { gas_amount: gas })?)
        }
        HookQueryMsg::Mailbox {} => unimplemented!("mailbox query not implemented on mock hook"),
    }
}
