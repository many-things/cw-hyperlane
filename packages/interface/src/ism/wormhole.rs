use super::IsmQueryMsg;
use crate::ownable::{OwnableMsg, OwnableQueryMsg};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub wormhole_core: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),

    SetWormholeCore { wormhole_core: String },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),

    Ism(IsmQueryMsg),

    WormholeIsm(WormholeIsmQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum WormholeIsmQueryMsg {
    #[returns(String)]
    WormholeCore {},
}
