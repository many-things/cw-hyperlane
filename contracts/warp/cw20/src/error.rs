use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ParseReplyError(#[from] cw_utils::ParseReplyError),

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("unauthorized")]
    Unauthorized,

    #[error("wrong length")]
    WrongLength {},

    #[error("invalid token option")]
    InvalidTokenOption,

    #[error("invalid reply id")]
    InvalidReplyId,

    #[error("invalid receive msg")]
    InvalidReceiveMsg,

    #[error("no router for domain {domain:?}")]
    NoRouter { domain: u32 },
}
