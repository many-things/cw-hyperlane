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
    // overrides
    Ownable(OwnableMsg),
    Pausable(PausableMsg),
    PostDispatch(PostDispatchMsg),

    // base
    UpdateMailbox { mailbox: String },
}

#[cw_serde]
#[derive(QueryResponses)]
#[serde(untagged)]
#[query_responses(nested)]
pub enum QueryMsg {
    // overrides
    Pausable(PausableQueryMsg),
    Ownable(OwnableQueryMsg),
    Hook(HookQueryMsg),

    // base
    Base(PausableHookQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum PausableHookQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub mailbox: String,
}
