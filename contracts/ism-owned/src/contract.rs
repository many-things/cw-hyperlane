#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response};
use cw2::set_contract_version;
use hpl_interface::ism::{
    owned::{ExecuteMsg, InstantiateMsg, MigrateMsg},
    ISMQueryMsg, ISMType, VerifyResponse,
};

use crate::{
    error::ContractError,
    state::{Config, CONFIG},
    verify::{self, sha256_digest},
    CONTRACT_NAME, CONTRACT_VERSION,
};

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        hpl: msg.hpl,
        owner: deps.api.addr_validate(&msg.owner)?,
        owner_pubkey: msg.owner_pubkey,
    };

    CONFIG.save(deps.storage, &config)?;

    assert_eq!(
        msg.owner,
        verify::pub_to_addr(config.owner_pubkey, &config.hpl)?,
        "addr, pubkey mismatch"
    );

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", config.owner))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
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
        UpdateOwner {
            owner: new_owner,
            owner_pubkey: new_owner_pubkey,
        } => {
            let config = CONFIG.load(deps.storage)?;
            assert_eq!(config.owner, info.sender, "not an owner");

            let config = Config {
                owner: deps.api.addr_validate(&new_owner)?,
                owner_pubkey: new_owner_pubkey,
                ..config
            };

            CONFIG.save(deps.storage, &config)?;

            assert_eq!(
                new_owner,
                verify::pub_to_addr(config.owner_pubkey, &config.hpl)?,
                "addr, pubkey mismatch"
            );

            Ok(Response::default())
        }
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: ISMQueryMsg) -> Result<Binary, ContractError> {
    use ISMQueryMsg::*;

    match msg {
        ModuleType => Ok(to_binary(&ISMType::Owned)?),

        Verify { metadata, message } => {
            let config = CONFIG.load(deps.storage)?;

            let digest = sha256_digest(Binary::from(message))?;

            let verified = deps
                .api
                .secp256k1_verify(&digest, &metadata, &config.owner_pubkey)?;

            Ok(to_binary(&VerifyResponse(verified))?)
        }
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
