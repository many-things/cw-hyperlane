use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CosmosMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Aggregate(Vec<CosmosMsg>),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AggregateResponse)]
    AggregateStatic(Vec<StaticCall>),
}

#[cw_serde]
pub struct StaticCall {
    pub path: String,
    pub data: Binary,
}

#[cw_serde]
pub struct AggregateResponse(pub Vec<Binary>);
