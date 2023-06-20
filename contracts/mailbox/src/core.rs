use cosmwasm_std::{
    to_binary, Addr, Binary, DepsMut, HexBinary, MessageInfo, QuerierWrapper, Response, StdResult,
    WasmMsg,
};
use hpl_interface::{
    hub, ism,
    mailbox::{DispatchResponse, ExpectedHandlerMsg, HandleMsg},
    types::message::Message,
};

use crate::{
    event::{emit_dispatch, emit_dispatch_id, emit_process, emit_process_id},
    state::{
        assert_addr_length, assert_already_delivered, assert_destination_domain,
        assert_message_version, assert_verify_response, CONFIG, MESSAGE_PROCESSED, MESSAGE_TREE,
        NONCE,
    },
    ContractError, MAILBOX_VERSION,
};

fn fetch_origin_domain(querier: &QuerierWrapper, factory: &Addr) -> StdResult<u32> {
    let resp: hub::OriginDomainResponse =
        querier.query_wasm_smart(factory, &hub::QueryMsg::OriginDomain {})?;

    Ok(resp.domain)
}

fn ism_verify(
    querier: &QuerierWrapper,
    default_ism: &Addr,
    receipient: &Addr,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<(), ContractError> {
    let ism_resp: ism::InterchainSecurityModuleResponse = querier.query_wasm_smart(
        receipient,
        &ism::ISMSpecifierQueryMsg::InterchainSecurityModule(),
    )?;

    let ism = ism_resp.ism.unwrap_or_else(|| default_ism.clone());

    let verify_resp: ism::VerifyResponse =
        querier.query_wasm_smart(ism, &ism::ISMQueryMsg::Verify { metadata, message })?;

    assert_verify_response(verify_resp.verified)?;

    Ok(())
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

    let msg = Message {
        version: MAILBOX_VERSION,
        nonce,
        origin_domain,
        sender: Binary(info.sender.as_bytes().to_vec()),
        dest_domain,
        recipient: recipient_addr.into(),
        body: msg_body.into(),
    };

    let id = msg.id();
    let mut tree = MESSAGE_TREE.load(deps.storage)?;
    tree.insert(id.clone());
    MESSAGE_TREE.save(deps.storage, &tree)?;

    Ok(Response::new()
        .set_data(to_binary(&DispatchResponse {
            message_id: id.clone(),
        })?)
        .add_events(vec![
            emit_dispatch_id(id),
            emit_dispatch(
                msg.sender.clone(),
                dest_domain,
                msg.recipient.clone(),
                msg.into(),
            ),
        ]))
}

pub fn process(
    deps: DepsMut,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let decoded_msg: Message = message.clone().into();

    assert_addr_length(decoded_msg.recipient.len())?;

    // FIXME: use hrp fetched from hub
    let recipient = decoded_msg.recipient_addr("osmo")?;

    let origin_domain = fetch_origin_domain(&deps.querier, &config.factory)?;

    assert_message_version(decoded_msg.version, MAILBOX_VERSION)?;

    assert_destination_domain(decoded_msg.dest_domain, origin_domain)?;

    let id = decoded_msg.id();

    assert_already_delivered(deps.storage, id.clone())?;

    MESSAGE_PROCESSED.save(deps.storage, id.0.clone(), &true)?;

    ism_verify(
        &deps.querier,
        &config.default_ism,
        &recipient,
        metadata,
        message,
    )?;

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
    #[test]
    fn test_dispatch() {}

    fn test_process() {}
}
