use cosmwasm_std::{Addr, Coin, Event, HexBinary, Uint128, Uint256};

pub fn emit_set_default_gas(owner: Addr, default_gas: u128) -> Event {
    Event::new("igp-core-set-default-gas")
        .add_attribute("owner", owner)
        .add_attribute("default-gas", default_gas.to_string())
}

pub fn emit_set_gas_for_domain(owner: Addr, gas_for_domain: Vec<(u32, u128)>) -> Event {
    Event::new("igp-core-set-gas-for-domain")
        .add_attribute("owner", owner)
        .add_attribute(
            "domains",
            gas_for_domain
                .into_iter()
                .map(|v| v.0.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
}

pub fn emit_unset_gas_for_domain(owner: Addr, domains: Vec<u32>) -> Event {
    Event::new("igp-core-unset-gas-for-domain")
        .add_attribute("owner", owner)
        .add_attribute(
            "domains",
            domains
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
}

pub fn emit_set_beneficiary(owner: Addr, beneficiary: String) -> Event {
    Event::new("igp-core-set-beneficiary")
        .add_attribute("owner", owner)
        .add_attribute("beneficiary", beneficiary)
}

pub fn emit_claim(beneficiary: Addr, balance: Coin) -> Event {
    Event::new("igp-core-claim")
        .add_attribute("beneficiary", beneficiary)
        .add_attribute("collected", balance.to_string())
}

pub fn emit_post_dispatch(metadata: HexBinary, message: HexBinary) -> Event {
    Event::new("igp-core-post-dispatch")
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

pub fn emit_pay_for_gas(
    sender: Addr,
    dest_domain: u32,
    message_id: HexBinary,
    gas_amount: Uint256,
    gas_refunded: Uint128,
    gas_required: Uint256,
    payment: Uint256,
) -> Event {
    Event::new("igp-core-pay-for-gas")
        .add_attribute("sender", sender)
        .add_attribute("dest_domain", dest_domain.to_string())
        .add_attribute("message_id", message_id.to_hex())
        .add_attribute("gas_amount", gas_amount)
        .add_attribute("gas_refunded", gas_refunded)
        .add_attribute("gas_required", gas_required)
        .add_attribute("payment", payment)
}
