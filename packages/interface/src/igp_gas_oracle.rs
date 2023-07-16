use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::ownable::OwnableMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct RemoteGasDataConfig {
    pub remote_domain: u32,
    pub token_exchange_rate: Uint128,
    pub gas_price: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    // ownership
    Ownership(OwnableMsg),

    // gas data
    SetRemoteGasDataConfigs { configs: Vec<RemoteGasDataConfig> },
    SetRemoteGasData { config: RemoteGasDataConfig },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(GetExchangeRateAndGasPriceResponse)]
    GetExchangeRateAndGasPrice { dest_domain: u32 },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
    pub pending_owner: Option<String>,
}

#[cw_serde]
pub struct GetExchangeRateAndGasPriceResponse {
    pub gas_price: Uint128,
    pub exchange_rate: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}
