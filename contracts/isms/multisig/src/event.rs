use cosmwasm_std::Event;

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
