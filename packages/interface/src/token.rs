use core::fmt;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, HexBinary, Uint256};

use crate::{mailbox, router};

#[cw_serde]
pub enum ExecuteMsg {
    Router(router::RouterMsg),

    // handle transfer remote
    Handle(mailbox::HandleMsg),

    // transfer to remote
    TransferRemote {
        dest_domain: u32,
        recipient: Binary,
        amount: Uint256,
    },
}

#[cw_serde]
pub enum TokenTypeNative {
    Fungible { denom: String },
    NonFungible { class: String },
}

#[cw_serde]
pub enum TokenTypeCW {
    CW20 { contract: String },
    CW721 { contract: String },
}

#[cw_serde]
pub enum TokenType {
    Native(TokenTypeNative),
    CW(TokenTypeCW),
}

#[cw_serde]
pub enum TokenMode {
    Bridged,
    Collateral,
}

impl fmt::Display for TokenMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bridged => "bridged",
                Self::Collateral => "collateral",
            }
        )
    }
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

#[cw_serde]
pub struct Message {
    pub recipient: Binary,
    pub amount: Uint256,
    pub metadata: Binary,
}

impl From<Message> for Binary {
    fn from(v: Message) -> Self {
        v.recipient
            .iter()
            .chain(v.amount.to_be_bytes().iter())
            .chain(v.metadata.iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<Message> for HexBinary {
    fn from(v: Message) -> Self {
        Binary::from(v).into()
    }
}

impl From<Binary> for Message {
    fn from(v: Binary) -> Self {
        Self {
            recipient: Binary(v[0..32].to_vec()),
            amount: Uint256::from_be_bytes(v[32..64].try_into().unwrap()),
            metadata: Binary(v[64..].to_vec()),
        }
    }
}

impl From<HexBinary> for Message {
    fn from(v: HexBinary) -> Self {
        Binary(v.into()).into()
    }
}
