use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Storage};
use cw_storage_plus::{Item, Map};

use crate::merkle::MerkleTree;

use crate::ContractError;

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

pub fn assert_owner(owner: &Addr, sender: &Addr) -> Result<(), ContractError> {
    if owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn assert_paused(storage: &dyn Storage) -> Result<(), ContractError> {
    if !PAUSE.load(storage)? {
        return Err(ContractError::Paused {});
    }

    Ok(())
}

pub fn assert_verify_response(resp: bool) -> Result<(), ContractError> {
    if !resp {
        return Err(ContractError::VerifyFailed {});
    }

    Ok(())
}

pub fn assert_addr_length(len: usize) -> Result<(), ContractError> {
    if len > 32 {
        return Err(ContractError::InvalidAddressLength { len });
    }

    Ok(())
}

pub fn assert_message_version(
    message_version: u8,
    mailbox_version: u8,
) -> Result<(), ContractError> {
    if message_version != mailbox_version {
        return Err(ContractError::InvalidMessageVersion {
            version: message_version,
        });
    }

    Ok(())
}

pub fn assert_destination_domain(
    message_dest_domain: u32,
    origin_domain: u32,
) -> Result<(), ContractError> {
    if message_dest_domain != origin_domain {
        return Err(ContractError::InvalidDestinationDomain {
            domain: message_dest_domain,
        });
    }

    Ok(())
}

pub fn assert_already_delivered(storage: &dyn Storage, id: Binary) -> Result<(), ContractError> {
    if !MESSAGE_PROCESSED.may_load(storage, id.0.clone())?.is_none() {
        return Err(ContractError::AlreadyDeliveredMessage {});
    }

    Ok(())
}

pub fn assert_full_merkle_tree(curr_cnt: u128, max_cnt: u128) -> Result<(), ContractError> {
    if !(curr_cnt < max_cnt) {
        return Err(ContractError::MerkleTreeIsFull {});
    }

    Ok(())
}
