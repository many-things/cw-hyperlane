use cosmwasm_std::{ensure, ensure_eq, DepsMut, Env, HexBinary, MessageInfo, Response};
use hpl_interface::{domain_routing_hook::HookConfig, types::keccak256_hash};

use crate::{
    event::{emit_config_custom_hook, emit_set_hook, emit_set_hooks},
    state::{CUSTOM_HOOK_CONFIG, HOOK_CONFIG, PAUSE},
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

pub fn config_custom_hook(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    destination: u32,
    recipient: HexBinary,
    hook: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::OWNER.load(deps.storage)?,
        ContractError::Unauthorized {}
    );
    ensure!(!PAUSE.load(deps.storage)?, ContractError::Paused {});

    let hook_key = keccak256_hash(
        destination
            .to_be_bytes()
            .iter()
            .chain(recipient.as_slice().iter())
            .cloned()
            .collect::<Vec<u8>>()
            .as_slice(),
    );
    let hook_addr = deps.api.addr_validate(&hook)?;

    CUSTOM_HOOK_CONFIG.save(
        deps.storage,
        hook_key.to_vec(),
        &HookConfig {
            destination,
            hook: hook_addr.clone(),
        },
    )?;
    Ok(Response::new().add_event(emit_config_custom_hook(destination, recipient, hook_addr)))
}
