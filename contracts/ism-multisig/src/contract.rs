#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use hpl_interface::ism::{
    multisig::{ExecuteMsg, InstantiateMsg, MigrateMsg},
    ISMQueryMsg, ISMType, VerifyResponse,
};

use crate::{
    error::ContractError,
    execute::{gov, threshold, validator},
    state::{Config, CONFIG, PENDING_OWNER},
    verify::{self, sha256_digest},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        chain_hpl: msg.chain_hpl,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", config.owner))
}

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
        EnrollValidator(msg) => validator::enroll_validator(deps, info, msg),
        EnrollValidators(validators) => validator::enroll_validators(deps, info, validators),
        UnenrollValidator {
            domain,
            validator: vald,
        } => validator::unenroll_validator(deps, info, domain, vald),
        SetThreshold(threshold) => threshold::set_threshold(deps, info, threshold),
        SetThresholds(thresholds) => threshold::set_thresholds(deps, info, thresholds),
        InitTransferOwnership(next_owner) => gov::init_transfer_ownership(deps, info, next_owner),
        FinishTransferOwnership() => gov::finish_transfer_ownership(deps, info),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: ISMQueryMsg) -> Result<Binary, ContractError> {
    use ISMQueryMsg::*;

    match msg {
        ModuleType => Ok(to_binary(&ISMType::Owned)?),
        // TODO: ask what is stand for?
        Verify { metadata, message } => {
            // let config = CONFIG.load(deps.storage)?;

            // let digest = sha256_digest(Binary::from(message))?;

            // let verified = deps
            //     .api
            //     .secp256k1_verify(&digest, &metadata, &config.owner_pubkey)?;

            Ok(to_binary(&VerifyResponse(true))?)
        }
    }
}
