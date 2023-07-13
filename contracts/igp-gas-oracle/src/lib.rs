pub mod contract;
pub mod error;
pub mod state;

#[cfg(test)]
pub mod contract_test;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
