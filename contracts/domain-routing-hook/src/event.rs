use cosmwasm_std::{Addr, Event, HexBinary};
use hpl_interface::domain_routing_hook::HookConfig;

pub fn emit_set_hook(destination: u32, hook: Addr) -> Event {
    Event::new("domain_routing_hook_set_hook")
        .add_attribute("destination", destination.to_string())
        .add_attribute("hook", hook.to_string())
}

pub fn emit_set_hooks(hooks: Vec<HookConfig>) -> Event {
    Event::new("domain_rouitng_hook_set_hooks")
        .add_attribute("hooks", serde_json::to_string(&hooks).unwrap())
}

pub fn emit_post_dispatch(addr: Addr, metadata: HexBinary, message: HexBinary) -> Event {
    Event::new("domain_routing_hook_post_dispatch")
        .add_attribute("addr", addr.to_string())
        .add_attribute("metadata", metadata.to_string())
        .add_attribute("message", message.to_string())
}
