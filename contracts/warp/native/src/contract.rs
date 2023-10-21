#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, CosmosMsg, Deps, DepsMut, Env, HexBinary, MessageInfo, QueryResponse, Reply,
    Response, SubMsg, Uint256,
};
use hpl_interface::{
    core::mailbox,
    to_binary,
    types::bech32_encode,
    warp::{
        self,
        native::{ExecuteMsg, InstantiateMsg, QueryMsg},
    },
    warp::{TokenMode, TokenModeMsg, TokenModeResponse, TokenTypeResponse},
};
use hpl_router::get_route;

use crate::{
    conv,
    error::ContractError,
    new_event,
    proto::{MsgCreateDenom, MsgCreateDenomResponse},
    CONTRACT_NAME, CONTRACT_VERSION, HRP, MAILBOX, MODE, REPLY_ID_CREATE_DENOM, TOKEN,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mode: TokenMode = msg.token.clone().into();
    let owner = deps.api.addr_validate(&msg.owner)?;

    HRP.save(deps.storage, &msg.hrp)?;
    MODE.save(deps.storage, &mode)?;
    MAILBOX.save(deps.storage, &deps.api.addr_validate(&msg.mailbox)?)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    let mut denom = "".into();

    let msgs = match msg.token {
        // create native denom if token is bridged
        TokenModeMsg::Bridged(token) => {
            let mut msgs = vec![];

            msgs.push(SubMsg::reply_on_success(
                MsgCreateDenom {
                    sender: env.contract.address.to_string(),
                    subdenom: token.denom.clone(),
                },
                REPLY_ID_CREATE_DENOM,
            ));

            if let Some(metadata) = token.metadata {
                msgs.push(SubMsg::new(conv::to_set_metadata_msg(
                    &env.contract.address,
                    metadata,
                )));
            }

            msgs
        }
        // use denom directly if token is native
        TokenModeMsg::Collateral(token) => {
            TOKEN.save(deps.storage, &token.denom)?;
            denom = token.denom;
            vec![]
        }
    };

    Ok(Response::new().add_submessages(msgs).add_event(
        new_event("instantiate")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("mode", format!("{}", mode))
            .add_attribute("denom", denom),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        Router(msg) => Ok(hpl_router::handle(deps, env, info, msg)?),
        Handle(msg) => mailbox_handle(deps, env, info, msg),
        TransferRemote {
            dest_domain,
            recipient,
        } => transfer_remote(deps, env, info, dest_domain, recipient),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let reply_data = msg.result.unwrap().data.unwrap();

    match msg.id {
        REPLY_ID_CREATE_DENOM => {
            let reply: MsgCreateDenomResponse = reply_data.try_into()?;

            TOKEN.save(deps.storage, &reply.new_token_denom)?;

            let resp = Response::new().add_event(
                new_event("reply-instantiate").add_attribute("denom", reply.new_token_denom),
            );

            Ok(resp)
        }

        _ => Err(ContractError::InvalidReplyId),
    }
}

fn mailbox_handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: hpl_interface::core::HandleMsg,
) -> Result<Response, ContractError> {
    // validate mailbox
    ensure_eq!(
        info.sender,
        MAILBOX.load(deps.storage)?,
        ContractError::Unauthorized
    );
    // validate message origin - this should be registered route
    ensure_eq!(
        msg.sender,
        get_route::<HexBinary>(deps.storage, msg.origin)?
            .route
            .expect("route not found"),
        ContractError::Unauthorized
    );

    let token_msg: warp::Message = msg.body.into();
    let recipient = bech32_encode(&HRP.load(deps.storage)?, &token_msg.recipient)?;

    let token = TOKEN.load(deps.storage)?;
    let mode = MODE.load(deps.storage)?;

    let mut msgs: Vec<CosmosMsg> = vec![];

    if mode == TokenMode::Bridged {
        // push token mint msg if token is bridged
        msgs.push(conv::to_mint_msg(&env.contract.address, &token, token_msg.amount).into());
    }

    // push token send msg
    msgs.push(
        conv::to_send_msg(
            &recipient,
            vec![conv::to_coin_u256(token_msg.amount, &token)?],
        )
        .into(),
    );

    Ok(Response::new().add_messages(msgs).add_event(
        new_event("handle")
            .add_attribute("recipient", recipient)
            .add_attribute("token", token)
            .add_attribute("amount", token_msg.amount),
    ))
}

fn transfer_remote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    dest_domain: u32,
    recipient: HexBinary,
) -> Result<Response, ContractError> {
    let token = TOKEN.load(deps.storage)?;
    let mode = MODE.load(deps.storage)?;
    let mailbox = MAILBOX.load(deps.storage)?;
    let transfer_amount = cw_utils::must_pay(&info, &token)?;

    let dest_router = get_route::<HexBinary>(deps.storage, dest_domain)?
        .route
        .expect("route not found");

    let mut msgs: Vec<CosmosMsg> = vec![];

    if mode == TokenMode::Bridged {
        // push token burn msg if token is bridged
        msgs.push(conv::to_burn_msg(&env.contract.address, &token, transfer_amount).into());
    }

    let dispatch_payload = warp::Message {
        recipient: recipient.clone(),
        amount: Uint256::from_uint128(transfer_amount),
        metadata: HexBinary::default(),
    };

    // push mailbox dispatch msg
    msgs.push(mailbox::dispatch(
        mailbox,
        dest_domain,
        dest_router,
        dispatch_payload.into(),
        None,
        None,
    )?);

    Ok(Response::new().add_messages(msgs).add_event(
        new_event("transfer-remote")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", recipient.to_hex())
            .add_attribute("token", token)
            .add_attribute("amount", transfer_amount.to_string()),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use warp::TokenWarpDefaultQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Router(msg) => Ok(hpl_router::handle_query(deps, env, msg)?),
        QueryMsg::TokenDefault(msg) => match msg {
            TokenType {} => to_binary(get_token_type(deps)),
            TokenMode {} => to_binary(get_token_mode(deps)),
        },
    }
}

fn get_token_type(deps: Deps) -> Result<TokenTypeResponse, ContractError> {
    let denom = TOKEN.load(deps.storage)?;

    Ok(TokenTypeResponse {
        typ: warp::TokenType::Native(warp::TokenTypeNative::Fungible { denom }),
    })
}

fn get_token_mode(deps: Deps) -> Result<TokenModeResponse, ContractError> {
    let mode = MODE.load(deps.storage)?;

    Ok(TokenModeResponse { mode })
}
