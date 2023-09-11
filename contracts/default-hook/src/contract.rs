#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use hpl_interface::default_hook::ExecuteMsg;
use hpl_interface::domain_routing_hook::{
    ExecuteMsg as DomainRoutingExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};

use crate::{
    event::emit_instantiated,
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

    let owner = deps.api.addr_validate(&msg.owner)?;
    let mailbox = deps.api.addr_validate(&msg.mailbox)?;
    hpl_ownable::OWNER.save(deps.storage, &owner)?;
    MAILBOX.save(deps.storage, &mailbox)?;
    PAUSE.save(deps.storage, &false)?;

    Ok(Response::new().add_event(emit_instantiated(owner, mailbox)))
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
        ExecuteMsg::DomainRoutingHookMsg(msg) => match msg {
            DomainRoutingExecuteMsg::Ownership(msg) => {
                Ok(hpl_ownable::handle(deps, env, info, msg)?)
            }
            DomainRoutingExecuteMsg::Pause {} => gov::pause(deps, info),
            DomainRoutingExecuteMsg::Unpause {} => gov::unpause(deps, info),
            DomainRoutingExecuteMsg::UpdateMailbox { mailbox } => {
                gov::update_mailbox(deps, info, mailbox)
            }
            DomainRoutingExecuteMsg::SetHook { destination, hook } => {
                hook::set_hook(deps, info, destination, hook)
            }
            DomainRoutingExecuteMsg::SetHooks { hooks } => hook::set_hooks(deps, info, hooks),
            DomainRoutingExecuteMsg::PostDispatch { metadata, message } => {
                dispatch::dispatch(deps, metadata, message)
            }
        },
        ExecuteMsg::ConfigCustomHook {
            destination_domain,
            recipient,
            hook,
        } => hook::config_custom_hook(deps, info, destination_domain, recipient, hook),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::QuoteDispatch(dispatch_msg) => quote_dispatch(deps, env, dispatch_msg),
        QueryMsg::PauseInfo {} => get_pause_info(deps),
        QueryMsg::Owner {} => get_owner_info(deps),
    }
}
