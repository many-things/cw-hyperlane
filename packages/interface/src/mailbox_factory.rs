use cosmwasm_schema::{cw_serde, QueryResponses};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub origin_domain: u32,
    pub mailbox_code: u64,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Instantiate { owner: String, default_ism: String },
    Migrate {},
}

#[cw_serde]
pub struct MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OriginDomainResponse)]
    OriginDomain,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct OriginDomainResponse(pub u32);
