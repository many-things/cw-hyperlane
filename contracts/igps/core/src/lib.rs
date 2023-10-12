pub mod contract;
mod error;
mod event;
pub mod execute;
pub mod query;
mod state;
#[cfg(test)]
pub mod tests;

pub use error::ContractError;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// constants
pub const TOKEN_EXCHANGE_RATE_SCALE: u128 = 10_000_000_000;
pub const DEFAULT_GAS_USAGE: u64 = 25_000;
