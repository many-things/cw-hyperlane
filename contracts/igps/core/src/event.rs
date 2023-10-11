use cosmwasm_std::{Addr, Binary, Coin, Event, Uint128, Uint256};

pub fn emit_set_gas_oracles(owner: Addr, domains: Vec<String>) -> Event {
    Event::new("igp-core-set-gas-oracle")
        .add_attribute("owner", owner)
        .add_attribute("domains", domains.join(","))
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

pub fn emit_post_dispatch(metadata: Binary, message: Binary) -> Event {
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
    message_id: Binary,
    gas_amount: Uint256,
    gas_refunded: Uint128,
    gas_required: Uint256,
    payment: Uint256,
) -> Event {
    Event::new("igp-core-pay-for-gas")
        .add_attribute("sender", sender)
        .add_attribute("message_id", message_id.to_base64())
        .add_attribute("gas_amount", gas_amount)
        .add_attribute("gas_refunded", gas_refunded)
        .add_attribute("gas_required", gas_required)
        .add_attribute("payment", payment)
}
