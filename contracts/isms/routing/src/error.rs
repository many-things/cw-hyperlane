use cosmwasm_std::{StdError, VerificationError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    VerificationError(#[from] VerificationError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("RouteNotFound")]
    RouteNotFound {},
}
