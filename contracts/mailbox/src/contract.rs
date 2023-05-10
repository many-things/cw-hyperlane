#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Addr, Binary, Deps, DepsMut, Env, Event, HexBinary, MessageInfo,
    QuerierWrapper, Reply, Response, StdResult, WasmMsg,
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
    state::{Config, CONFIG, MESSAGE_PROCESSED},
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

    match msg {
        SetDefaultISM(new_default_ism) => {
            let config = CONFIG.load(deps.storage)?;
            if config.owner != info.sender {
                return Err(ContractError::Unauthorized {});
            }

            // FIXME: clone
            let new_default_ism = deps.api.addr_validate(&new_default_ism)?;
            CONFIG.save(
                deps.storage,
                &Config {
                    default_ism: new_default_ism.clone(),
                    ..config.clone()
                },
            )?;

            let resp = Response::new().add_attributes(vec![
                attr("method", "set_default_ism"),
                attr("owner", config.owner),
                attr("new_default_ism", new_default_ism),
            ]);

            Ok(resp)
        }

        Dispatch {
            dest_domain,
            recipient_addr,
            msg_body,
        } => {
            assert!(recipient_addr.len() <= 32);

            unimplemented!();

            Ok(Response::default())
        }
        Process { metadata, message } => {
            let config = CONFIG.load(deps.storage)?;

            let decoded_msg: Message = message.clone().into();
            assert!(decoded_msg.recipient.len() <= 32);

            let receipient = decoded_msg.recipient_addr(deps.api)?;

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
                &receipient,
                metadata,
                message,
            )?;

            let handle_msg = WasmMsg::Execute {
                contract_addr: receipient.to_string(),
                msg: to_binary(&ExpectedHandlerMsg::Handle(HandleMsg {
                    origin: decoded_msg.origin_domain,
                    sender: decoded_msg.sender.clone().into(),
                    body: decoded_msg.body.into(),
                }))?,
                funds: vec![],
            };

            let resp = Response::new().add_message(handle_msg).add_events(vec![
                Event::new("mailbox_process_id")
                    .add_attributes(vec![attr("id", HexBinary::from(id).to_hex())]),
                Event::new("mailbox_process").add_attributes(vec![
                    attr("origin", format!("{}", origin_domain)),
                    attr("sender", HexBinary::from(decoded_msg.sender).to_hex()),
                    attr("recipient", receipient),
                ]),
            ]);

            Ok(resp)
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
