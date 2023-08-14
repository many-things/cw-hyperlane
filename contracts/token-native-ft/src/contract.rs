use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Reply, Response, SubMsg,
};
use hpl_interface::{
    router::{DomainsResponse, RouterResponse},
    token::{
        ExecuteMsg, QueryMsg, TokenMode, TokenModeResponse, TokenType, TokenTypeNative,
        TokenTypeResponse,
    },
};

use crate::{
    error::ContractError,
    proto::{self, MsgCreateDenom, MsgCreateDenomResponse, MsgSetDenomMetadata},
    state::{MODE, TOKEN},
    CONTRACT_NAME, CONTRACT_VERSION, REPLY_ID_CREATE_DENOM,
};

#[cw_serde]
pub struct DenomUnit {
    pub denom: String,
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub exponent: u32,
    pub aliases: Vec<String>,
}

#[cw_serde]
pub struct Metadata {
    pub description: String,
    pub denom_units: Vec<DenomUnit>,
    pub base: String,
    pub display: String,
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub metadata: Option<Metadata>,
    pub mode: TokenMode,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    MODE.save(deps.storage, &msg.mode)?;

    let mut resp = Response::new();

    if msg.mode == TokenMode::Collateral {
        resp = resp.add_submessage(SubMsg::reply_on_success(
            MsgCreateDenom {
                sender: env.contract.address.to_string(),
                subdenom: msg.denom,
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
    }

    Ok(resp.add_event(
        Event::new("init-token-native-fungible")
            .add_attribute("creator", info.sender)
            .add_attribute("mode", format!("{}", msg.mode)),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_CREATE_DENOM => {
            let reply_data = msg.result.unwrap().data.unwrap();
            let reply: MsgCreateDenomResponse = reply_data.try_into()?;

            TOKEN.save(deps.storage, &reply.new_token_denom)?;

            let resp = Response::new().add_event(
                Event::new("reply-init-token-native-fungible")
                    .add_attribute("method", "reply_instantiate")
                    .add_attribute("new_denom", reply.new_token_denom),
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
            typ: TokenType::Native(TokenTypeNative::Fungible {
                denom: TOKEN.load(deps.storage)?,
            }),
        })?),
        QueryMsg::TokenMode {} => Ok(to_binary(&TokenModeResponse {
            mode: MODE.load(deps.storage)?,
        })?),
    }
}

#[cw_serde]
pub struct MigrateMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
