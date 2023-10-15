pub mod contract;
mod error;
mod event;
pub mod execute;
pub mod query;

#[cfg(test)]
pub mod tests;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
pub use error::ContractError;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// constants
pub const TOKEN_EXCHANGE_RATE_SCALE: u128 = 10_000_000_000;
pub const DEFAULT_GAS_USAGE: u64 = 25_000;

pub const HRP_KEY: &str = "hrp";
pub const HRP: Item<String> = Item::new(HRP_KEY);

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);

pub const GAS_TOKEN_KEY: &str = "gas_token";
pub const GAS_TOKEN: Item<String> = Item::new(GAS_TOKEN_KEY);

pub const BENEFICAIRY_KEY: &str = "beneficiary";
pub const BENEFICIARY: Item<Addr> = Item::new(BENEFICAIRY_KEY);
