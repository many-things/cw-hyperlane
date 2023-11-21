mod error;

pub use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    ensure_eq, Addr, Binary, Deps, DepsMut, Empty, Env, Event, HexBinary, MessageInfo,
    QueryResponse, Response,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use hpl_interface::{
    ism::{
        wormhole::{ExecuteMsg, InstantiateMsg, QueryMsg, WormholeIsmQueryMsg},
        IsmQueryMsg, IsmType, ModuleTypeResponse, VerifyInfoResponse, VerifyResponse,
    },
    to_binary,
    types::{bech32_decode, Message},
};

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const WORMHOLE_CORE_KEY: &str = "wormhole_core";
const WORMHOLE_CORE: Item<Addr> = Item::new(WORMHOLE_CORE_KEY);

#[cw_serde]
enum WormholeQueryMsg {
    VerifyVAA { vaa: Binary, block_time: u64 },
}

#[cw_serde]
struct ParsedVAA {
    pub version: u8,
    pub guardian_set_index: u32,
    pub timestamp: u32,
    pub nonce: u32,
    pub len_signers: u8,

    pub emitter_chain: u16,
    pub emitter_address: Vec<u8>,
    pub sequence: u64,
    pub consistency_level: u8,
    pub payload: Vec<u8>,

    pub hash: Vec<u8>,
}

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_ism_wormhole::{}", name))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    hpl_ownable::initialize(deps.storage, &owner)?;

    let wormhole_core = deps.api.addr_validate(&msg.wormhole_core)?;
    WORMHOLE_CORE.save(deps.storage, &wormhole_core)?;

    Ok(Response::new().add_event(
        new_event("instantiate")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("wormhole_core", wormhole_core),
    ))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::SetWormholeCore { wormhole_core } => {
            ensure_eq!(
                hpl_ownable::get_owner(deps.storage)?,
                info.sender,
                ContractError::Unauthorized
            );

            let wormhole_core = deps.api.addr_validate(&wormhole_core)?;
            WORMHOLE_CORE.save(deps.storage, &wormhole_core)?;

            Ok(Response::new().add_event(
                new_event("set_wormhole_core").add_attribute("wormhole_core", wormhole_core),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use IsmQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),

        QueryMsg::Ism(msg) => match msg {
            ModuleType {} => to_binary({
                Ok::<_, ContractError>(ModuleTypeResponse {
                    typ: IsmType::Wormhole,
                })
            }),
            Verify { metadata, message } => to_binary(verify(deps, metadata, message)),
            VerifyInfo { message } => to_binary(verify_info(deps, message)),
        },

        QueryMsg::WormholeIsm(msg) => match msg {
            WormholeIsmQueryMsg::WormholeCore {} => Ok(cosmwasm_std::to_binary(
                &WORMHOLE_CORE.load(deps.storage)?.into_string(),
            )?),
        },
    }
}

fn verify(
    deps: Deps,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<VerifyResponse, ContractError> {
    let wormhole_core = WORMHOLE_CORE.load(deps.storage)?;
    let wormhole_query_msg = WormholeQueryMsg::VerifyVAA {
        vaa: cosmwasm_std::to_binary(metadata.as_slice())?,
        block_time: 0,
    };

    let parsed_vaa: ParsedVAA = deps
        .querier
        .query_wasm_smart(wormhole_core, &wormhole_query_msg)?;

    let message: Message = message.into();
    let id = message.id();

    Ok(VerifyResponse {
        verified: HexBinary::from(parsed_vaa.payload) == id,
    })
}

fn verify_info(deps: Deps, _message: HexBinary) -> Result<VerifyInfoResponse, ContractError> {
    // this is not entirely correct, but I don't see a better way to do this
    // we cannot query validators from Wormhole Core contract
    Ok(VerifyInfoResponse {
        threshold: 1,
        validators: vec![bech32_decode(WORMHOLE_CORE.load(deps.storage)?.as_str())?.into()],
    })
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    Ok(Response::default())
}
