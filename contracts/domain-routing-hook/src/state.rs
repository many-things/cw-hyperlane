use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use hpl_interface::hook::HookConfig;

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);

pub const PENDING_OWNER_KEY: &str = "pending_owner";
pub const PENDING_OWNER: Item<Addr> = Item::new(PENDING_OWNER_KEY);

pub const HOOK_CONFIG_KEY: &str = "hook_config";
pub const HOOK_CONFIG: Map<u32, HookConfig> = Map::new(HOOK_CONFIG_KEY);

pub const PAUSE_KEY: &str = "pause";
pub const PAUSE: Item<bool> = Item::new(PAUSE_KEY);
