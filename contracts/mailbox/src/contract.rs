#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, HexBinary, MessageInfo, QuerierWrapper, Reply,
    Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use hpl_interface::{
    ism,
    mailbox::{ExecuteMsg, ExpectedHandlerMsg, HandleMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    mailbox_factory,
    types::message::Message,
};

use crate::{
    error::ContractError,
    event::{
        emit_default_ism_changed, emit_dispatch, emit_dispatch_id, emit_paused, emit_process,
        emit_process_id, emit_unpaused,
    },
    state::{Config, CONFIG, MESSAGE_PROCESSED, NONCE, PAUSE},
    CONTRACT_NAME, CONTRACT_VERSION, MAILBOX_VERSION,
};

fn fetch_origin_domain(querier: &QuerierWrapper, factory: &Addr) -> StdResult<u32> {
    let resp: mailbox_factory::OriginDomainResponse =
        querier.query_wasm_smart(factory, &mailbox_factory::QueryMsg::OriginDomain)?;

    Ok(resp.0)
}

fn ism_verify(
    querier: &QuerierWrapper,
    default_ism: &Addr,
    receipient: &Addr,
    metadata: HexBinary,
    message: HexBinary,
) -> StdResult<()> {
    let ism_resp: ism::InterchainSecurityModuleResponse =
        querier.query_wasm_smart(receipient, &ism::QueryMsg::InterchainSecurityModule())?;

    let ism = ism_resp.0.unwrap_or_else(|| default_ism.clone());

    let verify_resp: ism::VerifyResponse =
        querier.query_wasm_smart(ism, &ism::QueryMsg::Verify { metadata, message })?;

    assert!(verify_resp.0);

    Ok(())
}

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            owner: deps.api.addr_validate(&msg.owner)?,
            factory: info.sender.clone(),
            default_ism: deps.api.addr_validate(&msg.default_ism)?,
        },
    )?;

    PAUSE.save(deps.storage, &false)?;

    NONCE.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    assert!(!PAUSE.load(deps.storage)?, "paused");

    match msg {
        Pause => {
            let config = CONFIG.load(deps.storage)?;
            assert_eq!(config.owner, info.sender, "not an owner");

            PAUSE.save(deps.storage, &true)?;

            Ok(Response::new().add_event(emit_paused(info.sender)))
        }
        Unpause => {
            let config = CONFIG.load(deps.storage)?;
            assert_eq!(config.owner, info.sender, "not an owner");

            PAUSE.save(deps.storage, &false)?;

            Ok(Response::new().add_event(emit_unpaused(info.sender)))
        }
        SetDefaultISM(new_default_ism) => {
            let config = CONFIG.load(deps.storage)?;
            assert_eq!(config.owner, info.sender, "not an owner");

            // FIXME: clone
            let new_default_ism = deps.api.addr_validate(&new_default_ism)?;
            CONFIG.save(
                deps.storage,
                &Config {
                    default_ism: new_default_ism.clone(),
                    ..config
                },
            )?;

            Ok(Response::new().add_event(emit_default_ism_changed(info.sender, new_default_ism)))
        }

        Dispatch {
            dest_domain,
            recipient_addr,
            msg_body,
        } => {
            assert!(recipient_addr.len() <= 32, "addr too long");

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

            // TODO: insert tree

            Ok(Response::new().add_events(vec![
                emit_dispatch_id(id),
                emit_dispatch(
                    msg.sender.clone(),
                    dest_domain,
                    msg.recipient.clone(),
                    msg.into(),
                ),
            ]))
        }
        Process { metadata, message } => {
            let config = CONFIG.load(deps.storage)?;

            let decoded_msg: Message = message.clone().into();
            assert!(decoded_msg.recipient.len() <= 32);

            let recipient = decoded_msg.recipient_addr(deps.api)?;

            let origin_domain = fetch_origin_domain(&deps.querier, &config.factory)?;

            assert_eq!(decoded_msg.version, MAILBOX_VERSION);
            assert_eq!(decoded_msg.dest_domain, origin_domain);

            let id = decoded_msg.id();
            assert!(MESSAGE_PROCESSED
                .may_load(deps.storage, id.0.clone())?
                .is_none());
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
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Find matched incoming message variant and query them your custom logic
        // and then construct your query response with the type usually defined
        // `msg.rs` alongside with the query message itself.
        //
        // use `cosmwasm_std::to_binary` to serialize query response to json binary.
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
