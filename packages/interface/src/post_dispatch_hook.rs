use cosmwasm_schema::cw_serde;
use cosmwasm_std::{HexBinary, Uint256};

#[cw_serde]
pub enum PostDispatchMsg {
    PostDispatch {
        metadata: HexBinary,
        message: HexBinary,
    },
}

#[cw_serde]
pub enum PostDispatchQueryMsg {
    QuoteDispatch {
        metadata: HexBinary,
        message: HexBinary,
    },
}

#[cw_serde]
pub struct QuoteDispatchResponse {
    pub gas_amount: Uint256,
}
