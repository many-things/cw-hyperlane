use cosmwasm_std::{HexBinary, StdError, StdResult};
use ripemd::Ripemd160;
use sha2::Sha256;
use sha3::Keccak256;

const PREFIX: &str = "\x19Ethereum Signed Message:\n";

pub fn keccak256_hash(bz: &[u8]) -> HexBinary {
    use sha3::Digest;

    let mut hasher = Keccak256::new();
    hasher.update(bz);
    let hash = hasher.finalize().to_vec();

    hash.into()
}

pub fn eth_hash(message: HexBinary) -> StdResult<HexBinary> {
    let mut eth_message = format!("{PREFIX}{}", message.len()).into_bytes();
    eth_message.extend_from_slice(&message);
    let message_hash = keccak256_hash(&eth_message);

    Ok(message_hash)
}

pub fn sha256_digest(bz: impl AsRef<[u8]>) -> StdResult<[u8; 32]> {
    use sha2::Digest;

    let mut hasher = Sha256::new();

    hasher.update(bz);

    hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|_| StdError::generic_err("wrong length"))
}

pub fn ripemd160_digest(bz: impl AsRef<[u8]>) -> StdResult<[u8; 20]> {
    use ripemd::Digest;

    let mut hasher = Ripemd160::new();

    hasher.update(bz);

    hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|_| StdError::generic_err("wrong length"))
}

pub fn pub_to_addr(pub_key: HexBinary) -> StdResult<HexBinary> {
    let sha_hash = sha256_digest(pub_key)?;
    let rip_hash = ripemd160_digest(sha_hash)?;

    Ok(rip_hash.to_vec().into())
}
