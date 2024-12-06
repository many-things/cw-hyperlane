use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ConnectionMsg {
    SetMailbox { mailbox: String },

    SetHook { hook: Option<String> },

    SetIsm { ism: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum ConnectionQueryMsg {
    #[returns(MailboxResponse)]
    GetMailbox {},

    #[returns(HookResponse)]
    GetHook {},

    #[returns(IsmResponse)]
    GetIsm {},
}

#[cw_serde]
pub struct MailboxResponse {
    pub mailbox: Option<String>,
}

#[cw_serde]
pub struct HookResponse {
    pub hook: Option<String>,
}

#[cw_serde]
pub struct IsmResponse {
    pub ism: Option<String>,
}
