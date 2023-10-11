use cosmwasm_schema::cw_serde;
use cosmwasm_std::HexBinary;

pub mod mailbox;
pub mod va;

#[cw_serde]
pub struct HandleMsg {
    pub origin: u32,
    pub sender: HexBinary,
    pub body: HexBinary,
}

#[cw_serde]
pub enum ExpectedHandleMsg {
    Handle(HandleMsg),
}
