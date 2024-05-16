use cosmwasm_std::Uint256;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("insufficient funds: needed {gas_needed:?}, but only received {received:?}")]
    InsufficientFunds {
        received: Uint256,
        gas_needed: Uint256,
    },

    #[error("invalid config. reason: {reason:?}")]
    InvalidConfig { reason: String },

    #[error("gas oracle not found for {0}")]
    GasOracleNotFound(u32),
}

impl ContractError {
    pub fn invalid_config(reason: &str) -> Self {
        Self::InvalidConfig {
            reason: reason.to_string(),
        }
    }
}
