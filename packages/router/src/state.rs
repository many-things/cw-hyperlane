use cosmwasm_std::Binary;
use cw_storage_plus::Map;

pub const ROUTES_PREFIX: &str = "routes";
pub const ROUTES: Map<u32, Binary> = Map::new(ROUTES_PREFIX);
