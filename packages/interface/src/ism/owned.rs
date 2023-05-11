use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {
    pub hpl: String,
    pub owner: String,
    pub owner_pubkey: Binary,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateOwner { owner: String, owner_pubkey: Binary },
}

#[cw_serde]
pub struct MigrateMsg {}
