pub mod merkle;
pub mod pausable;
pub mod routing;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

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

/// This is the basic message to demonstrate the required interface
#[cw_serde]
pub enum ExpectedHookMsg {
    PostDispatch(PostDispatchMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum HookQueryMsg {
    #[returns(MailboxResponse)]
    Mailbox {},
}

#[cw_serde]
pub struct MailboxResponse {
    pub mailbox: String,
}
