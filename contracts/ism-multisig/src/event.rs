use cosmwasm_std::{Addr, Event};

pub fn emit_init_transfer_ownership(next_owner: String) -> Event {
    Event::new("ism_multisig_init_transfer_ownership").add_attribute("next_owner", next_owner)
}

pub fn emit_finish_transfer_ownership(owner: Addr) -> Event {
    Event::new("ism_multisig_finish_transfer_owner").add_attribute("owner", owner)
}

pub fn emit_revoke_transfer_ownership() -> Event {
    Event::new("ism_multisig_revoke_transfer_ownership")
}
