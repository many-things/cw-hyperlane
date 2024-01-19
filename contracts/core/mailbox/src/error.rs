use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] cw_utils::PaymentError),

    #[error("insufficient hook payment: wanted {wanted:?}, received {received:?}")]
    HookPayment {
        wanted: Vec<Coin>,
        received: Vec<Coin>,
    },

    #[error("{0}")]
    CoinsError(#[from] cosmwasm_std::CoinsError),

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

    #[error("message already delivered")]
    AlreadyDeliveredMessage {},
}

impl ContractError {
    pub fn invalid_config(reason: &str) -> Self {
        Self::InvalidConfig {
            reason: reason.to_string(),
        }
    }
}
