use cosmwasm_std::{
    coin, ensure, ensure_eq, to_binary, wasm_execute, Coin, DepsMut, HexBinary, MessageInfo,
    QuerierWrapper, Response,
};
use cw_utils::PaymentError;
use hpl_interface::{
    core::{
        mailbox::{DispatchMsg, DispatchResponse},
        HandleMsg,
    },
    hook::{post_dispatch, quote_dispatch},
    ism,
    types::Message,
};

use hpl_ownable::get_owner;

use crate::{
    event::{
        emit_default_hook_set, emit_default_ism_set, emit_dispatch, emit_dispatch_id, emit_process,
        emit_process_id, emit_required_hook_set,
    },
    state::{Delivery, CONFIG, DELIVERIES, LATEST_DISPATCHED_ID, NONCE},
    ContractError, MAILBOX_VERSION,
};

fn get_required_value(
    querier: &QuerierWrapper,
    info: &MessageInfo,
    hook: impl Into<String>,
    metadata: HexBinary,
    msg_body: HexBinary,
) -> Result<(Coin, Coin), ContractError> {
    let required_value = quote_dispatch(querier, hook, metadata, msg_body)?
        .gas_amount
        .expect("should receive valida gas amount");

    match info.funds.len() {
        0 => Ok((coin(0u128, &required_value.denom), required_value)),
        1 => {
            let gas = &info.funds[0];
            ensure_eq!(
                gas.denom,
                required_value.denom,
                PaymentError::ExtraDenom(gas.denom.clone())
            );
            Ok((
                gas.clone(),
                coin(
                    gas.amount.min(required_value.amount).u128(),
                    required_value.denom,
                ),
            ))
        }
        _ => Err(PaymentError::MultipleDenoms {}.into()),
    }
}

pub fn set_default_ism(
    deps: DepsMut,
    info: MessageInfo,
    new_default_ism: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        get_owner(deps.storage)?,
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
        get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    let new_default_hook = deps.api.addr_validate(&new_default_hook)?;
    let event = emit_default_hook_set(info.sender, new_default_hook.clone());

    CONFIG.update::<_, ContractError>(deps.storage, |mut config| {
        config.default_hook = Some(new_default_hook);

        Ok(config)
    })?;

    Ok(Response::new().add_event(event))
}

pub fn set_required_hook(
    deps: DepsMut,
    info: MessageInfo,
    new_required_hook: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    let new_required_hook = deps.api.addr_validate(&new_required_hook)?;
    let event = emit_required_hook_set(info.sender, new_required_hook.clone());

    CONFIG.update::<_, ContractError>(deps.storage, |mut config| {
        config.required_hook = Some(new_required_hook);

        Ok(config)
    })?;

    Ok(Response::new().add_event(event))
}

pub fn dispatch(
    deps: DepsMut,
    info: MessageInfo,
    dispatch_msg: DispatchMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let nonce = NONCE.load(deps.storage)?;

    ensure!(
        dispatch_msg.recipient_addr.len() <= 32,
        ContractError::InvalidAddressLength {
            len: dispatch_msg.recipient_addr.len()
        }
    );

    // calculate gas
    let required_hook = config.required_hook.expect("required_hook not set");
    let (received_value, required_value) = get_required_value(
        &deps.querier,
        &info,
        required_hook.as_str(),
        dispatch_msg.metadata.clone().unwrap_or_default(),
        dispatch_msg.msg_body.clone(),
    )?;

    // interaction
    let hook = dispatch_msg
        .get_hook_addr(deps.api, config.default_hook)?
        .expect("default_hook not set");
    let hook_metadata = dispatch_msg.metadata.clone().unwrap_or_default();

    let msg = dispatch_msg.to_msg(MAILBOX_VERSION, nonce, config.local_domain, &info.sender)?;
    let msg_id = msg.id();

    // effects
    NONCE.save(deps.storage, &(nonce + 1))?;
    LATEST_DISPATCHED_ID.save(deps.storage, &msg_id.to_vec())?;

    // make message
    let post_dispatch_msgs = vec![
        post_dispatch(
            required_hook,
            hook_metadata.clone(),
            msg.clone(),
            Some(vec![required_value.clone()]),
        )?,
        post_dispatch(
            hook,
            hook_metadata,
            msg.clone(),
            Some(vec![coin(
                (received_value.amount - required_value.amount).u128(),
                required_value.denom,
            )]),
        )?,
    ];

    Ok(Response::new()
        .add_event(emit_dispatch_id(msg_id.clone()))
        .add_event(emit_dispatch(msg))
        .set_data(to_binary(&DispatchResponse { message_id: msg_id })?)
        .add_messages(post_dispatch_msgs))
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
    let ism = ism::recipient(&deps.querier, &recipient)?.unwrap_or(config.get_default_ism());

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

    ensure!(
        ism::verify(&deps.querier, ism, metadata, message)?,
        ContractError::VerifyFailed {}
    );

    let handle_msg = wasm_execute(
        recipient,
        &HandleMsg {
            origin: decoded_msg.origin_domain,
            sender: decoded_msg.sender.clone(),
            body: decoded_msg.body,
        }
        .wrap(),
        vec![],
    )?;

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
        Addr, QuerierResult, SystemResult, WasmQuery,
    };

    use hpl_interface::types::bech32_encode;
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::rstest;

    use super::*;

    use crate::state::Config;

    const OWNER: &str = "owner";
    const NOT_OWNER: &str = "not_owner";

    const LOCAL_DOMAIN: u32 = 26657;
    const DEST_DOMAIN: u32 = 11155111;

    #[rstest]
    #[case(addr(OWNER), addr("default_ism"))]
    #[should_panic(expected = "unauthorized")]
    #[case(addr(NOT_OWNER), addr("default_ism"))]
    fn test_set_default_ism(#[case] sender: Addr, #[case] new_default_ism: Addr) {
        let mut deps = mock_dependencies();

        CONFIG
            .save(deps.as_mut().storage, &Default::default())
            .unwrap();

        hpl_ownable::initialize(deps.as_mut().storage, &addr(OWNER)).unwrap();

        let res = set_default_ism(
            deps.as_mut(),
            mock_info(sender.as_str(), &[]),
            new_default_ism.to_string(),
        )
        .unwrap();

        assert_eq!(
            res,
            Response::new().add_event(emit_default_ism_set(sender, new_default_ism))
        );
    }

    #[rstest]
    #[case(addr(OWNER), addr("default_hook"))]
    #[should_panic(expected = "unauthorized")]
    #[case(addr(NOT_OWNER), addr("default_hook"))]
    fn test_set_default_hook(#[case] sender: Addr, #[case] new_default_hook: Addr) {
        let mut deps = mock_dependencies();

        CONFIG
            .save(deps.as_mut().storage, &Default::default())
            .unwrap();

        hpl_ownable::initialize(deps.as_mut().storage, &addr(OWNER)).unwrap();

        let res = set_default_hook(
            deps.as_mut(),
            mock_info(sender.as_str(), &[]),
            new_default_hook.to_string(),
        )
        .unwrap();

        assert_eq!(
            res,
            Response::new().add_event(emit_default_hook_set(sender, new_default_hook))
        )
    }

    #[rstest]
    #[case(DEST_DOMAIN, gen_bz(20), gen_bz(32))]
    #[should_panic(expected = "invalid address length: 33")]
    #[case(DEST_DOMAIN, gen_bz(20), gen_bz(33))]
    fn test_dispatch(
        #[values("osmo", "neutron")] hrp: &str,
        #[case] dest_domain: u32,
        #[case] sender: HexBinary,
        #[case] recipient_addr: HexBinary,
    ) {
        let sender = bech32_encode(hrp, sender.as_slice()).unwrap();
        let msg_body = gen_bz(123);

        let mut deps = mock_dependencies();

        CONFIG
            .save(
                deps.as_mut().storage,
                &Config::new(hrp, LOCAL_DOMAIN)
                    .with_hook(addr("default_hook"))
                    .with_ism(addr("default_ism")),
            )
            .unwrap();
        NONCE.save(deps.as_mut().storage, &0u32).unwrap();

        hpl_ownable::initialize(deps.as_mut().storage, &addr(OWNER)).unwrap();

        let dispatch_msg = DispatchMsg::new(dest_domain, recipient_addr, msg_body);
        let msg = dispatch_msg
            .clone()
            .to_msg(
                MAILBOX_VERSION,
                NONCE.load(deps.as_ref().storage).unwrap(),
                LOCAL_DOMAIN,
                &sender,
            )
            .unwrap();

        let _res = dispatch(deps.as_mut(), mock_info(sender.as_str(), &[]), dispatch_msg).unwrap();

        assert_eq!(NONCE.load(deps.as_ref().storage).unwrap(), 1u32);
        assert_eq!(
            LATEST_DISPATCHED_ID.load(deps.as_ref().storage).unwrap(),
            msg.id().to_vec()
        );
    }

    fn test_process_query_handler(query: &WasmQuery) -> QuerierResult {
        match query {
            WasmQuery::Smart { contract_addr, msg } => {
                if let Ok(req) = cosmwasm_std::from_binary::<ism::ISMSpecifierQueryMsg>(msg) {
                    match req {
                        ism::ISMSpecifierQueryMsg::InterchainSecurityModule() => {
                            return SystemResult::Ok(
                                cosmwasm_std::to_binary(&ism::InterchainSecurityModuleResponse {
                                    ism: Some(addr("default_ism")),
                                })
                                .into(),
                            );
                        }
                    }
                }

                if let Ok(req) = cosmwasm_std::from_binary::<ism::ISMQueryMsg>(msg) {
                    assert_eq!(contract_addr, &addr("default_ism"));

                    match req {
                        ism::ISMQueryMsg::Verify { metadata, .. } => {
                            return SystemResult::Ok(
                                cosmwasm_std::to_binary(&ism::VerifyResponse {
                                    verified: metadata[0] == 1,
                                })
                                .into(),
                            );
                        }
                        _ => unreachable!("not in test coverage"),
                    }
                }

                unreachable!("not in test coverage")
            }
            _ => unimplemented!("only for smart query"),
        }
    }

    #[rstest]
    #[case(MAILBOX_VERSION, LOCAL_DOMAIN, gen_bz(32), false, true)]
    #[should_panic(expected = "invalid message version: 99")]
    #[case(99, LOCAL_DOMAIN, gen_bz(32), false, true)]
    #[should_panic(expected = "message already delivered")]
    #[case(MAILBOX_VERSION, LOCAL_DOMAIN, gen_bz(32), true, true)]
    #[should_panic(expected = "invalid destination domain: 11155111")]
    #[case(MAILBOX_VERSION, DEST_DOMAIN, gen_bz(32), false, true)]
    #[should_panic(expected = "ism verify failed")]
    #[case(MAILBOX_VERSION, LOCAL_DOMAIN, gen_bz(32), false, false)]
    fn test_process_revised(
        #[values("osmo", "neutron")] hrp: &str,
        #[case] version: u8,
        #[case] dest_domain: u32,
        #[case] recipient_addr: HexBinary,
        #[case] duplicate: bool,
        #[case] verified: bool,
    ) {
        let sender = gen_bz(32);
        let sender_addr = bech32_encode(hrp, &sender).unwrap();
        let msg_body = gen_bz(123);

        let mut deps = mock_dependencies();

        deps.querier.update_wasm(test_process_query_handler);

        CONFIG
            .save(
                deps.as_mut().storage,
                &Config::new(hrp, LOCAL_DOMAIN)
                    .with_hook(addr("default_hook"))
                    .with_ism(addr("default_ism")),
            )
            .unwrap();

        let msg = Message {
            version,
            nonce: 123,
            origin_domain: DEST_DOMAIN,
            sender,
            dest_domain,
            recipient: recipient_addr,
            body: msg_body,
        };
        let msg_id = msg.id();

        if duplicate {
            DELIVERIES
                .save(
                    deps.as_mut().storage,
                    msg.id().to_vec(),
                    &Delivery {
                        sender: sender_addr.clone(),
                    },
                )
                .unwrap();
        }

        let _res = process(
            deps.as_mut(),
            mock_info(sender_addr.as_str(), &[]),
            vec![verified.into()].into(),
            msg.into(),
        )
        .map_err(|v| v.to_string())
        .unwrap();

        let delivery = DELIVERIES
            .load(deps.as_ref().storage, msg_id.to_vec())
            .unwrap();
        assert_eq!(delivery.sender, sender_addr);
    }
}
