use cosmwasm_std::{Addr, Event, HexBinary};
use hpl_interface::hook::HookConfig;

pub fn emit_set_hook(destination: u32, hook: Addr) -> Event {
    Event::new("domain-routing-hook-set-hook")
        .add_attribute("destination", destination.to_string())
        .add_attribute("hook", hook.to_string())
}

pub fn emit_set_hooks(hooks: Vec<HookConfig>) -> Event {
    Event::new("domain_rouitng_hook_set_hooks")
        .add_attribute("hooks", serde_json_wasm::to_string(&hooks).unwrap())
}

pub fn emit_post_dispatch(addr: Addr, metadata: HexBinary, message: HexBinary) -> Event {
    Event::new("domain-routing-hook-post-dispatch")
        .add_attribute("addr", addr.to_string())
        .add_attribute(
            "metadata",
            if metadata.is_empty() {
                "0x".to_string()
            } else {
                metadata.to_string()
            },
        )
        .add_attribute("message", message.to_string())
}
