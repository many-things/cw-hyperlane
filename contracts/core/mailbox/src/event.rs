use cosmwasm_std::{Addr, Binary, Event, HexBinary};
use hpl_interface::types::message::Message;

pub fn emit_instantiated(owner: Addr) -> Event {
    Event::new("mailbox_instantiated").add_attribute("owner", owner)
}

pub fn emit_paused(owner: Addr) -> Event {
    Event::new("mailbox_paused").add_attribute("owner", owner)
}

pub fn emit_unpaused(owner: Addr) -> Event {
    Event::new("mailbox_unpaused").add_attribute("owner", owner)
}

pub fn emit_default_ism_changed(owner: Addr, new_default_ism: Addr) -> Event {
    Event::new("mailbox_default_ism_changed")
        .add_attribute("owner", owner)
        .add_attribute("new_default_ism", new_default_ism)
}

pub fn emit_dispatch_id(id: Binary) -> Event {
    Event::new("mailbox_dispatch_id").add_attribute("message_id", HexBinary::from(id).to_hex())
}

// we only emit these redundantly in the ethereum mailbox because of "indexed" fields
pub fn emit_dispatch(msg: Message) -> Event {
    Event::new("mailbox_dispatch")
        .add_attribute("sender", HexBinary::from(msg.sender.clone()).to_hex())
        .add_attribute("destination", msg.dest_domain.to_string())
        .add_attribute("recipient", HexBinary::from(msg.recipient.clone()).to_hex())
        .add_attribute("message", HexBinary::from(msg).to_hex())
}

pub fn emit_process_id(id: Binary) -> Event {
    Event::new("mailbox_process_id").add_attribute("message_id", HexBinary::from(id).to_hex())
}

pub fn emit_process(origin: u32, sender: Binary, recipient: Binary) -> Event {
    Event::new("mailbox_process")
        .add_attribute("origin", format!("{origin}"))
        .add_attribute("sender", HexBinary::from(sender).to_hex())
        .add_attribute("recipient", HexBinary::from(recipient).to_hex())
}

pub fn emit_default_hook_changed(owner: Addr, new_default_ism: Addr) -> Event {
    Event::new("mailbox_default_hook_changed")
        .add_attribute("owner", owner.to_string())
        .add_attribute("new_default_hook", new_default_ism.to_string())
}
