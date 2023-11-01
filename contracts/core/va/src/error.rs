use cosmwasm_std::{RecoverPubkeyError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    RecoverPubkeyError(#[from] RecoverPubkeyError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("invalid address. reason: {0}")]
    InvalidAddress(String),

    #[error("verify failed")]
    VerifyFailed {},
}

impl ContractError {
    pub fn invalid_addr(reason: &str) -> Self {
        ContractError::InvalidAddress(reason.into())
    }
}
