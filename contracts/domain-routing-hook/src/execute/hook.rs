use cosmwasm_std::{ensure, ensure_eq, DepsMut, Env, MessageInfo, Response};
use hpl_interface::domain_routing_hook::HookConfig;

use crate::{
    event::{emit_set_hook, emit_set_hooks},
    state::{HOOK_CONFIG, PAUSE},
    ContractError,
};

pub fn set_hook(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    destination: u32,
    hook_value: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::OWNER.load(deps.storage)?,
        ContractError::Unauthorized {}
    );
    ensure!(!PAUSE.load(deps.storage)?, ContractError::Paused {});

    let hook = deps.api.addr_validate(&hook_value)?;
    HOOK_CONFIG.save(
        deps.storage,
        destination,
        &HookConfig {
            destination,
            hook: hook.clone(),
        },
    )?;

    Ok(Response::new().add_event(emit_set_hook(destination, hook)))
}

pub fn set_hooks(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    hooks: Vec<HookConfig>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::OWNER.load(deps.storage)?,
        ContractError::Unauthorized {}
    );
    ensure!(!PAUSE.load(deps.storage)?, ContractError::Paused {});

    for hook in hooks.iter() {
        HOOK_CONFIG.save(deps.storage, hook.destination, hook)?;
    }

    Ok(Response::new().add_event(emit_set_hooks(hooks)))
}
