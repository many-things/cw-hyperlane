pub mod contract;
pub mod error;
pub mod msg;
pub mod proto;
pub mod serde;
pub mod state;

#[cfg(test)]
mod tests;

// reply message
pub const REPLY_ID_CREATE_DENOM: u64 = 0;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
