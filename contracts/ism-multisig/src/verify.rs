use crate::error::ContractError;
use bech32::ToBase32;
use cosmwasm_std::Binary;
use k256::ecdsa::VerifyingKey;
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

pub fn pub_to_addr(pub_key: Binary, prefix: &str) -> Result<String, ContractError> {
    let sha_hash = sha256_digest(pub_key)?;
    let rip_hash = ripemd160_digest(sha_hash)?;

    let addr = bech32::encode(prefix, rip_hash.to_base32(), bech32::Variant::Bech32)
        .map_err(|_| ContractError::InvalidPubKey {})?;

    Ok(addr)
}

pub fn uncompress_pubkey(pub_key_binary: Binary) -> Result<Binary, ContractError> {
    let pub_key = VerifyingKey::from_sec1_bytes(&pub_key_binary).unwrap();

    Ok(pub_key.to_encoded_point(false).as_bytes().into())
}
