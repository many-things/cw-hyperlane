use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub chain_hpl: String,
}

#[cw_serde]
pub struct ValidatorSet {
    pub signer: Addr,
    pub signer_pubkey: Binary,
}

#[cw_serde]
pub struct Validators(pub Vec<ValidatorSet>);

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const PENDING_OWNER_KEY: &str = "pending_owner";
pub const PENDING_OWNER: Item<Addr> = Item::new(PENDING_OWNER_KEY);

pub const VALIDATORS_PREFIX: &str = "validators";
pub const VALIDATORS: Map<u64, Validators> = Map::new(VALIDATORS_PREFIX);
