use cosmwasm_std::{Addr, Event};

pub fn emit_instantiated(creator: Addr) -> Event {
    Event::new("mailbox_instantiated").add_attribute("creator", creator)
}
