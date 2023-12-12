#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct QuerySupplyOfRequest {
    /// denom is the coin denom to query balances for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}

/// QuerySupplyOfResponse is the response type for the Query/SupplyOf RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct QuerySupplyOfResponse {
    /// amount is the supply of the coin.
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<Coin>,
}

#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
