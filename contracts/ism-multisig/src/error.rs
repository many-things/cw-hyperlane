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
    WrongLength {},

    #[error("Invalid pubkey")]
    InvalidPubKey {},

    #[error("Ownership transfer not started")]
    OwnershipTransferNotStarted {},

    #[error("Ownership transfer already started")]
    OwnershipTransferAlreadyStarted {},

    #[error("Validator pubkey mismatched")]
    ValidatorPubKeyMismatched {},

    #[error("Duplicate Validator")]
    ValidatorDuplicate {},
}
