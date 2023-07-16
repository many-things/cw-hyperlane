pub mod contract;
mod core;
mod error;
mod event;
mod gov;
pub mod merkle;
mod query;
pub mod state;
pub mod verify;

pub use crate::error::ContractError;

pub const MAILBOX_VERSION: u8 = 0;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
