use crate::error::ContractError;
use bech32::ToBase32;
use cosmwasm_std::HexBinary;
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

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
