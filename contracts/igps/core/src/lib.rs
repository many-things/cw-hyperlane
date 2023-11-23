pub mod contract;
mod error;
mod event;
pub mod execute;
pub mod query;

#[cfg(test)]
pub mod tests;

use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};
pub use error::ContractError;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// constants
pub const TOKEN_EXCHANGE_RATE_SCALE: u128 = 10_000_000_000;

pub const HRP_KEY: &str = "hrp";
pub const HRP: Item<String> = Item::new(HRP_KEY);

pub const GAS_TOKEN_KEY: &str = "gas_token";
pub const GAS_TOKEN: Item<String> = Item::new(GAS_TOKEN_KEY);

pub const DEFAULT_GAS_USAGE_KEY: &str = "default_gas_usage";
pub const DEFAULT_GAS_USAGE: Item<u128> = Item::new(DEFAULT_GAS_USAGE_KEY);

pub const GAS_FOR_DOMAIN_PREFIX: &str = "gas_for_domain";
pub const GAS_FOR_DOMAIN: Map<u32, u128> = Map::new(GAS_FOR_DOMAIN_PREFIX);

pub const BENEFICIARY_KEY: &str = "beneficiary";
pub const BENEFICIARY: Item<Addr> = Item::new(BENEFICIARY_KEY);

pub fn get_default_gas(storage: &dyn Storage, domain: u32) -> StdResult<u128> {
    let custom_gas = GAS_FOR_DOMAIN.may_load(storage, domain)?;
    let default_gas = DEFAULT_GAS_USAGE.load(storage)?;

    Ok(custom_gas.unwrap_or(default_gas))
}
