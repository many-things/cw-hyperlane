use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::{
    core,
    ownable::OwnableQueryMsg,
    router::{self, RouterQuery},
};

use super::{TokenMode, TokenWarpDefaultQueryMsg};

#[cw_serde]
pub enum TokenOption {
    Create {
        code_id: u64,
        init_msg: Box<cw20_base::msg::InstantiateMsg>,
    },
    Reuse {
        contract: String,
    },
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token: Option<TokenOption>,
    pub mode: TokenMode,

    pub hrp: String,
    pub owner: String,
    pub mailbox: String,
}

#[cw_serde]
pub enum ReceiveMsg {
    // transfer to remote
    TransferRemote {
        dest_domain: u32,
        recipient: HexBinary,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    Router(router::RouterMsg<HexBinary>),

    /// handle transfer remote
    Handle(core::HandleMsg),

    // cw20 receiver
    Receive(cw20::Cw20ReceiveMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
#[serde(untagged)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),
    Router(RouterQuery<HexBinary>),
    TokenDefault(TokenWarpDefaultQueryMsg),
}
