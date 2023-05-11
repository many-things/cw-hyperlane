use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub default_ism: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Pause,
    Unpause,
    SetDefaultISM(String),

    Dispatch {
        dest_domain: u32,
        recipient_addr: HexBinary,
        msg_body: HexBinary,
    },
    Process {
        metadata: HexBinary,
        message: HexBinary,
    },
}

#[cw_serde]
pub struct HandleMsg {
    pub origin: u32,
    pub sender: HexBinary,
    pub body: HexBinary,
}

#[cw_serde]
pub enum ExpectedHandlerMsg {
    Handle(HandleMsg),
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    // #[returns(YourQueryResponse)]
    // YourQuery {},
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
