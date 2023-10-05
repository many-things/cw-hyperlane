use cosmwasm_std::{
    to_binary, Addr, Binary, DepsMut, HexBinary, MessageInfo, QuerierWrapper, Response, StdResult,
    WasmMsg,
};
use hpl_interface::{
    hub,
    mailbox::{DispatchResponse, ExpectedHandlerMsg, HandleMsg},
    types::{bech32_to_h256, message::Message},
};

use crate::{
    contract_querier::{ism_verify, recipient_ism},
    event::{emit_dispatch, emit_dispatch_id, emit_process, emit_process_id},
    state::{
        assert_addr_length, assert_destination_domain, assert_message_version, assert_undelivered,
        Delivery, CONFIG, DELIVERY, LATEST_DISPATCHED_ID, NONCE,
    },
    ContractError, MAILBOX_VERSION,
};

fn fetch_origin_domain(querier: &QuerierWrapper, factory: &Addr) -> StdResult<u32> {
    let resp: hub::OriginDomainResponse =
        querier.query_wasm_smart(factory, &hub::QueryMsg::OriginDomain {})?;

    Ok(resp.domain)
}

pub fn dispatch(
    deps: DepsMut,
    info: MessageInfo,
    dest_domain: u32,
    recipient_addr: HexBinary,
    msg_body: HexBinary,
) -> Result<Response, ContractError> {
    assert_addr_length(recipient_addr.len())?;

    let config = CONFIG.load(deps.storage)?;

    let nonce = NONCE.load(deps.storage)?;
    NONCE.save(deps.storage, &(nonce + 1))?;

    let origin_domain = fetch_origin_domain(&deps.querier, &config.factory)?;

    let sender = bech32_to_h256(info.sender.as_str())?;

    let msg = Message {
        version: MAILBOX_VERSION,
        nonce,
        origin_domain,
        sender: Binary(sender.to_vec()),
        dest_domain,
        recipient: recipient_addr.into(),
        body: msg_body.into(),
    };

    let id = msg.id();

    // Effects
    NONCE.save(deps.storage, &(nonce + 1))?;
    LATEST_DISPATCHED_ID.save(deps.storage, &id.to_vec())?;

    // INTERACTION

    let binary_msg: HexBinary = msg.clone().into();
    let wasm_msg = WasmMsg::Execute {
        contract_addr: config.default_hook.to_string(),
        msg: to_binary(
            &hpl_interface::post_dispatch_hook::PostDispatchMsg::PostDispatch {
                metadata: binary_msg.as_slice()[0..0].into(), // Should be handle without metadata?
                message: binary_msg,
            },
        )?,
        funds: info.funds,
    };

    Ok(Response::new()
        .set_data(to_binary(&DispatchResponse {
            message_id: id.clone(),
        })?)
        .add_message(wasm_msg)
        .add_events(vec![emit_dispatch_id(id), emit_dispatch(msg)]))
}

pub fn process(
    deps: DepsMut,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<Response, ContractError> {
    let decoded_msg: Message = message.clone().into();

    assert_addr_length(HexBinary::from(decoded_msg.recipient.to_vec()).len())?;
    assert_message_version(decoded_msg.version, MAILBOX_VERSION)?;

    let config = CONFIG.load(deps.storage)?;
    // FIXME: use hrp fetched from hub
    let recipient = decoded_msg.recipient_addr("osmo")?;
    let origin_domain = fetch_origin_domain(&deps.querier, &config.factory)?;

    assert_destination_domain(decoded_msg.dest_domain, origin_domain)?;

    let id = decoded_msg.id();
    let ism = recipient_ism(deps.as_ref(), &recipient)?;

    assert_undelivered(deps.storage, id.clone())?;

    DELIVERY.save(deps.storage, id.0.clone(), &Delivery { ism: ism.clone() })?;

    ism_verify(&deps.querier, &ism, metadata, message)?;

    let handle_msg = WasmMsg::Execute {
        contract_addr: recipient.to_string(),
        msg: to_binary(&ExpectedHandlerMsg::Handle(HandleMsg {
            origin: decoded_msg.origin_domain,
            sender: decoded_msg.sender.clone().into(),
            body: decoded_msg.body.into(),
        }))?,
        funds: vec![],
    };

    Ok(Response::new().add_message(handle_msg).add_events(vec![
        emit_process_id(id),
        emit_process(origin_domain, decoded_msg.sender, decoded_msg.recipient),
    ]))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_info};

    use super::*;

    const DEST_DOMAIN: u32 = 11155111;
    const RECIPIENT: &str = "b75d7d24e428c7859440498efe7caa3997cefb08c99bdd581b6b1f9f866096f0";
    const MSG: &str = "48656c6c6f21";
    const METADATA: &str = "48656c6c6f21";

    #[test]
    fn test_dispatch() {
        let mut deps = mock_dependencies();

        let long_recipient_address = HexBinary::from_hex(
            "b75d7d24e428c7859440498efe7caa3997cefb08c99bdd581b6b1f9f866096f073c8c3b0316abe",
        )
        .unwrap();

        // Invalid address length
        let invalid_address_length_assert = dispatch(
            deps.as_mut(),
            mock_info("owner", &[]),
            DEST_DOMAIN,
            long_recipient_address.clone(),
            HexBinary::from_hex(MSG).unwrap(),
        )
        .unwrap_err();

        assert_eq!(
            invalid_address_length_assert,
            ContractError::InvalidAddressLength {
                len: long_recipient_address.len()
            }
        );
    }

    #[test]
    fn test_process() {
        let mut deps = mock_dependencies();
        let hex = |v: &str| -> Binary { HexBinary::from_hex(v).unwrap().into() };

        // Invalid message version
        let wrong_version_message: HexBinary = HexBinary::from(Message {
            version: 0,
            nonce: 2,
            origin_domain: 3,
            sender: hex("000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f16"),
            dest_domain: 11155111,
            recipient: hex(RECIPIENT),
            body: hex("48656c6c6f21"),
        });
        let wrong_decoded_message: Message = wrong_version_message.clone().into();
        let invalid_message_version_assert = process(
            deps.as_mut(),
            HexBinary::from_hex(METADATA).unwrap(),
            wrong_version_message,
        )
        .unwrap_err();

        assert_eq!(
            invalid_message_version_assert,
            ContractError::InvalidMessageVersion {
                version: wrong_decoded_message.version
            }
        );
    }
}
