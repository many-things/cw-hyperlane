use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{HexBinary, Uint128};

use crate::{
    connection::{ConnectionMsg, ConnectionQueryMsg},
    core,
    ism::IsmSpecifierQueryMsg,
    ownable::{OwnableMsg, OwnableQueryMsg},
    router::{RouterMsg, RouterQuery},
};

use super::{TokenModeMsg, TokenWarpDefaultQueryMsg};

#[cw_serde]
pub struct DenomUnit {
    pub denom: String,
    #[serde(
        serialize_with = "as_str::serialize",
        deserialize_with = "as_str::deserialize"
    )]
    pub exponent: u32,
    pub aliases: Vec<String>,
}

#[cw_serde]
pub struct Metadata {
    pub description: String,
    pub denom_units: Vec<DenomUnit>,
    pub base: String,
    pub display: String,
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct NativeModeBriged {
    pub denom: String,
    pub metadata: Option<Metadata>,
}

#[cw_serde]
pub struct NativeModeCollateral {
    pub denom: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token: TokenModeMsg<NativeModeBriged, NativeModeCollateral>,

    pub hrp: String,
    pub owner: String,
    pub mailbox: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    Router(RouterMsg<HexBinary>),
    Connection(ConnectionMsg),

    // handle transfer remote
    Handle(core::HandleMsg),

    // transfer to remote
    TransferRemote {
        dest_domain: u32,
        recipient: HexBinary,
        amount: Uint128,
        hook: Option<String>,
        metadata: Option<HexBinary>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),

    Router(RouterQuery<HexBinary>),

    Connection(ConnectionQueryMsg),

    TokenDefault(TokenWarpDefaultQueryMsg),

    IsmSpecifier(IsmSpecifierQueryMsg),
}

mod as_str {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).map_err(de::Error::custom)
    }

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        serializer.serialize_str(&value.to_string())
    }
}
