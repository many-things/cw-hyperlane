pub mod merkle;
pub mod pausable;
pub mod routing;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{HexBinary, Uint256};

#[cw_serde]
pub struct PostDispatchMsg {
    pub metadata: HexBinary,
    pub message: HexBinary,
}

impl PostDispatchMsg {
    pub fn wrap(self) -> ExpectedHookMsg {
        ExpectedHookMsg::PostDispatch(self)
    }
}

#[cw_serde]
pub struct QuoteDispatchMsg {
    pub metadata: HexBinary,
    pub message: HexBinary,
}

impl QuoteDispatchMsg {
    pub fn wrap(self) -> HookQueryMsg {
        HookQueryMsg::QuoteDispatch(self)
    }
}

/// This is the basic message to demonstrate the required interface
#[cw_serde]
pub enum ExpectedHookMsg {
    PostDispatch(PostDispatchMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum HookQueryMsg {
    #[returns(QuoteDispatchResponse)]
    QuoteDispatch(QuoteDispatchMsg),
}

#[cw_serde]
pub struct QuoteDispatchResponse {
    pub gas_amount: Uint256,
}
