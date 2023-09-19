use cosmwasm_std::{ensure, ensure_eq, DepsMut, HexBinary, MessageInfo, Response};
use hpl_interface::{domain_routing_hook::HookConfig, types::keccak256_hash};

use crate::{
    event::{emit_config_custom_hook, emit_set_hook, emit_set_hooks},
    state::{CUSTOM_HOOK_CONFIG, HOOK_CONFIG, PAUSE},
    ContractError,
};

pub fn set_hook(
    deps: DepsMut,
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

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr, Storage,
    };

    use super::*;
    const ADDR1_VALUE: &str = "addr1";
    const ADDR2_VALUE: &str = "addr2";

    fn mock_owner(storage: &mut dyn Storage, owner: &str) {
        hpl_ownable::OWNER
            .save(storage, &Addr::unchecked(owner))
            .unwrap();
    }

    #[test]
    fn test_set_hook() {
        let mut deps = mock_dependencies();
        mock_owner(deps.as_mut().storage, ADDR1_VALUE);

        let destination = 11155111;
        let hook = Addr::unchecked("osmo109ns4u04l44kqdkvp876hukd3hxz8zzm7809el");

        let unauthorized = set_hook(
            deps.as_mut(),
            mock_info(ADDR2_VALUE, &[]),
            destination,
            hook.to_string(),
        )
        .unwrap_err();
        assert!(matches!(unauthorized, ContractError::Unauthorized {}));

        // already paused
        PAUSE.save(deps.as_mut().storage, &true).unwrap();
        let already_paused = set_hook(
            deps.as_mut(),
            mock_info(ADDR1_VALUE, &[]),
            destination,
            hook.to_string(),
        )
        .unwrap_err();
        assert!(matches!(already_paused, ContractError::Paused {}));

        // success hook
        PAUSE.save(deps.as_mut().storage, &false).unwrap();

        let res = set_hook(
            deps.as_mut(),
            mock_info(ADDR1_VALUE, &[]),
            destination,
            hook.to_string(),
        )
        .unwrap();

        assert_eq!(
            HOOK_CONFIG
                .load(deps.as_ref().storage, destination)
                .unwrap(),
            HookConfig {
                destination,
                hook: hook.clone(),
            }
        );
        assert_eq!(
            res,
            Response::new().add_event(emit_set_hook(destination, hook))
        );
    }

    #[test]
    fn test_set_hooks() {
        let mut deps = mock_dependencies();
        mock_owner(deps.as_mut().storage, ADDR1_VALUE);

        let hooks = vec![
            HookConfig {
                destination: 5,
                hook: Addr::unchecked("osmo109ns4u04l44kqdkvp876hukd3hxz8zzm7809el"),
            },
            HookConfig {
                destination: 11155111,
                hook: Addr::unchecked("osmo1mhnkm6fwaq53yzu7c0r3khhy60v04vse4c6gk5"),
            },
        ];

        let unauthorized =
            set_hooks(deps.as_mut(), mock_info(ADDR2_VALUE, &[]), hooks.clone()).unwrap_err();
        assert!(matches!(unauthorized, ContractError::Unauthorized {}));

        // already paused
        PAUSE.save(deps.as_mut().storage, &true).unwrap();
        let already_paused =
            set_hooks(deps.as_mut(), mock_info(ADDR1_VALUE, &[]), hooks.clone()).unwrap_err();
        assert!(matches!(already_paused, ContractError::Paused {}));

        // success hook
        PAUSE.save(deps.as_mut().storage, &false).unwrap();

        let res = set_hooks(deps.as_mut(), mock_info(ADDR1_VALUE, &[]), hooks.clone()).unwrap();

        assert_eq!(
            HOOK_CONFIG
                .load(deps.as_ref().storage, hooks[0].destination)
                .unwrap(),
            hooks[0]
        );
        assert_eq!(
            HOOK_CONFIG
                .load(deps.as_ref().storage, hooks[1].destination)
                .unwrap(),
            hooks[1]
        );
        assert_eq!(res, Response::new().add_event(emit_set_hooks(hooks)));
    }
}
