use cosmwasm_std::{Addr, Binary};
use cw_storage_plus::{Item, Map};
use hpl_interface::{hook::HookConfig, types::keccak256_hash};

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);

pub const PENDING_OWNER_KEY: &str = "pending_owner";
pub const PENDING_OWNER: Item<Addr> = Item::new(PENDING_OWNER_KEY);

pub const HOOK_CONFIG_KEY: &str = "hook_config";
pub const HOOK_CONFIG: Map<u32, HookConfig> = Map::new(HOOK_CONFIG_KEY);

pub const CUSTOM_HOOK_CONFIG_KEY: &str = "custom_hook_config";
pub const CUSTOM_HOOK_CONFIG: Map<Vec<u8>, HookConfig> = Map::new(CUSTOM_HOOK_CONFIG_KEY);

pub const PAUSE_KEY: &str = "pause";
pub const PAUSE: Item<bool> = Item::new(PAUSE_KEY);

pub fn generate_hook_key(destination: u32, recipient: Binary) -> Vec<u8> {
    keccak256_hash(
        destination
            .to_be_bytes()
            .iter()
            .chain(recipient.as_slice().iter())
            .cloned()
            .collect::<Vec<u8>>()
            .as_slice(),
    )
    .to_vec()
}
