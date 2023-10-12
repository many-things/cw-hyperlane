use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::{
    ownable::{OwnableMsg, OwnableQueryMsg},
    pausable::{PausableMsg, PausableQueryMsg},
};

use super::{HookQueryMsg, PostDispatchMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub mailbox: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    Pausable(PausableMsg),
    PostDispatch(PostDispatchMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
#[serde(untagged)]
#[query_responses(nested)]
pub enum QueryMsg {
    Pausable(PausableQueryMsg),
    Ownable(OwnableQueryMsg),
    Hook(HookQueryMsg),
}
