pub mod aggregate;
pub mod merkle;
pub mod pausable;
pub mod routing;
pub mod routing_custom;
pub mod routing_fallback;
pub mod wormhole;
pub mod axelar;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    wasm_execute, Coin, CustomQuery, HexBinary, QuerierWrapper, StdResult, WasmMsg, Uint128,
};

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
#[derive(Default)]
pub struct QuoteDispatchMsg {
    pub metadata: HexBinary,
    pub message: HexBinary,
}

impl QuoteDispatchMsg {
    pub fn wrap(self) -> HookQueryMsg {
        HookQueryMsg::QuoteDispatch(self)
    }

    pub fn request(self) -> ExpectedHookQueryMsg {
        ExpectedHookQueryMsg::Hook(self.wrap())
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

    #[returns(MailboxResponse)]
    Mailbox {},
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum ExpectedHookQueryMsg {
    Hook(HookQueryMsg),
}

#[cw_serde]
pub struct MailboxResponse {
    pub mailbox: String,
}

#[cw_serde]
pub struct QuoteDispatchResponse {
    pub gas_amount: Option<Coin>,
}

pub fn post_dispatch(
    hook: impl Into<String>,
    metadata: impl Into<HexBinary>,
    message: impl Into<HexBinary>,
    funds: Option<Vec<Coin>>,
) -> StdResult<WasmMsg> {
    wasm_execute(
        hook,
        &PostDispatchMsg {
            metadata: metadata.into(),
            message: message.into(),
        }
        .wrap(),
        funds.unwrap_or_default(),
    )
}

pub fn quote_dispatch<C: CustomQuery>(
    querier: &QuerierWrapper<C>,
    hook: impl Into<String>,
    metadata: impl Into<HexBinary>,
    message: impl Into<HexBinary>,
) -> StdResult<QuoteDispatchResponse> {
    querier.query_wasm_smart(
        hook,
        &QuoteDispatchMsg {
            metadata: metadata.into(),
            message: message.into(),
        }
        .request(),
    )
}
