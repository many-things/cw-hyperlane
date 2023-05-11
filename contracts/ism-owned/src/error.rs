use cosmwasm_std::{StdError, VerificationError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    VerificationError(#[from] VerificationError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Wrong length")]
    WrongLength,

    #[error("Invalid pubkey")]
    InvalidPubKey,
}
