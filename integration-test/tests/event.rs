use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Event, HexBinary};

#[cw_serde]
pub struct CwDispatchEvent {
    pub sender: HexBinary,
    pub destination: u32,
    pub recipient: HexBinary,
    pub message: HexBinary,
}

pub fn parse_dispatch_from_res(events: &[Event]) -> CwDispatchEvent {
    let found = events
        .iter()
        .find(|v| v.ty == "wasm-mailbox_dispatch")
        .unwrap();

    let actual = &found.attributes[1..];

    CwDispatchEvent {
        sender: HexBinary::from_hex(&actual[0].value).unwrap(),
        destination: actual[1].value.parse::<u32>().unwrap(),
        recipient: HexBinary::from_hex(&actual[2].value).unwrap(),
        message: HexBinary::from_hex(&actual[3].value).unwrap(),
    }
}

#[cw_serde]
pub struct CwDispatchId {
    pub id: HexBinary,
}

pub fn parse_dispatch_id_from_res(events: &[Event]) -> CwDispatchId {
    let found = events
        .iter()
        .find(|v| v.ty == "wasm-mailbox_dispatch_id")
        .unwrap();

    let actual = &found.attributes[1..];

    CwDispatchId {
        id: HexBinary::from_hex(&actual[0].value).unwrap(),
    }
}
