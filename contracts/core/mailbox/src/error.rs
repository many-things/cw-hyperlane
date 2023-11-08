use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] cw_utils::PaymentError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("ism verify failed")]
    VerifyFailed {},

    #[error("invalid config. reason: {reason:?}")]
    InvalidConfig { reason: String },

    #[error("invalid address length: {len:?}")]
    InvalidAddressLength { len: usize },

    #[error("invalid message version: {version:?}")]
    InvalidMessageVersion { version: u8 },

    #[error("invalid destination domain: {domain:?}")]
    InvalidDestinationDomain { domain: u32 },

    #[error("insufficient funds. required: {required}, received: {received}")]
    InsufficientFunds { required: Coin, received: Coin },

    #[error("message already delivered")]
    AlreadyDeliveredMessage {},

    #[error("message not found")]
    MessageNotFound {},
}

impl ContractError {
    pub fn invalid_config(reason: &str) -> Self {
        Self::InvalidConfig {
            reason: reason.to_string(),
        }
    }
}
