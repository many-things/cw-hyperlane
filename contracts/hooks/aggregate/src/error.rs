use cosmwasm_std::StdError;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("hook paused")]
    Paused {},

    #[error("invalid hook gas denom. expected: {expected}, actual: {actual}")]
    InvalidGas { expected: String, actual: String },
}
