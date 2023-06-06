use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub chain_hpl: String,
}

#[cw_serde]
pub struct ValidatorSet {
    pub domain: u64,
    pub validator: String,
    pub validator_pubkey: Binary,
}

#[cw_serde]
pub struct ThresholdSet {
    pub domain: u64,
    pub threshold: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    EnrollValidator(ValidatorSet),
    EnrollValidators(Vec<ValidatorSet>),
    UnenrollValidator { domain: u64, validator: String },

    SetThreshold(ThresholdSet),
    SetThresholds(Vec<ThresholdSet>),

    InitTransferOwnership(String),
    FinishTransferOwnership(),
    RevokeTransferOwnership(),
}

#[cw_serde]
pub struct MigrateMsg {}
