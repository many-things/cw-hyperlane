use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary};
use cw_storage_plus::{Item, Map};

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

pub const HRP_KEY: &str = "hrp";
pub const HRP: Item<String> = Item::new(HRP_KEY);

pub const VALIDATORS_PREFIX: &str = "validators";
pub const VALIDATORS: Map<u32, Validators> = Map::new(VALIDATORS_PREFIX);

pub const THRESHOLD_PREFIX: &str = "threshold";
pub const THRESHOLD: Map<u32, u8> = Map::new(THRESHOLD_PREFIX);
