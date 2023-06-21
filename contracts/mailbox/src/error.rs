use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Contract is paused")]
    Paused {},

    #[error("ISM verify is failed")]
    VerifyFailed {},

    #[error("Invalid address length: {len:?}")]
    InvalidAddressLength { len: usize },

    #[error("Invalid message version: {version:?}")]
    InvalidMessageVersion { version: u8 },

    #[error("Invalid destination domain: {domain:?}")]
    InvalidDestinationDomain { domain: u32 },

    #[error("Already delivered message")]
    AlreadyDeliveredMessage {},

    #[error("Merkle tree is already full")]
    MerkleTreeIsFull {},
}
