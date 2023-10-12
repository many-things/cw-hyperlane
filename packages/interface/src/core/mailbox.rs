use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub hrp: String,
    pub owner: String,
    pub domain: u32,
}

#[cw_serde]
pub enum ExecuteMsg {
    // overrides
    Ownable(OwnableMsg),

    // base
    SetDefaultIsm {
        ism: String,
    },
    SetDefaultHook {
        hook: String,
    },

    Dispatch {
        dest_domain: u32,
        recipient_addr: HexBinary,
        msg_body: HexBinary,
        hook: Option<String>,
        metadata: Option<HexBinary>,
    },
    Process {
        metadata: HexBinary,
        message: HexBinary,
    },
}

#[cw_serde]
pub struct DispatchResponse {
    pub message_id: HexBinary,
}

#[cw_serde]
#[derive(QueryResponses)]
#[serde(untagged)]
#[query_responses(nested)]
pub enum QueryMsg {
    // overrides
    Ownable(OwnableQueryMsg),

    // base
    Base(MailboxQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum MailboxQueryMsg {
    #[returns(HrpResponse)]
    Hrp {},

    #[returns(LocalDomainResponse)]
    LocalDomain {},

    #[returns(MessageDeliveredResponse)]
    MessageDelivered { id: HexBinary },

    #[returns(DefaultIsmResponse)]
    DefaultIsm {},

    #[returns(DefaultHookResponse)]
    DefaultHook {},

    #[returns(RecipientIsmResponse)]
    RecipientIsm { recipient_addr: String },
}

#[cw_serde]
pub struct HrpResponse {
    pub hrp: String,
}

#[cw_serde]
pub struct LocalDomainResponse {
    pub local_domain: u32,
}

#[cw_serde]
pub struct MessageDeliveredResponse {
    pub delivered: bool,
}

#[cw_serde]
pub struct DefaultIsmResponse {
    pub default_ism: String,
}

#[cw_serde]
pub struct DefaultHookResponse {
    pub default_hook: String,
}

#[cw_serde]
pub struct RecipientIsmResponse {
    pub ism: String,
}
