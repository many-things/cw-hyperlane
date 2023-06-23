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

pub fn emit_enroll_validator(domain: u32, validator: String) -> Event {
    Event::new("ism_multisig_enroll_validator")
        .add_attribute("domain", domain.to_string())
        .add_attribute("validator", validator)
}

pub fn emit_unenroll_validator(domain: u32, validator: String) -> Event {
    Event::new("ism_multisig_unenroll_validator")
        .add_attribute("domain", domain.to_string())
        .add_attribute("validator", validator)
}

pub fn emit_set_threshold(domain: u32, threshold: u8) -> Event {
    Event::new("ism_multisig_set_threshold")
        .add_attribute("domain", domain.to_string())
        .add_attribute("threshold", threshold.to_string())
}
