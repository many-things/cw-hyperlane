use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::merkle::MerkleTree;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub factory: Addr,
    pub default_ism: Addr,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const PAUSE_KEY: &str = "pause";
pub const PAUSE: Item<bool> = Item::new(PAUSE_KEY);

pub const NONCE_KEY: &str = "nonce";
pub const NONCE: Item<u32> = Item::new(NONCE_KEY);

pub const MESSAGE_TREE_KEY: &str = "message_tree";
pub const MESSAGE_TREE: Item<MerkleTree> = Item::new(MESSAGE_TREE_KEY);

pub const MESSAGE_PROCESSED_PREFIX: &str = "message_processed";
pub const MESSAGE_PROCESSED: Map<Vec<u8>, bool> = Map::new(MESSAGE_PROCESSED_PREFIX);
