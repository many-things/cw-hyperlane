use cosmwasm_std::HexBinary;
use sha3::{Digest, Keccak256};

pub fn keccak256_hash(bz: &[u8]) -> HexBinary {
    let mut hasher = Keccak256::new();
    hasher.update(bz);
    let hash = hasher.finalize().to_vec();

    hash.into()
}
