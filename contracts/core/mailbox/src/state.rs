use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::ContractError;

#[cw_serde]
pub struct Config {
    pub hrp: String,
    pub local_domain: u32,
    pub default_ism: Option<Addr>,
    pub default_hook: Option<Addr>,
}

impl Config {
    pub fn get_default_ism(&self) -> Addr {
        self.default_ism.clone().expect("default_ism not set")
    }

    pub fn get_default_hook(&self) -> Addr {
        self.default_hook.clone().expect("default_hook not set")
    }
}

#[cw_serde]
pub struct Delivery {
    pub sender: Addr,
    // uint48 value?
    // uint48 timestamp?
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const NONCE_KEY: &str = "nonce";
pub const NONCE: Item<u32> = Item::new(NONCE_KEY);

pub const LATEST_DISPATCHED_ID_KEY: &str = "latest_dispatched_id";
pub const LATEST_DISPATCHED_ID: Item<Vec<u8>> = Item::new(LATEST_DISPATCHED_ID_KEY);

pub const DELIVERIES_PREFIX: &str = "deliveries";
pub const DELIVERIES: Map<Vec<u8>, Delivery> = Map::new(DELIVERIES_PREFIX);

pub fn assert_verify_response(resp: bool) -> Result<(), ContractError> {
    if !resp {
        return Err(ContractError::VerifyFailed {});
    }

    Ok(())
}
