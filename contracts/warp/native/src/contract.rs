use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, ensure_ne, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, Event, HexBinary,
    MessageInfo, QueryResponse, Reply, Response, SubMsg, Uint128, Uint256, WasmMsg,
};
use hpl_interface::{
    core::mailbox,
    to_binary,
    types::bech32_encode,
    warp::{
        self,
        native::{ExecuteMsg, InstantiateMsg, QueryMsg},
    },
    warp::{TokenMode, TokenModeResponse, TokenTypeResponse},
};
use hpl_ownable::get_owner;
use hpl_router::get_route;

use crate::{
    error::ContractError,
    proto::{self, MsgBurn, MsgCreateDenom, MsgCreateDenomResponse, MsgMint, MsgSetDenomMetadata},
    state::{HRP, MAILBOX, MODE, TOKEN},
    CONTRACT_NAME, CONTRACT_VERSION, REPLY_ID_CREATE_DENOM,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    HRP.save(deps.storage, &msg.hrp)?;
    MODE.save(deps.storage, &msg.mode)?;
    MAILBOX.save(deps.storage, &deps.api.addr_validate(&msg.mailbox)?)?;

    let owner = deps.api.addr_validate(&msg.owner)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    let mut resp = Response::new();

    // create native denom if token is bridged
    if msg.mode == TokenMode::Bridged {
        resp = resp.add_submessage(SubMsg::reply_on_success(
            MsgCreateDenom {
                sender: env.contract.address.to_string(),
                subdenom: msg.denom.clone(),
            },
            REPLY_ID_CREATE_DENOM,
        ));

        if let Some(metadata) = msg.metadata {
            resp = resp.add_message(MsgSetDenomMetadata {
                sender: env.contract.address.to_string(),
                metadata: Some(proto::Metadata {
                    description: metadata.description,
                    denom_units: metadata
                        .denom_units
                        .into_iter()
                        .map(|v| proto::DenomUnit {
                            denom: v.denom,
                            exponent: v.exponent,
                            aliases: v.aliases,
                        })
                        .collect(),
                    base: metadata.base,
                    display: metadata.display,
                    name: metadata.name,
                    symbol: metadata.symbol,
                }),
            });
        }
    } else {
        // use denom directly if token is native
        TOKEN.save(deps.storage, &msg.denom)?;
    }

    Ok(resp.add_event(
        Event::new("hpl::token-native::init")
            .add_attribute("creator", info.sender)
            .add_attribute("mode", format!("{}", msg.mode))
            .add_attribute(
                "denom",
                if msg.mode == TokenMode::Bridged {
                    ""
                } else {
                    &msg.denom
                },
            ),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Router(msg) => {
            ensure_eq!(
                info.sender,
                get_owner(deps.storage)?,
                ContractError::Unauthorized {}
            );
            Ok(hpl_router::handle(deps, env, info, msg)?)
        }
        ExecuteMsg::Handle(msg) => {
            ensure_eq!(
                info.sender,
                MAILBOX.load(deps.storage)?,
                ContractError::Unauthorized
            );
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
                msgs.push(
                    MsgMint {
                        sender: env.contract.address.to_string(),
                        amount: Some(proto::Coin {
                            denom: token.clone(),
                            amount: token_msg.amount.to_string(),
                        }),
                    }
                    .into(),
                );
            }

            // push token send msg
            msgs.push(
                BankMsg::Send {
                    to_address: recipient.to_string(),
                    amount: vec![Coin {
                        denom: token.clone(),
                        amount: Uint128::from_str(&token_msg.amount.to_string())?,
                    }],
                }
                .into(),
            );

            Ok(Response::new().add_messages(msgs).add_event(
                Event::new("hpl::token-native::handle")
                    .add_attribute("recipient", recipient)
                    .add_attribute("token", token)
                    .add_attribute("amount", token_msg.amount),
            ))
        }
        ExecuteMsg::TransferRemote {
            dest_domain,
            recipient,
        } => {
            let token = TOKEN.load(deps.storage)?;
            let mode = MODE.load(deps.storage)?;
            let mailbox = MAILBOX.load(deps.storage)?;
            let paid = cw_utils::must_pay(&info, &token)?;

            let dest_router = get_route::<HexBinary>(deps.storage, dest_domain)?
                .route
                .expect("route not found");
            ensure_ne!(
                dest_router,
                HexBinary::default(),
                ContractError::NoRouter {
                    domain: dest_domain
                }
            );

            let mut msgs: Vec<CosmosMsg> = vec![];

            if mode == TokenMode::Bridged {
                // push token burn msg if token is bridged
                msgs.push(
                    MsgBurn {
                        sender: env.contract.address.to_string(),
                        amount: Some(proto::Coin {
                            denom: token.clone(),
                            amount: paid.to_string(),
                        }),
                    }
                    .into(),
                );
            }

            let dispatch_payload = warp::Message {
                recipient: recipient.clone(),
                amount: Uint256::from_str(&paid.to_string())?,
                metadata: HexBinary::default(),
            };

            // push mailbox dispatch msg
            msgs.push(
                WasmMsg::Execute {
                    contract_addr: mailbox.to_string(),
                    msg: cosmwasm_std::to_binary(&mailbox::ExecuteMsg::Dispatch {
                        dest_domain,
                        recipient_addr: dest_router,
                        msg_body: dispatch_payload.into(),
                        hook: None,
                        metadata: None,
                    })?,
                    funds: vec![],
                }
                .into(),
            );

            Ok(Response::new().add_messages(msgs).add_event(
                Event::new("hpl::token-native::transfer-remote")
                    .add_attribute("sender", info.sender)
                    .add_attribute("recipient", recipient.to_hex())
                    .add_attribute("token", token)
                    .add_attribute("amount", paid.to_string()),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_CREATE_DENOM => {
            let reply_data = msg.result.unwrap().data.unwrap();
            let reply: MsgCreateDenomResponse = reply_data.try_into()?;

            TOKEN.save(deps.storage, &reply.new_token_denom)?;

            let resp = Response::new().add_event(
                Event::new("hpl::token-native::reply-init")
                    .add_attribute("method", "reply_instantiate")
                    .add_attribute("new_denom", reply.new_token_denom),
            );

            Ok(resp)
        }

        _ => Err(ContractError::InvalidReplyId),
    }
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
