use bech32::ToBase32;
use cosmwasm_std::HexBinary;
use error::ContractError;
use hpl_interface::types::keccak256_hash;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub mod contract;
pub mod contract_querier;
pub mod error;
pub mod state;

#[cfg(test)]
mod test;

const PREFIX: &str = "\x19Ethereum Signed Message:\n";

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn eth_hash(message: HexBinary) -> Result<HexBinary, ContractError> {
    let mut eth_message = format!("{PREFIX}{}", message.len()).into_bytes();
    eth_message.extend_from_slice(&message);
    let message_hash = keccak256_hash(&eth_message);

    Ok(message_hash)
}

pub fn sha256_digest(bz: impl AsRef<[u8]>) -> Result<[u8; 32], ContractError> {
    let mut hasher = Sha256::new();

    hasher.update(bz);

    hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongLength {})
}

pub fn ripemd160_digest(bz: impl AsRef<[u8]>) -> Result<[u8; 20], ContractError> {
    let mut hasher = Ripemd160::new();

    hasher.update(bz);

    hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongLength {})
}

pub fn pub_to_addr(pub_key: HexBinary, prefix: &str) -> Result<String, ContractError> {
    let sha_hash = sha256_digest(pub_key)?;
    let rip_hash = ripemd160_digest(sha_hash)?;

    let addr = bech32::encode(prefix, rip_hash.to_base32(), bech32::Variant::Bech32)
        .map_err(|_| ContractError::InvalidPubKey {})?;

    Ok(addr)
}

pub fn pub_to_addr_binary(pub_key: HexBinary) -> Result<HexBinary, ContractError> {
    let sha_hash = sha256_digest(pub_key)?;
    let rip_hash = ripemd160_digest(sha_hash)?;

    Ok(HexBinary::from(rip_hash.to_vec()))
}
