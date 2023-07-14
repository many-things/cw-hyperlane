use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128, Uint256};

use crate::ownable::OwnableMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub beneficiary: String,
}

#[cw_serde]
pub struct GasOracleConfig {
    pub remote_domain: u32,
    pub gas_oracle: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownership(OwnableMsg),

    SetGasOracles {
        configs: Vec<GasOracleConfig>,
    },
    SetBeneficiary {
        beneficiary: String,
    },

    PayForGas {
        message_id: Binary,
        dest_domain: u32,
        gas_amount: Uint256,
        refund_address: String,
    },
    Claim {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QuoteGasPaymentResponse)]
    QuoteGasPayment {
        dest_domain: u32,
        gas_amount: Uint256,
    },

    #[returns(GetExchangeRateAndGasPriceResponse)]
    GetExchangeRateAndGasPrice { dest_domain: u32 },
}

#[cw_serde]
pub struct QuoteGasPaymentResponse {
    pub gas_needed: Uint256,
}

#[cw_serde]
pub struct GetExchangeRateAndGasPriceResponse {
    pub gas_price: Uint128,
    pub exchange_rate: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}
