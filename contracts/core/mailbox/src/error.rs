use cosmwasm_std::StdError;
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

    #[error("invalid address length: {len:?}")]
    InvalidAddressLength { len: usize },

    #[error("invalid message version: {version:?}")]
    InvalidMessageVersion { version: u8 },

    #[error("invalid destination domain: {domain:?}")]
    InvalidDestinationDomain { domain: u32 },

    #[error("message already delivered")]
    AlreadyDeliveredMessage {},
}
