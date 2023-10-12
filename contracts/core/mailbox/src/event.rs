use cosmwasm_std::{Addr, Event, HexBinary};
use hpl_interface::types::message::Message;

pub fn emit_instantiated(owner: Addr) -> Event {
    Event::new("mailbox_instantiated").add_attribute("owner", owner)
}

pub fn emit_default_ism_set(owner: Addr, new_default_ism: Addr) -> Event {
    Event::new("mailbox_default_ism_set")
        .add_attribute("owner", owner)
        .add_attribute("new_default_ism", new_default_ism)
}

pub fn emit_default_hook_set(owner: Addr, new_default_hook: Addr) -> Event {
    Event::new("mailbox_default_hook_set")
        .add_attribute("owner", owner)
        .add_attribute("new_default_hook", new_default_hook)
}

pub fn emit_dispatch_id(id: HexBinary) -> Event {
    Event::new("mailbox_dispatch_id").add_attribute("message_id", id.to_hex())
}

pub fn emit_dispatch(msg: Message) -> Event {
    Event::new("mailbox_dispatch")
        .add_attribute("sender", msg.sender.clone().to_hex())
        .add_attribute("destination", msg.dest_domain.to_string())
        .add_attribute("recipient", msg.recipient.clone().to_hex())
        .add_attribute("message", HexBinary::from(msg).to_hex())
}

pub fn emit_process_id(id: HexBinary) -> Event {
    Event::new("mailbox_process_id").add_attribute("message_id", id.to_hex())
}

pub fn emit_process(origin: u32, sender: HexBinary, recipient: HexBinary) -> Event {
    Event::new("mailbox_process")
        .add_attribute("origin", format!("{origin}"))
        .add_attribute("sender", sender.to_hex())
        .add_attribute("recipient", recipient.to_hex())
}
