use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, HexBinary, Uint256};

use crate::{ownable::OwnableMsg, post_dispatch_hook::PostDispatchQueryMsg};

#[cw_serde]
pub struct HookConfig {
    pub destination: u32,
    pub hook: Addr,
}

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
    QuoteDispatch(PostDispatchQueryMsg),

    #[returns(PauseInfoResponse)]
    PauseInfo {},

    #[returns(OwnerResponse)]
    Owner {},
}

#[cw_serde]
pub struct QuoteDispatchResponse {
    pub gas_amount: Uint256,
}

#[cw_serde]
pub struct PauseInfoResponse {
    pub paused: bool,
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: String,
}
