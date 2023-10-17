use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ConnectionMsg {
    SetMailbox { mailbox: String },

    SetIgp { igp: String },

    SetIsm { ism: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum ConnectionQueryMsg {
    #[returns(MailboxResponse)]
    GetMailbox {},

    #[returns(IgpResponse)]
    GetIgp {},

    #[returns(IsmResponse)]
    GetIsm {},
}

#[cw_serde]
pub struct MailboxResponse {
    pub mailbox: String,
}

#[cw_serde]
pub struct IgpResponse {
    pub igp: String,
}

#[cw_serde]
pub struct IsmResponse {
    pub ism: String,
}
