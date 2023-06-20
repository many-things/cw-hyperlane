pub mod contract;
mod error;
pub mod event;
pub mod execute;
pub mod query;
pub mod state;
mod verify;

pub use crate::error::ContractError;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
