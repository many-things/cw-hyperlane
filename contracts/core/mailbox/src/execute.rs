use cosmwasm_std::{
    ensure, ensure_eq, to_binary, DepsMut, HexBinary, MessageInfo, Response, WasmMsg,
};
use hpl_interface::{
    core::{mailbox::DispatchResponse, HandleMsg},
    hook::PostDispatchMsg,
    types::{bech32_to_h256, message::Message},
};

use crate::{
    contract_querier::{ism_verify, recipient_ism},
    event::{
        emit_default_hook_set, emit_default_ism_set, emit_dispatch, emit_dispatch_id, emit_process,
        emit_process_id,
    },
    state::{Delivery, CONFIG, DELIVERIES, LATEST_DISPATCHED_ID, NONCE},
    ContractError, MAILBOX_VERSION,
};

pub fn set_default_ism(
    deps: DepsMut,
    info: MessageInfo,
    new_default_ism: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        hpl_ownable::get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    let new_default_ism = deps.api.addr_validate(&new_default_ism)?;
    let event = emit_default_ism_set(info.sender, new_default_ism.clone());

    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        config.default_ism = Some(new_default_ism);

        Ok(config)
    })?;

    Ok(Response::new().add_event(event))
}

pub fn set_default_hook(
    deps: DepsMut,
    info: MessageInfo,
    new_default_hook: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        hpl_ownable::get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    let new_default_hook = deps.api.addr_validate(&new_default_hook)?;
    let event = emit_default_hook_set(info.sender, new_default_hook.clone());

    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        config.default_hook = Some(new_default_hook);

        Ok(config)
    })?;

    Ok(Response::new().add_event(event))
}

pub fn dispatch(
    deps: DepsMut,
    info: MessageInfo,
    dest_domain: u32,
    recipient_addr: HexBinary,
    msg_body: HexBinary,
    hook: Option<String>,
    metadata: Option<HexBinary>,
) -> Result<Response, ContractError> {
    ensure!(
        recipient_addr.len() <= 32,
        ContractError::InvalidAddressLength {
            len: recipient_addr.len()
        }
    );

    let config = CONFIG.load(deps.storage)?;
    let nonce = NONCE.load(deps.storage)?;
    let sender = bech32_to_h256(info.sender.as_str())?;

    let msg = Message {
        version: MAILBOX_VERSION,
        nonce,
        origin_domain: config.local_domain,
        sender: sender.to_vec().into(),
        dest_domain,
        recipient: recipient_addr,
        body: msg_body,
    };

    let id = msg.id();

    // effects
    NONCE.save(deps.storage, &(nonce + 1))?;
    LATEST_DISPATCHED_ID.save(deps.storage, &id.to_vec())?;

    // interaction
    let default_hook = config.get_default_hook();
    let hook = hook
        .map(|v| deps.api.addr_validate(&v))
        .transpose()?
        .unwrap_or(default_hook);

    // make message
    let wasm_msg = WasmMsg::Execute {
        contract_addr: hook.to_string(),
        msg: to_binary(
            &PostDispatchMsg {
                metadata: metadata.unwrap_or_default(),
                message: msg.clone().into(),
            }
            .wrap(),
        )?,
        funds: info.funds,
    };

    Ok(Response::new()
        .set_data(to_binary(&DispatchResponse {
            message_id: id.clone(),
        })?)
        .add_message(wasm_msg)
        .add_event(emit_dispatch_id(id))
        .add_event(emit_dispatch(msg)))
}

pub fn process(
    deps: DepsMut,
    info: MessageInfo,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let decoded_msg: Message = message.clone().into();
    let recipient = decoded_msg.recipient_addr(&config.hrp)?;

    ensure!(
        decoded_msg.recipient.len() <= 32,
        ContractError::InvalidAddressLength {
            len: decoded_msg.recipient.len()
        }
    );
    ensure_eq!(
        decoded_msg.version,
        MAILBOX_VERSION,
        ContractError::InvalidMessageVersion {
            version: decoded_msg.version
        }
    );
    ensure_eq!(
        decoded_msg.dest_domain,
        config.local_domain,
        ContractError::InvalidDestinationDomain {
            domain: decoded_msg.dest_domain
        }
    );

    let id = decoded_msg.id();
    let ism = recipient_ism(deps.as_ref(), &recipient)?;

    ensure!(
        !DELIVERIES.has(deps.storage, id.to_vec()),
        ContractError::AlreadyDeliveredMessage {}
    );

    DELIVERIES.save(
        deps.storage,
        id.to_vec(),
        &Delivery {
            sender: info.sender,
        },
    )?;

    ism_verify(&deps.querier, &ism, metadata, message)?;

    let handle_msg = WasmMsg::Execute {
        contract_addr: recipient.to_string(),
        msg: to_binary(
            &HandleMsg {
                origin: decoded_msg.origin_domain,
                sender: decoded_msg.sender.clone(),
                body: decoded_msg.body,
            }
            .wrap(),
        )?,
        funds: vec![],
    };

    Ok(Response::new().add_message(handle_msg).add_events(vec![
        emit_process_id(id),
        emit_process(
            config.local_domain,
            decoded_msg.sender,
            decoded_msg.recipient,
        ),
    ]))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr,
    };

    use super::*;

    const OWNER: &str = "owner";
    const NOT_OWNER: &str = "not_owner";

    const DEST_DOMAIN: u32 = 11155111;
    const RECIPIENT: &str = "b75d7d24e428c7859440498efe7caa3997cefb08c99bdd581b6b1f9f866096f0";
    const MSG: &str = "48656c6c6f21";
    const METADATA: &str = "48656c6c6f21";

    fn hex(v: &str) -> HexBinary {
        HexBinary::from_hex(v).unwrap()
    }

    #[test]
    fn test_set_default_ism() {
        let mut deps = mock_dependencies();

        let owner = Addr::unchecked(OWNER);

        hpl_ownable::initialize(deps.as_mut().storage, &owner).unwrap();

        // Sender is not authorized
        let sender = NOT_OWNER;
        let info = mock_info(sender, &[]);
        let new_default_ism = "new_default_ism".to_string();

        let owner_assert = set_default_ism(deps.as_mut(), info, new_default_ism).unwrap_err();

        assert!(matches!(owner_assert, ContractError::Unauthorized {}));
    }

    #[test]
    fn test_set_default_hook() {
        let mut deps = mock_dependencies();

        let owner = Addr::unchecked(OWNER);

        hpl_ownable::initialize(deps.as_mut().storage, &owner).unwrap();

        // Sender is not authorized
        let sender = NOT_OWNER;
        let info = mock_info(sender, &[]);
        let new_default_hook = "new_default_hook".to_string();

        let owner_assert = set_default_hook(deps.as_mut(), info, new_default_hook).unwrap_err();

        assert!(matches!(owner_assert, ContractError::Unauthorized {}));
    }

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
            hex(MSG),
            None,
            None,
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

        let sender = mock_info("sender", &[]);

        // Invalid message version
        let wrong_version_message: HexBinary = HexBinary::from(Message {
            version: 99,
            nonce: 2,
            origin_domain: 3,
            sender: hex("000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f16"),
            dest_domain: 11155111,
            recipient: hex(RECIPIENT),
            body: hex("48656c6c6f21"),
        });
        let wrong_decoded_message: Message = wrong_version_message.clone().into();
        let invalid_message_version_assert =
            process(deps.as_mut(), sender, hex(METADATA), wrong_version_message).unwrap_err();

        assert_eq!(
            invalid_message_version_assert,
            ContractError::InvalidMessageVersion {
                version: wrong_decoded_message.version
            }
        );
    }
}
