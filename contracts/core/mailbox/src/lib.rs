#[cfg(not(feature = "library"))]
mod contract;
mod contract_querier;
mod error;
mod event;
pub mod execute;
pub mod query;
mod state;

pub use crate::error::ContractError;

pub const MAILBOX_VERSION: u8 = 0;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
