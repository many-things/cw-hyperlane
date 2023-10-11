use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub prefix: String,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);


pub const GAS_TOKEN_KEY: &str = "gas_token";
pub const GAS_TOKEN: Item<String> = Item::new(GAS_TOKEN_KEY);

pub const BENEFICAIRY_KEY: &str = "beneficiary";
pub const BENEFICIARY: Item<Addr> = Item::new(BENEFICAIRY_KEY);

pub const GAS_ORACLE_PREFIX: &str = "gas_oracle";
pub const GAS_ORACLE: Map<u32, Addr> = Map::new(GAS_ORACLE_PREFIX);
