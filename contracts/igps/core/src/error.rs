#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("insufficient funds")]
    InsufficientFunds {},

    #[error("gas oracle not found for {0}")]
    GasOracleNotFound(u32),
}
