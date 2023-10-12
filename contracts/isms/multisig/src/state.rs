use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary, Storage};
use cw_storage_plus::{Item, Map};

use crate::ContractError;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub addr_prefix: String,
}

#[cw_serde]
pub struct ValidatorSet {
    pub signer: Addr,
    pub signer_pubkey: HexBinary,
}

#[cw_serde]
pub struct Validators(pub Vec<ValidatorSet>);

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const PENDING_OWNER_KEY: &str = "pending_owner";
pub const PENDING_OWNER: Item<Addr> = Item::new(PENDING_OWNER_KEY);

pub const VALIDATORS_PREFIX: &str = "validators";
pub const VALIDATORS: Map<u32, Validators> = Map::new(VALIDATORS_PREFIX);

pub const THRESHOLD_PREFIX: &str = "threshold";
pub const THRESHOLD: Map<u32, u8> = Map::new(THRESHOLD_PREFIX);

pub fn assert_owned(storage: &dyn Storage, sender: Addr) -> Result<(), ContractError> {
    if CONFIG.load(storage)?.owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn assert_pending_owner(storage: &dyn Storage, sender: Addr) -> Result<(), ContractError> {
    let pending_owner = PENDING_OWNER.may_load(storage)?;

    if pending_owner.is_none() {
        return Err(ContractError::OwnershipTransferNotStarted {});
    }

    if PENDING_OWNER.load(storage)? != sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn assert_pending_owner_empty(storage: &dyn Storage) -> Result<(), ContractError> {
    if PENDING_OWNER.may_load(storage)?.is_some() {
        return Err(ContractError::OwnershipTransferAlreadyStarted {});
    }

    Ok(())
}

pub fn assert_pending_owner_exist(storage: &dyn Storage) -> Result<(), ContractError> {
    if PENDING_OWNER.may_load(storage)?.is_none() {
        return Err(ContractError::OwnershipTransferNotStarted {});
    }

    Ok(())
}
