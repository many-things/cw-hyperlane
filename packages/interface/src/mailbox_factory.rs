use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub origin_domain: u32,
    pub mailbox_code: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Instantiate { owner: String, default_ism: String },
    Migrate {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OriginDomainResponse)]
    OriginDomain,
}

#[cw_serde]
pub struct OriginDomainResponse(pub u32);
