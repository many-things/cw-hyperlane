use cosmwasm_std::{RecoverPubkeyError, StdError};
use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    RecoverPubkeyError(#[from] RecoverPubkeyError),

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("unauthorized")]
    Unauthorized,

    #[error("wrong length")]
    WrongLength,

    #[error("invalid reply id")]
    InvalidReplyId,

    #[error("insufficient funds")]
    InsufficientFunds,

    #[error("no route for domain {domain:?}")]
    NoRouter { domain: u32 },
}
