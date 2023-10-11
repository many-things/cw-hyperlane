use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::{
    hook::{HookConfig, OwnerResponse, PauseInfoResponse},
    ownable::OwnableMsg,
};

use super::{PostDispatchMsg, QuoteDispatchResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub mailbox: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownership(OwnableMsg),
    Pause {},
    Unpause {},
    UpdateMailbox {
        mailbox: String,
    },
    SetHook {
        destination: u32,
        hook: String,
    },
    SetHooks {
        hooks: Vec<HookConfig>,
    },
    PostDispatch {
        metadata: HexBinary,
        message: HexBinary,
    },
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QuoteDispatchResponse)]
    QuoteDispatch(PostDispatchMsg),

    #[returns(PauseInfoResponse)]
    PauseInfo {},

    #[returns(OwnerResponse)]
    Owner {},
}
