use cosmwasm_std::{Addr, Event};

pub fn emit_instantiated(creator: Addr) -> Event {
    Event::new("hub_instantiate_mailbox").add_attribute("creator", creator)
}
