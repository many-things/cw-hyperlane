mod error;

pub use crate::error::ContractError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, to_json_binary, Addr, Deps, DepsMut, Empty, Env, Event, HexBinary,
    MessageInfo, QueryResponse, Response, StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use hpl_interface::{
    ism::{
        aggregate::{AggregateIsmQueryMsg, ExecuteMsg, InstantiateMsg, IsmsResponse, QueryMsg},
        IsmQueryMsg, IsmType, ModuleTypeResponse, VerifyInfoResponse, VerifyResponse,
    },
    to_binary,
    types::{bech32_decode, AggregateMetadata},
};
use hpl_ownable::get_owner;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const ISMS_KEY: &str = "isms";
const ISMS: Item<Vec<Addr>> = Item::new(ISMS_KEY);

const THRESHOLD_KEY: &str = "threshold";
const THRESHOLD: Item<u8> = Item::new(THRESHOLD_KEY);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_ism_aggregate::{}", name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    let isms = msg
        .isms
        .iter()
        .map(|v| deps.api.addr_validate(v))
        .collect::<StdResult<_>>()?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    ISMS.save(deps.storage, &isms)?;
    THRESHOLD.save(deps.storage, &msg.threshold)?;

    Ok(Response::new().add_event(
        new_event("instantiate")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("isms", msg.isms.join(","))
            .add_attribute("threshold", msg.threshold.to_string()),
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
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::SetIsms { isms, threshold } => {
            ensure_eq!(
                get_owner(deps.storage)?,
                info.sender,
                ContractError::Unauthorized
            );
            ensure!(
                threshold > 0,
                ContractError::InvalidThreshold("threshold must not be zero".to_string())
            );
            ensure!(
                isms.len() >= threshold as usize,
                ContractError::InvalidThreshold(
                    "threshold should be less than ism count".to_string()
                )
            );

            let parsed_isms = isms
                .iter()
                .map(|v| deps.api.addr_validate(v))
                .collect::<StdResult<_>>()?;

            ISMS.save(deps.storage, &parsed_isms)?;
            THRESHOLD.save(deps.storage, &threshold)?;

            Ok(Response::new()
                .add_event(new_event("set_isms").add_attribute("isms", isms.join(","))))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use IsmQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),

        QueryMsg::Ism(msg) => match msg {
            ModuleType {} => to_binary({
                Ok::<_, ContractError>(ModuleTypeResponse {
                    typ: IsmType::Aggregation,
                })
            }),
            Verify { metadata, message } => to_binary(verify(deps, metadata, message)),
            VerifyInfo { message } => to_binary(verify_info(deps, message)),
        },

        QueryMsg::AggregateIsm(msg) => match msg {
            AggregateIsmQueryMsg::Isms {} => Ok(to_json_binary(&IsmsResponse {
                isms: ISMS
                    .load(deps.storage)?
                    .into_iter()
                    .map(|v| v.into())
                    .collect(),
                threshold: THRESHOLD.load(deps.storage)?,
            })?),
        },
    }
}

fn verify(
    deps: Deps,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<VerifyResponse, ContractError> {
    let isms = ISMS.load(deps.storage)?;

    let mut threshold = THRESHOLD.load(deps.storage)?;

    for (ism, meta) in AggregateMetadata::from_hex(metadata, isms) {
        let verified = hpl_interface::ism::verify(&deps.querier, ism, meta, message.clone())?;

        if verified {
            threshold -= 1;
        }

        if threshold == 0 {
            break;
        }
    }

    Ok(VerifyResponse {
        verified: threshold == 0,
    })
}

fn verify_info(deps: Deps, _message: HexBinary) -> Result<VerifyInfoResponse, ContractError> {
    Ok(VerifyInfoResponse {
        threshold: THRESHOLD.load(deps.storage)?,
        validators: ISMS
            .load(deps.storage)?
            .into_iter()
            .map(|v| Ok(bech32_decode(v.as_str())?.into()))
            .collect::<StdResult<_>>()?,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
