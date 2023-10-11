use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Ownership transfer not started")]
    OwnershipTransferNotStarted,

    #[error("Ownership transfer already started")]
    OwnershipTransferAlreadyStarted,

    #[error("Paused")]
    Paused {},

    #[error("Not paused")]
    NotPaused {},

    #[error("Hook not registered for dest: {0}")]
    HookNotRegistered(u32),
}
