use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Uint128, Uint256};

use crate::{ownable::OwnableMsg, post_dispatch_hook::PostDispatchQueryMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub gas_token: String,
    pub beneficiary: String,
    pub prefix: String,
}

#[cw_serde]
pub struct GasOracleConfig {
    pub remote_domain: u32,
    pub gas_oracle: String,
}

impl From<(u32, String)> for GasOracleConfig {
    fn from((remote_domain, gas_oracle): (u32, String)) -> Self {
        Self {
            remote_domain,
            gas_oracle,
        }
    }
}

impl From<(u32, Addr)> for GasOracleConfig {
    fn from((remote_domain, gas_oracle): (u32, Addr)) -> Self {
        Self {
            remote_domain,
            gas_oracle: gas_oracle.to_string(),
        }
    }
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
    PostDispatch {
        metadata: Binary,
        message: Binary,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QuoteDispatchResponse)]
    QuoteDispatch(PostDispatchQueryMsg),

    #[returns(QuoteGasPaymentResponse)]
    QuoteGasPayment {
        dest_domain: u32,
        gas_amount: Uint256,
    },

    #[returns(GetExchangeRateAndGasPriceResponse)]
    GetExchangeRateAndGasPrice { dest_domain: u32 },
}

#[cw_serde]
pub struct QuoteDispatchResponse {
    pub gas_amount: Uint256,
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
