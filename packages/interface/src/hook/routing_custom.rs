use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::{
    ownable::{OwnableMsg, OwnableQueryMsg},
    router::{RouterMsg, RouterQuery},
};

use super::{HookQueryMsg, PostDispatchMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub mailbox: String,
}

#[cw_serde]
pub struct RegisterCustomHookMsg {
    pub dest_domain: u32,
    pub recipient: String,
    pub hook: String,
}

#[cw_serde]
pub struct ClearCustomHookMsg {
    pub dest_domain: u32,
    pub recipient: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    PostDispatch(PostDispatchMsg),
    Router(RouterMsg<Addr>),

    RegisterCustomHook(RegisterCustomHookMsg),
    RegisterCustomHooks(Vec<RegisterCustomHookMsg>),

    ClearCustomHook(ClearCustomHookMsg),
    ClearCustomHooks(Vec<ClearCustomHookMsg>),
}

#[cw_serde]
#[derive(QueryResponses)]
#[serde(untagged)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),
    Router(RouterQuery<Addr>),
    Hook(HookQueryMsg),
}
