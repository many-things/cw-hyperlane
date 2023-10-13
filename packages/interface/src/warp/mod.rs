pub mod cw20;
pub mod native;

use std::fmt;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{HexBinary, Uint256};

#[cw_serde]
pub enum TokenTypeNative {
    Fungible { denom: String },
    NonFungible { class: String },
}

#[cw_serde]
pub enum TokenType {
    Native(TokenTypeNative),
    CW20 { contract: String },
    CW721 { contract: String },
}

#[cw_serde]
pub enum TokenModeMsg<Bridged, Collateral> {
    Bridged(Bridged),
    Collateral(Collateral),
}

impl<A, B> From<TokenModeMsg<A, B>> for TokenMode {
    fn from(v: TokenModeMsg<A, B>) -> Self {
        match v {
            TokenModeMsg::Bridged(_) => Self::Bridged,
            TokenModeMsg::Collateral(_) => Self::Collateral,
        }
    }
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
pub struct Message {
    pub recipient: HexBinary,
    pub amount: Uint256,
    pub metadata: HexBinary,
}

impl From<Message> for HexBinary {
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

impl From<HexBinary> for Message {
    fn from(v: HexBinary) -> Self {
        Self {
            recipient: v[0..32].to_vec().into(),
            amount: Uint256::from_be_bytes(v[32..64].try_into().unwrap()),
            metadata: v[64..].to_vec().into(),
        }
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum TokenWarpDefaultQueryMsg {
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
