use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, HexBinary, Uint256};

use crate::{
    mailbox, router,
    token::{TokenMode, TokenType},
};

#[cw_serde]
pub enum ExecuteMsg {
    Router(router::RouterMsg),

    // handle transfer remote
    Handle(mailbox::HandleMsg),

    // transfer to remote
    TransferRemote { dest_domain: u32, recipient: Binary },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(router::DomainsResponse)]
    Domains {},

    #[returns(router::RouterResponse)]
    Router { domain: u32 },

    #[returns(TokenTypeResponse)]
    TokenType {},

    #[returns(TokenModeResponse)]
    TokenMode {},
}

#[cw_serde]
pub struct TokenTypeResponse {
    #[serde(rename = "type")]
    pub typ: TokenType,
}

#[cw_serde]
pub struct TokenModeResponse {
    pub mode: TokenMode,
}
