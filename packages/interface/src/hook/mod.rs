pub mod merkle;
pub mod pausable;
pub mod routing;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary, Uint256};

/// This is the basic message to demonstrate the required interface
#[cw_serde]
pub enum PostDispatchWrapperMsg {
    PostDispatch(PostDispatchMsg),
}

#[cw_serde]
pub struct PostDispatchMsg {
    metadata: HexBinary,
    message: HexBinary,
}

#[cw_serde]
pub struct QuoteDispatchMsg {
    metadata: HexBinary,
    message: HexBinary,
}

#[cw_serde]
pub struct HookConfig {
    pub destination: u32,
    pub hook: Addr,
}

#[cw_serde]
pub struct PauseInfoResponse {
    pub paused: bool,
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: String,
}

#[cw_serde]
pub struct QuoteDispatchResponse {
    pub gas_amount: Uint256,
}
