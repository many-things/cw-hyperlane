use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary};

use crate::domain_routing_hook;

#[cw_serde]
pub enum ExecuteMsg {
    DomainRoutingHookMsg(domain_routing_hook::ExecuteMsg),
    ConfigCustomHook {
        destination_domain: u32,
        recipient: HexBinary,
        hook: String,
    },
}
