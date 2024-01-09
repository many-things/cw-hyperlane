use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::{ownable::{OwnableMsg, OwnableQueryMsg}, pausable::{PausableMsg, PausableQueryMsg}};

use super::IsmQueryMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub paused: bool
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    Pausable(PausableMsg)
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),
    Ism(IsmQueryMsg),
    Pausable(PausableQueryMsg)
}
