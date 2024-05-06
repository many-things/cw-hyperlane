use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{HexBinary, Uint128};

use crate::{
    connection::{ConnectionMsg, ConnectionQueryMsg},
    core,
    ism::IsmSpecifierQueryMsg,
    ownable::{OwnableMsg, OwnableQueryMsg},
    router::{self, RouterQuery},
};

use super::{TokenModeMsg, TokenWarpDefaultQueryMsg};

pub use cw20_base::msg::InstantiateMsg as Cw20InitMsg;

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
pub struct Cw20ModeBridged {
    pub code_id: u64,
    pub init_msg: Box<cw20_base::msg::InstantiateMsg>,
}

#[cw_serde]
pub struct Cw20ModeCollateral {
    pub address: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token: TokenModeMsg<Cw20ModeBridged, Cw20ModeCollateral>,

    pub hrp: String,
    pub owner: String,
    pub mailbox: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    Router(router::RouterMsg<HexBinary>),
    Connection(ConnectionMsg),

    // handle transfer remote
    Handle(core::HandleMsg),

    // transfer to remote
    TransferRemote {
        dest_domain: u32,
        recipient: HexBinary,
        amount: Uint128,
        hook: Option<String>,
        metadata: Option<HexBinary>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),

    Router(RouterQuery<HexBinary>),

    Connection(ConnectionQueryMsg),

    TokenDefault(TokenWarpDefaultQueryMsg),

    IsmSpecifier(IsmSpecifierQueryMsg),
}
