use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{wasm_execute, Addr, Api, Coin, CosmosMsg, HexBinary, StdResult};

#[allow(unused_imports)]
use crate::{
    hook::QuoteDispatchResponse,
    ownable::{OwnableMsg, OwnableQueryMsg},
    types,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub hrp: String,
    pub owner: String,
    pub domain: u32,
}

#[cw_serde]
pub struct DispatchMsg {
    pub dest_domain: u32,
    pub recipient_addr: HexBinary,
    pub msg_body: HexBinary,
    pub hook: Option<String>,
    pub metadata: Option<HexBinary>,
}

impl DispatchMsg {
    pub fn new(
        dest_domain: u32,
        recipient_addr: impl Into<HexBinary>,
        msg_body: impl Into<HexBinary>,
    ) -> Self {
        Self {
            dest_domain,
            recipient_addr: recipient_addr.into(),
            msg_body: msg_body.into(),
            hook: None,
            metadata: None,
        }
    }

    pub fn with_hook(mut self, hook: impl Into<String>) -> Self {
        self.hook = Some(hook.into());
        self
    }

    pub fn with_metadata(mut self, metadata: impl Into<HexBinary>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    pub fn to_msg(
        self,
        version: u8,
        nonce: u32,
        origin_domain: u32,
        sender: impl Into<String>,
    ) -> StdResult<types::Message> {
        Ok(types::Message {
            version,
            nonce,
            origin_domain,
            sender: types::bech32_to_h256(&sender.into())?.to_vec().into(),
            dest_domain: self.dest_domain,
            recipient: self.recipient_addr,
            body: self.msg_body,
        })
    }

    pub fn get_hook_addr(&self, api: &dyn Api, default: Addr) -> StdResult<Addr> {
        Ok(self
            .hook
            .as_ref()
            .map(|v| api.addr_validate(v))
            .transpose()?
            .unwrap_or(default))
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    // overrides
    Ownable(OwnableMsg),

    // Mailbox
    SetDefaultIsm {
        ism: String,
    },

    SetDefaultHook {
        hook: String,
    },

    SetRequiredHook {
        hook: String,
    },

    Dispatch(DispatchMsg),

    Process {
        metadata: HexBinary,
        message: HexBinary,
    },
}

pub fn dispatch(
    mailbox: impl Into<String>,
    dest_domain: u32,
    recipient_addr: HexBinary,
    msg_body: HexBinary,
    hook: Option<String>,
    metadata: Option<HexBinary>,
    funds: Vec<Coin>,
) -> StdResult<CosmosMsg> {
    Ok(wasm_execute(
        mailbox,
        &ExecuteMsg::Dispatch(DispatchMsg {
            dest_domain,
            recipient_addr,
            msg_body,
            hook,
            metadata,
        }),
        funds,
    )?
    .into())
}

pub fn process(mailbox: impl Into<String>, metadata: HexBinary, message: HexBinary) -> CosmosMsg {
    wasm_execute(mailbox, &ExecuteMsg::Process { metadata, message }, vec![])
        .unwrap()
        .into()
}

#[cw_serde]
pub struct DispatchResponse {
    pub message_id: HexBinary,
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    // overrides
    Ownable(OwnableQueryMsg),

    Hook(MailboxHookQueryMsg),

    // mailbox
    Mailbox(MailboxQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum MailboxHookQueryMsg {
    #[returns(QuoteDispatchResponse)]
    QuoteDispatch { sender: String, msg: DispatchMsg },
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

    #[returns(RequiredHookResponse)]
    RequiredHook {},

    #[returns(NonceResponse)]
    Nonce {},

    #[returns(RecipientIsmResponse)]
    RecipientIsm { recipient_addr: String },

    #[returns(LatestDispatchedIdResponse)]
    LatestDispatchId {},
}
impl MailboxQueryMsg {
    pub fn wrap(self) -> QueryMsg {
        QueryMsg::Mailbox(self)
    }
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
pub struct RequiredHookResponse {
    pub required_hook: String,
}

#[cw_serde]
pub struct RecipientIsmResponse {
    pub ism: String,
}

#[cw_serde]
pub struct NonceResponse {
    pub nonce: u32,
}

#[cw_serde]
pub struct LatestDispatchedIdResponse {
    pub message_id: HexBinary,
}
