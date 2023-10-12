use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, HexBinary, Uint256};

use crate::{
    hook::{HookQueryMsg, PostDispatchMsg},
    ownable::{OwnableMsg, OwnableQueryMsg},
    router::{RouterMsg, RouterQuery},
};

use super::gas_oracle::IgpGasOracleQueryMsg;

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
    // overrides
    Ownable(OwnableMsg),
    Router(RouterMsg<Addr>),
    PostDispatch(PostDispatchMsg),

    // base
    SetBeneficiary {
        beneficiary: String,
    },
    PayForGas {
        message_id: HexBinary,
        dest_domain: u32,
        gas_amount: Uint256,
        refund_address: String,
    },
    Claim {},
}

#[cw_serde]
#[derive(QueryResponses)]
#[serde(untagged)]
#[query_responses(nested)]
pub enum QueryMsg {
    // overrides
    Ownable(OwnableQueryMsg),
    Hook(HookQueryMsg),
    Router(RouterQuery<Addr>),
    Oracle(IgpGasOracleQueryMsg),

    // base
    Base(IgpQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum IgpQueryMsg {
    #[returns(BeneficiaryResponse)]
    Beneficiary {},

    #[returns(QuoteGasPaymentResponse)]
    QuoteGasPayment {
        dest_domain: u32,
        gas_amount: Uint256,
    },
}

#[cw_serde]
pub struct BeneficiaryResponse {
    pub beneficiary: String,
}

#[cw_serde]
pub struct QuoteGasPaymentResponse {
    pub gas_needed: Uint256,
}
