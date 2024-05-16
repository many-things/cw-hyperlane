use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("invalid config. reason: {reason:?}")]
    InvalidConfig { reason: String },
}

impl ContractError {
    pub fn invalid_config(reason: &str) -> Self {
        Self::InvalidConfig {
            reason: reason.to_string(),
        }
    }
}
