use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary};
use cw_storage_plus::Map;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub addr_prefix: String,
}

pub const VALIDATORS_PREFIX: &str = "validators";
pub const VALIDATORS: Map<u32, Vec<HexBinary>> = Map::new(VALIDATORS_PREFIX);

pub const THRESHOLD_PREFIX: &str = "threshold";
pub const THRESHOLD: Map<u32, u8> = Map::new(THRESHOLD_PREFIX);
