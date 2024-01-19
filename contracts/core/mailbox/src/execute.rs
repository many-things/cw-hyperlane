use cosmwasm_std::{
    ensure, ensure_eq, to_json_binary, wasm_execute, Coin, Coins, DepsMut, Env, HexBinary,
    MessageInfo, Response
};
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
        dispatch_msg.recipient_addr.len() == 32,
        ContractError::InvalidAddressLength {
            len: dispatch_msg.recipient_addr.len()
        }
    );

    // build hyperlane message
    let msg =
        dispatch_msg
            .clone()
            .to_msg(MAILBOX_VERSION, nonce, config.local_domain, &info.sender)?;
    let metadata = dispatch_msg.clone().metadata.unwrap_or_default();
    let hook = dispatch_msg.get_hook_addr(deps.api, config.get_default_hook())?;

    // assert gas received satisfies required gas
    let required_hook = config.get_required_hook();
    let required_hook_fees: Vec<Coin> =
        quote_dispatch(&deps.querier, &required_hook, metadata.clone(), msg.clone())?.fees;

    let mut funds = Coins::try_from(info.funds.clone())?;
    for coin in required_hook_fees.iter() {
        if let Err(_) = funds.sub(coin.clone()) {
            return Err(ContractError::HookPayment {
                wanted: required_hook_fees,
                received: info.funds,
            });
        }
    }

    // commit to message
    let msg_id = msg.id();
    NONCE.save(deps.storage, &(nonce + 1))?;
    LATEST_DISPATCHED_ID.save(deps.storage, &msg_id.to_vec())?;

    // build post dispatch calls
    let post_dispatch_msgs = vec![
        post_dispatch(
            required_hook,
            metadata.clone(),
            msg.clone(),
            Some(required_hook_fees),
        )?,
        post_dispatch(hook, metadata, msg.clone(), Some(funds.to_vec()))?,
    ];

    Ok(Response::new()
        .add_event(emit_dispatch_id(msg_id.clone()))
        .add_event(emit_dispatch(msg))
        .set_data(to_json_binary(&DispatchResponse { message_id: msg_id })?)
        .add_messages(post_dispatch_msgs))
}

pub fn process(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let decoded_msg: Message = message.into();
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
            block_number: env.block.height,
        },
    )?;

    let verify = ism::verify(&deps.querier, ism, metadata, decoded_msg.clone().into())?;

    deps.api
        .debug(&format!("mailbox::process: verify: {}", verify));

    ensure!(verify, ContractError::VerifyFailed {});

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
    use std::collections::HashMap;

    use cosmwasm_std::{
        coin, from_json,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_json_binary, Addr, ContractResult, CosmosMsg, OwnedDeps, QuerierResult,
        SystemResult, WasmMsg, WasmQuery,
    };

    use hpl_interface::{
        core::mailbox::InstantiateMsg,
        hook::{ExpectedHookQueryMsg, HookQueryMsg, PostDispatchMsg, QuoteDispatchResponse},
        ism::IsmQueryMsg,
        types::bech32_encode,
    };
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::{fixture, rstest};

    use super::*;

    use crate::{contract::instantiate, state::Config};

    const OWNER: &str = "owner";
    const NOT_OWNER: &str = "not_owner";

    const LOCAL_DOMAIN: u32 = 26657;
    const DEST_DOMAIN: u32 = 11155111;

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    fn mock_query_handler(
        req: &WasmQuery,
        addr_fees: &Option<HashMap<String, Vec<Coin>>>,
    ) -> QuerierResult {
        let (req, addr) = match req {
            WasmQuery::Smart { msg, contract_addr } => (from_json(msg).unwrap(), contract_addr),
            _ => unreachable!("wrong query type"),
        };

        let req = match req {
            ExpectedHookQueryMsg::Hook(HookQueryMsg::QuoteDispatch(msg)) => msg,
            _ => unreachable!("wrong query type"),
        };

        let mut fees = match addr_fees {
            Some(fees) => fees.get(addr).unwrap_or(&vec![]).clone(),
            None => vec![],
        };

        if !req.metadata.is_empty() {
            let parsed_fee = u32::from_be_bytes(req.metadata.as_slice().try_into().unwrap());

            fees = vec![coin(parsed_fee as u128, "utest")];
        }

        let res = QuoteDispatchResponse { fees };
        let res = to_json_binary(&res).unwrap();

        SystemResult::Ok(ContractResult::Ok(res))
    }

    #[fixture]
    fn deps(#[default("deployer")] sender: &str) -> TestDeps {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender, &[]),
            InstantiateMsg {
                hrp: "osmo".to_string(),
                owner: OWNER.to_string(),
                domain: LOCAL_DOMAIN,
            },
        )
        .unwrap();

        deps
    }

    #[rstest]
    fn test_init(deps: TestDeps) {
        let config = CONFIG.load(deps.as_ref().storage).unwrap();

        assert_eq!(config.hrp, "osmo");
        assert_eq!(config.local_domain, LOCAL_DOMAIN);
        assert_eq!(config.default_ism, None);
        assert_eq!(config.default_hook, None);
        assert_eq!(config.required_hook, None);

        let nonce = NONCE.load(deps.as_ref().storage).unwrap();
        assert_eq!(nonce, 0u32);
    }

    #[rstest]
    #[case(addr(OWNER), addr("default_ism"))]
    #[should_panic(expected = "unauthorized")]
    #[case(addr(NOT_OWNER), addr("default_ism"))]
    fn test_set_default_ism(
        mut deps: TestDeps,
        #[case] sender: Addr,
        #[case] new_default_ism: Addr,
    ) {
        let res = set_default_ism(
            deps.as_mut(),
            mock_info(sender.as_str(), &[]),
            new_default_ism.to_string(),
        )
        .map_err(|e| e.to_string())
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
    fn test_set_default_hook(
        mut deps: TestDeps,
        #[case] sender: Addr,
        #[case] new_default_hook: Addr,
    ) {
        let res = set_default_hook(
            deps.as_mut(),
            mock_info(sender.as_str(), &[]),
            new_default_hook.to_string(),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        assert_eq!(
            res,
            Response::new().add_event(emit_default_hook_set(sender, new_default_hook))
        )
    }

    #[rstest]
    #[case(addr(OWNER), addr("required_hook"))]
    #[should_panic(expected = "unauthorized")]
    #[case(addr(NOT_OWNER), addr("required_hook"))]
    fn test_set_required_hook(
        mut deps: TestDeps,
        #[case] sender: Addr,
        #[case] new_required_hook: Addr,
    ) {
        let res = set_required_hook(
            deps.as_mut(),
            mock_info(sender.as_str(), &[]),
            new_required_hook.to_string(),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        assert_eq!(
            res,
            Response::new().add_event(emit_required_hook_set(sender, new_required_hook))
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

        deps.querier.update_wasm(|q| mock_query_handler(q, &None));

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER, &[]),
            InstantiateMsg {
                hrp: hrp.to_string(),
                owner: OWNER.to_string(),
                domain: LOCAL_DOMAIN,
            },
        )
        .unwrap();

        set_default_ism(deps.as_mut(), mock_info(OWNER, &[]), "default_ism".into()).unwrap();
        set_default_hook(deps.as_mut(), mock_info(OWNER, &[]), "default_hook".into()).unwrap();
        set_required_hook(deps.as_mut(), mock_info(OWNER, &[]), "required_hook".into()).unwrap();

        let dispatch_msg = DispatchMsg::new(dest_domain, recipient_addr, msg_body)
            .with_metadata(1500u32.to_be_bytes().to_vec());

        let msg = dispatch_msg
            .clone()
            .to_msg(
                MAILBOX_VERSION,
                NONCE.load(deps.as_ref().storage).unwrap(),
                LOCAL_DOMAIN,
                &sender,
            )
            .unwrap();

        let _res = dispatch(
            deps.as_mut(),
            mock_info(sender.as_str(), &[coin(1500, "utest")]),
            dispatch_msg,
        )
        .map_err(|e| e.to_string())
        .unwrap();

        assert_eq!(NONCE.load(deps.as_ref().storage).unwrap(), 1u32);
        assert_eq!(
            LATEST_DISPATCHED_ID.load(deps.as_ref().storage).unwrap(),
            msg.id().to_vec()
        );
    }

    #[rstest]
    #[case(vec![coin(100, "usd")], vec![coin(100, "usd")])]
    #[should_panic]
    #[case(vec![coin(100, "usd")], vec![coin(50, "usd")])]
    #[should_panic]
    #[case(vec![coin(100, "usdt")], vec![coin(100, "usd")])]
    #[case(vec![coin(50, "usd")], vec![coin(100, "usd")])]
    fn test_post_dispatch(#[case] required_hook_fees: Vec<Coin>, #[case] funds: Vec<Coin>) {
        let mut deps = mock_dependencies();

        let mut hook_fees = HashMap::new();
        hook_fees.insert("required_hook".into(), required_hook_fees.clone());

        // not enforced by mailbox
        // hook_fees.insert("default_hook".into(), default_hook_fees);

        let opt = Some(hook_fees);

        deps.querier
            .update_wasm(move |q| mock_query_handler(q, &opt));

        let hrp = "osmo";

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER, &[]),
            InstantiateMsg {
                hrp: "osmo".to_string(),
                owner: OWNER.to_string(),
                domain: LOCAL_DOMAIN,
            },
        )
        .unwrap();

        set_default_hook(deps.as_mut(), mock_info(OWNER, &[]), "default_hook".into()).unwrap();
        set_required_hook(deps.as_mut(), mock_info(OWNER, &[]), "required_hook".into()).unwrap();

        let dispatch_msg = DispatchMsg::new(DEST_DOMAIN, gen_bz(32), gen_bz(123));

        let sender = bech32_encode(hrp, gen_bz(32).as_slice()).unwrap();

        let msg = dispatch_msg
            .clone()
            .to_msg(
                MAILBOX_VERSION,
                NONCE.load(deps.as_ref().storage).unwrap(),
                LOCAL_DOMAIN,
                &sender,
            )
            .unwrap();

        let post_dispatch_msg = to_json_binary(
            &PostDispatchMsg {
                metadata: HexBinary::default(),
                message: msg.into(),
            }
            .wrap(), // not sure why I need this
        )
        .unwrap();

        let res = dispatch(
            deps.as_mut(),
            mock_info(sender.as_str(), &funds),
            dispatch_msg.clone(),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        let msgs: Vec<_> = res.messages.into_iter().map(|v| v.msg).collect();

        assert_eq!(
            msgs[0],
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "required_hook".to_string(),
                msg: post_dispatch_msg.clone(),
                funds: required_hook_fees.clone()
            },)
        );

        // subtract required_hook_fees from funds
        let mut remaining_funds = Coins::try_from(funds).unwrap();
        for coin in required_hook_fees {
            remaining_funds.sub(coin).unwrap();
        }

        assert_eq!(
            msgs[1],
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "default_hook".to_string(),
                msg: post_dispatch_msg,
                funds: remaining_funds.into_vec() // forward all remaining funds
            })
        );
    }

    fn test_process_query_handler(query: &WasmQuery) -> QuerierResult {
        match query {
            WasmQuery::Smart { contract_addr, msg } => {
                if let Ok(req) = cosmwasm_std::from_json::<ism::ExpectedIsmSpecifierQueryMsg>(msg) {
                    match req {
                        hpl_interface::ism::ExpectedIsmSpecifierQueryMsg::IsmSpecifier(
                            ism::IsmSpecifierQueryMsg::InterchainSecurityModule(),
                        ) => {
                            return SystemResult::Ok(
                                to_json_binary(&ism::InterchainSecurityModuleResponse {
                                    ism: Some(addr("default_ism")),
                                })
                                .into(),
                            );
                        }
                    }
                }

                if let Ok(req) = cosmwasm_std::from_json::<ism::ExpectedIsmQueryMsg>(msg) {
                    assert_eq!(contract_addr, &addr("default_ism"));

                    match req {
                        ism::ExpectedIsmQueryMsg::Ism(IsmQueryMsg::Verify { metadata, .. }) => {
                            return SystemResult::Ok(
                                to_json_binary(&ism::VerifyResponse {
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
    fn test_process(
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
                    .with_hook(addr("default_hook"), addr("required_hook"))
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
                        block_number: mock_env().block.height,
                    },
                )
                .unwrap();
        }

        let _res = process(
            deps.as_mut(),
            mock_env(),
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
