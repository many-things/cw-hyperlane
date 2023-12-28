pub mod contract;
mod error;
pub mod state;

#[cfg(test)]
mod tests;

use cosmwasm_std::Event;

pub use crate::error::ContractError;

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_ism_routing::{}", name))
}

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
