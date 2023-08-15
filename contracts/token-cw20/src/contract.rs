use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, from_binary, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, Event,
    MessageInfo, QueryResponse, Reply, Response, SubMsg, Uint128, Uint256, WasmMsg,
};
use hpl_interface::{
    router::{DomainsResponse, RouterResponse},
    token::{self, TokenMode, TokenType},
    token_cw20::{ExecuteMsg, QueryMsg, ReceiveMsg, TokenModeResponse, TokenTypeResponse},
    types::bech32_encode,
};

use crate::{
    error::ContractError,
    msg::{InstantiateMsg, MigrateMsg, TokenOption},
    state::{MAILBOX, MODE, OWNER, TOKEN},
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

    MODE.save(deps.storage, &msg.mode)?;
    OWNER.save(deps.storage, &deps.api.addr_validate(&msg.owner)?)?;
    MAILBOX.save(deps.storage, &deps.api.addr_validate(&msg.mailbox)?)?;

    let mut resp = Response::new();

    if msg.mode == TokenMode::Bridged {
        ensure!(msg.token.is_some(), ContractError::InvalidTokenOption);
        let token = msg.token.clone().unwrap();

        match token {
            TokenOption::Create { code_id, init_msg } => {
                resp = resp.add_submessage(SubMsg::reply_on_success(
                    WasmMsg::Instantiate {
                        admin: Some(env.contract.address.to_string()),
                        code_id,
                        msg: to_binary(&init_msg)?,
                        funds: vec![],
                        label: "created by hpl-toen-cw20".to_string(),
                    },
                    REPLY_ID_CREATE_DENOM,
                ));
            }
            TokenOption::Reuse { contract } => {
                TOKEN.save(deps.storage, &deps.api.addr_validate(&contract)?)?
            }
        }
    }

    Ok(resp.add_event(
        Event::new("init-token-cw20")
            .add_attribute("creator", info.sender)
            .add_attribute("mode", format!("{}", msg.mode))
            .add_attribute(
                "token",
                msg.token
                    .map(|v| match v {
                        TokenOption::Create { .. } => "".to_string(),
                        TokenOption::Reuse { contract } => contract,
                    })
                    .unwrap_or_default(),
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
                OWNER.load(deps.storage)?,
                ContractError::Unauthorized
            );

            Ok(hpl_router::handle(deps, env, info, msg)?)
        }
        ExecuteMsg::Handle(msg) => {
            ensure_eq!(
                info.sender,
                MAILBOX.load(deps.storage)?,
                ContractError::Unauthorized
            );

            let token_msg: token::Message = msg.body.into();
            let recipient = bech32_encode("osmo", &token_msg.recipient)?;

            let token = TOKEN.load(deps.storage)?;
            let mode = MODE.load(deps.storage)?;

            let msg = match mode {
                TokenMode::Bridged =>
                // make token mint msg if token mode is bridged
                {
                    WasmMsg::Execute {
                        contract_addr: token.to_string(),
                        msg: to_binary(&cw20::Cw20ExecuteMsg::Mint {
                            recipient: recipient.to_string(),
                            amount: Uint128::from_str(&token_msg.amount.to_string())?,
                        })?,
                        funds: vec![],
                    }
                }
                TokenMode::Collateral =>
                // make token transfer msg if token mode is collateral
                // we can consider to use MsgSend for further utility
                {
                    WasmMsg::Execute {
                        contract_addr: token.to_string(),
                        msg: to_binary(&cw20::Cw20ExecuteMsg::Transfer {
                            recipient: recipient.to_string(),
                            amount: Uint128::from_str(&token_msg.amount.to_string())?,
                        })?,
                        funds: vec![],
                    }
                }
            };

            Ok(Response::new().add_message(msg).add_event(
                Event::new("token-cw20-handle")
                    .add_attribute("recipient", recipient)
                    .add_attribute("token", token)
                    .add_attribute("amount", token_msg.amount),
            ))
        }
        ExecuteMsg::Receive(msg) => {
            let token = TOKEN.load(deps.storage)?;

            ensure_eq!(info.sender, token, ContractError::Unauthorized);

            match from_binary::<ReceiveMsg>(&msg.msg)? {
                ReceiveMsg::TransferRemote {
                    dest_domain,
                    recipient,
                } => {
                    let token = TOKEN.load(deps.storage)?;
                    let mode = MODE.load(deps.storage)?;
                    let mailbox = MAILBOX.load(deps.storage)?;

                    let dest_router = hpl_router::get_router(deps.storage, dest_domain)?;

                    let mut msgs: Vec<CosmosMsg> = vec![];

                    if mode == TokenMode::Bridged {
                        // push token burn msg if token is bridged
                        msgs.push(
                            WasmMsg::Execute {
                                contract_addr: token.to_string(),
                                msg: to_binary(&cw20::Cw20ExecuteMsg::Burn {
                                    amount: Uint128::from_str(&msg.amount.to_string())?,
                                })?,
                                funds: vec![],
                            }
                            .into(),
                        );
                    }

                    let dispatch_payload = token::Message {
                        recipient: recipient.clone(),
                        amount: Uint256::from_str(&msg.amount.to_string())?,
                        metadata: Binary::default(),
                    };

                    // push mailbox dispatch msg
                    msgs.push(
                        WasmMsg::Execute {
                            contract_addr: mailbox.to_string(),
                            msg: to_binary(&hpl_interface::mailbox::ExecuteMsg::Dispatch {
                                dest_domain,
                                recipient_addr: dest_router.into(),
                                msg_body: dispatch_payload.into(),
                            })?,
                            funds: vec![],
                        }
                        .into(),
                    );

                    Ok(Response::new().add_messages(msgs).add_event(
                        Event::new("token-cw20-transfer-remote")
                            .add_attribute("sender", msg.sender)
                            .add_attribute("recipient", recipient.to_base64())
                            .add_attribute("token", token)
                            .add_attribute("amount", msg.amount),
                    ))
                }
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_CREATE_DENOM => {
            let reply_data = msg.result.unwrap().data.unwrap();
            let init_resp = cw_utils::parse_instantiate_response_data(&reply_data)?;
            let init_addr = deps.api.addr_validate(&init_resp.contract_address)?;

            TOKEN.save(deps.storage, &init_addr)?;

            let resp = Response::new().add_event(
                Event::new("reply-init-token-cw20").add_attribute("new_token", init_addr),
            );

            Ok(resp)
        }

        _ => Err(ContractError::InvalidReplyId),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::Domains {} => Ok(to_binary(&DomainsResponse {
            domains: hpl_router::get_domains(deps.storage)?,
        })?),
        QueryMsg::Router { domain } => Ok(to_binary(&RouterResponse {
            router: hpl_router::get_router(deps.storage, domain)?,
        })?),

        QueryMsg::TokenType {} => Ok(to_binary(&TokenTypeResponse {
            typ: TokenType::CW20 {
                contract: TOKEN.load(deps.storage)?.to_string(),
            },
        })?),
        QueryMsg::TokenMode {} => Ok(to_binary(&TokenModeResponse {
            mode: MODE.load(deps.storage)?,
        })?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
