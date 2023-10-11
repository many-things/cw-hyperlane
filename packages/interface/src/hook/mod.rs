pub mod merkle;
pub mod pausable;
pub mod routing;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{HexBinary, Uint256};

/// This is the basic message to demonstrate the required interface
#[cw_serde]
pub enum ExpectedHookMsg {
    PostDispatch(PostDispatchMsg),
}

#[cw_serde]
pub struct PostDispatchMsg {
    metadata: HexBinary,
    message: HexBinary,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum HookQueryMsg {
    #[returns(QuoteDispatchResponse)]
    QuoteDispatch(QuoteDispatchMsg),
}

#[cw_serde]
pub struct QuoteDispatchMsg {
    metadata: HexBinary,
    message: HexBinary,
}

#[cw_serde]
pub struct QuoteDispatchResponse {
    pub gas_amount: Uint256,
}
