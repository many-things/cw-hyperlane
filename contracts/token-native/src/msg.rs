use cosmwasm_schema::cw_serde;
use hpl_interface::token::TokenMode;

#[cw_serde]
pub struct DenomUnit {
    pub denom: String,
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
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
pub struct InstantiateMsg {
    pub denom: String,
    pub metadata: Option<Metadata>,
    pub mode: TokenMode,

    pub hrp: String,
    pub owner: String,
    pub mailbox: String,
}

#[cw_serde]
pub struct MigrateMsg {}
