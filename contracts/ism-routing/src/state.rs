use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const MODULES_PREFIX: &str = "modules";
pub const MODULES: Map<u32, Addr> = Map::new(MODULES_PREFIX);
