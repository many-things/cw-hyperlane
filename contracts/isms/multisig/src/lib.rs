pub mod contract;
mod error;
pub mod event;
pub mod execute;
pub mod query;
pub mod state;

use cosmwasm_std::{HexBinary, StdResult};
use hpl_interface::types::keccak256_hash;

pub use crate::error::ContractError;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn domain_hash(local_domain: u32, address: HexBinary) -> StdResult<HexBinary> {
    let mut bz = vec![];
    bz.append(&mut local_domain.to_be_bytes().to_vec());
    bz.append(&mut address.to_vec());
    bz.append(&mut "HYPERLANE".as_bytes().to_vec());

    let hash = keccak256_hash(&bz);

    Ok(hash)
}

pub fn multisig_hash(
    mut domain_hash: Vec<u8>,
    mut root: Vec<u8>,
    index: u32,
    mut message_id: Vec<u8>,
) -> Result<HexBinary, ContractError> {
    let mut bz = vec![];

    bz.append(&mut domain_hash);
    bz.append(&mut root);
    bz.append(&mut index.to_be_bytes().to_vec());
    bz.append(&mut message_id);

    let hash = keccak256_hash(&bz);

    Ok(hash)
}
