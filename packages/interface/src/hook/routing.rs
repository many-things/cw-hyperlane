use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::{
    ownable::{OwnableMsg, OwnableQueryMsg},
    pausable::{PausableMsg, PausableQueryMsg},
    router::{RouterMsg, RouterQuery},
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

    // routing
    UpdateMailbox { mailbox: String },
    Router(RouterMsg<Addr>),
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

    // routing
    Routing(RoutingQueryMsg),
    Router(RouterQuery<Addr>),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum RoutingQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub mailbox: String,
}

#[cw_serde]
pub struct MigrateMsg {}
