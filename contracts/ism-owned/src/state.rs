use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub hpl: String,
    pub owner: Addr,
    pub owner_pubkey: Binary,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);
