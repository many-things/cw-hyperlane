use cosmwasm_std::HexBinary;
use cw_storage_plus::Map;

pub const VALIDATORS_PREFIX: &str = "validators";
pub const VALIDATORS: Map<u32, Vec<HexBinary>> = Map::new(VALIDATORS_PREFIX);

pub const THRESHOLD_PREFIX: &str = "threshold";
pub const THRESHOLD: Map<u32, u8> = Map::new(THRESHOLD_PREFIX);
