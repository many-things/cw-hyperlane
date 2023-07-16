use cosmwasm_std::{RecoverPubkeyError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    RecoverPubkeyError(#[from] RecoverPubkeyError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("WrongLength")]
    WrongLength {},

    #[error("InvalidPubKey")]
    InvalidPubKey {},
}
