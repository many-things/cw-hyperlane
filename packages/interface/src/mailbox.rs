use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, HexBinary};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub default_ism: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Pause {},
    Unpause {},
    SetDefaultISM {
        ism: String,
    },

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
pub struct DispatchResponse {
    pub message_id: Binary,
}

#[cw_serde]
pub struct HandleMsg {
    pub origin: u32,
    pub sender: HexBinary,
    pub body: HexBinary,
}

#[cw_serde]
pub enum ExpectedHandlerMsg {
    Handle { msg: HandleMsg },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(RootResponse)]
    Root {},

    #[returns(CountResponse)]
    Count {},

    #[returns(CheckPointResponse)]
    CheckPoint {},

    #[returns(PausedResponse)]
    Paused {},

    #[returns(NonceResponse)]
    Nonce {},
}

#[cw_serde]
pub struct RootResponse {
    pub root: HexBinary,
}

#[cw_serde]
pub struct CountResponse {
    pub count: u128,
}

#[cw_serde]
pub struct CheckPointResponse {
    pub root: HexBinary,
    pub count: u128,
}

#[cw_serde]
pub struct PausedResponse {
    pub paused: bool,
}

#[cw_serde]
pub struct NonceResponse {
    pub nonce: u32,
}
