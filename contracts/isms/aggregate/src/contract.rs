use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, Deps, DepsMut, Env, HexBinary, MessageInfo, QueryResponse, Response, StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use hpl_interface::{
    ism::{
        aggregate::InstantiateMsg, ExpectedIsmQueryMsg, IsmQueryMsg, IsmType, ModuleTypeResponse,
        VerifyInfoResponse, VerifyResponse,
    },
    to_binary,
};

use crate::{error::ContractError, CONTRACT_NAME, CONTRACT_VERSION};

#[cw_serde]
pub struct Config {
    pub isms: Vec<Addr>,
    pub threshold: u8,
}

const CONFIG_KEY: &str = "config";
const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let isms = msg
        .isms
        .into_iter()
        .map(|v| deps.api.addr_validate(&v))
        .collect::<StdResult<_>>()?;

    CONFIG.save(
        deps.storage,
        &Config {
            isms,
            threshold: msg.threshold,
        },
    )?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: ExpectedIsmQueryMsg,
) -> Result<QueryResponse, ContractError> {
    use IsmQueryMsg::*;

    match msg {
        ExpectedIsmQueryMsg::Ism(msg) => match msg {
            ModuleType {} => to_binary({
                Ok::<_, ContractError>(ModuleTypeResponse {
                    typ: IsmType::Aggregation,
                })
            }),
            Verify { metadata, message } => to_binary(verify(deps, metadata, message)),
            VerifyInfo { message } => to_binary(verify_info(deps, message)),
        },
    }
}

fn verify(
    deps: Deps,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<VerifyResponse, ContractError> {
    Ok(VerifyResponse { verified: true })
}

fn verify_info(deps: Deps, message: HexBinary) -> Result<VerifyInfoResponse, ContractError> {
    Ok(VerifyInfoResponse {
        threshold: 1,
        validators: vec![],
    })
}
