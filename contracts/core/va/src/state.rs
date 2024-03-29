use cosmwasm_std::Empty;
use cw_storage_plus::{Item, Map};

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Vec<u8>> = Item::new(MAILBOX_KEY);

pub const LOCAL_DOMAIN_KEY: &str = "local_domain";
pub const LOCAL_DOMAIN: Item<u32> = Item::new(LOCAL_DOMAIN_KEY);

pub const VALIDATORS_PREFIX: &str = "validators";
pub const VALIDATORS: Map<Vec<u8>, Empty> = Map::new(VALIDATORS_PREFIX);

pub const STORAGE_LOCATIONS_PREFIX: &str = "storage_locations";
pub const STORAGE_LOCATIONS: Map<Vec<u8>, Vec<String>> = Map::new(STORAGE_LOCATIONS_PREFIX);

pub const REPLAY_PROTECTIONS_PREFIX: &str = "replay_protections";
pub const REPLAY_PROTECITONS: Map<Vec<u8>, Empty> = Map::new(REPLAY_PROTECTIONS_PREFIX);
