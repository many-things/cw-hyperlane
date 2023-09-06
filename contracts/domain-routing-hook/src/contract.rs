#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use hpl_interface::domain_routing_hook::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::{
    query::{get_owner_info, get_pause_info, quote_dispatch},
    state::{MAILBOX, PAUSE},
    ContractError, CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    hpl_ownable::OWNER.save(deps.storage, &deps.api.addr_validate(&msg.owner)?)?;
    MAILBOX.save(deps.storage, &deps.api.addr_validate(&msg.mailbox)?)?;
    PAUSE.save(deps.storage, &false)?;

    Ok(Response::new())
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
    use crate::execute::{dispatch, gov, hook};

    match msg {
        ExecuteMsg::Ownership(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::Pause {} => gov::pause(deps, env, info),
        ExecuteMsg::Unpause {} => gov::unpause(deps, env, info),
        ExecuteMsg::UpdateMailbox { mailbox } => gov::update_mailbox(deps, env, info, mailbox),
        ExecuteMsg::SetHook { destination, hook } => {
            hook::set_hook(deps, env, info, destination, hook)
        }
        ExecuteMsg::SetHooks { hooks } => hook::set_hooks(deps, env, info, hooks),
        ExecuteMsg::PostDispatch { metadata, message } => {
            dispatch::dispatch(deps, env, info, metadata, message)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::QuoteDispatch(msg) => quote_dispatch(deps, env, msg),
        QueryMsg::PauseInfo {} => get_pause_info(deps),
        QueryMsg::Owner {} => get_owner_info(deps),
    }
}
