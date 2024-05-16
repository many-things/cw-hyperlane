use cosmwasm_std::{RecoverPubkeyError, StdError, VerificationError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    VerificationError(#[from] VerificationError),

    #[error("{0}")]
    RecoverPubkeyError(#[from] RecoverPubkeyError),

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("unauthorized")]
    Unauthorized,

    #[error("wrong length")]
    WrongLength,

    #[error("invalid pubkey")]
    InvalidPubKey,

    #[error("invalid address. reason: {0}")]
    InvalidAddress(String),

    #[error("invalid arguments. reason: {reason:?}")]
    InvalidArguments { reason: String },

    #[error("duplicate validator")]
    ValidatorDuplicate,

    #[error("validator not exists")]
    ValidatorNotExist,
}

impl ContractError {
    pub fn invalid_addr(reason: &str) -> Self {
        ContractError::InvalidAddress(reason.into())
    }

    pub fn invalid_args(reason: &str) -> Self {
        ContractError::InvalidArguments {
            reason: reason.into(),
        }
    }
}
