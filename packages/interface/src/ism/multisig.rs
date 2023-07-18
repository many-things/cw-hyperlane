use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, HexBinary};

#[allow(unused_imports)]
use super::{ModuleTypeResponse, VerifyResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub addr_prefix: String,
}

#[cw_serde]
pub struct ValidatorSet {
    pub domain: u32,
    pub validator: String,
    pub validator_pubkey: Binary,
}

#[cw_serde]
pub struct ThresholdSet {
    pub domain: u32,
    pub threshold: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    EnrollValidator { set: ValidatorSet },
    EnrollValidators { set: Vec<ValidatorSet> },
    UnenrollValidator { domain: u32, validator: String },

    SetThreshold { set: ThresholdSet },
    SetThresholds { set: Vec<ThresholdSet> },

    InitTransferOwnership { owner: String },
    FinishTransferOwnership(),
    RevokeTransferOwnership(),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ModuleTypeResponse)]
    ModuleType {},

    #[returns(VerifyResponse)]
    Verify {
        metadata: HexBinary,
        message: HexBinary,
    },
}

#[cw_serde]
pub struct MigrateMsg {}
