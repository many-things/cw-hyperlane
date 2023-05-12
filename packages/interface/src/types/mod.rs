use cosmwasm_std::Binary;
use sha3::{Digest, Keccak256};

pub mod message;

pub fn keccak256_hash(bz: &[u8]) -> Binary {
    let mut hasher = Keccak256::new();
    hasher.update(bz);
    let hash = hasher.finalize().to_vec();

    Binary(hash)
}
