#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response};

use hpl_interface::hook::HookQueryMsg;
use hpl_interface::igp::core::{ExecuteMsg, IgpQueryMsg, InstantiateMsg, QueryMsg};
use hpl_interface::igp::gas_oracle::IgpGasOracleQueryMsg;

use crate::state::{Config, CONFIG};
use crate::{
    state::{BENEFICIARY, GAS_TOKEN},
    ContractError, CONTRACT_NAME, CONTRACT_VERSION,
};

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_igp_core::{}", name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    hpl_ownable::initialize(deps.storage, &deps.api.addr_validate(&msg.owner)?)?;

    BENEFICIARY.save(deps.storage, &deps.api.addr_validate(&msg.beneficiary)?)?;
    GAS_TOKEN.save(deps.storage, &msg.gas_token)?;
    CONFIG.save(deps.storage, &Config { prefix: msg.prefix })?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", msg.owner)
            .add_attribute("beneficiary", msg.beneficiary),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute;

    match msg {
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::Router(msg) => Ok(hpl_router::handle(deps, env, info, msg)?),
        ExecuteMsg::PostDispatch(msg) => Ok(execute::post_dispatch(deps, info, msg)?),

        ExecuteMsg::SetBeneficiary { beneficiary } => {
            execute::set_beneficiary(deps, info, beneficiary)
        }
        ExecuteMsg::Claim {} => execute::claim(deps, env, info),

        ExecuteMsg::PayForGas {
            message_id,
            dest_domain,
            gas_amount,
            refund_address,
        } => execute::pay_for_gas(
            deps,
            info,
            message_id,
            dest_domain,
            gas_amount,
            refund_address,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Router(msg) => Ok(hpl_router::handle_query(deps, env, msg)?),

        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::QuoteDispatch(msg) => quote_dispatch(deps, msg),
            HookQueryMsg::Mailbox {} => todo!(),
        },
        QueryMsg::Oracle(msg) => match msg {
            IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { dest_domain } => {
                get_exchange_rate_and_gas_price(deps, dest_domain)
            }
        },
        QueryMsg::Base(msg) => match msg {
            IgpQueryMsg::Beneficiary {} => todo!(),
            IgpQueryMsg::QuoteGasPayment {
                dest_domain,
                gas_amount,
            } => quote_gas_payment(deps, dest_domain, gas_amount),
        },
    }
}
