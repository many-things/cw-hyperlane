pub mod contract;
mod error;
mod migration;
pub mod query;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Event, HexBinary, StdResult};
use cw_storage_plus::Map;
use hpl_interface::types::keccak256_hash;

pub use crate::error::ContractError;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct ValidatorSet {
    pub validators: Vec<HexBinary>,
    pub threshold: u8,
}

pub const VALIDATORS_PREFIX: &str = "validators";
pub const VALIDATORS: Map<u32, ValidatorSet> = Map::new(VALIDATORS_PREFIX);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_ism_multisig::{name}"))
}

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
