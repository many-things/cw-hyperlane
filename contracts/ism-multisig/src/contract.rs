use std::collections::HashSet;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use hpl_interface::{
    ism::{
        multisig::{ExecuteMsg, InstantiateMsg, MigrateMsg},
        ISMQueryMsg, ISMType, VerifyResponse,
    },
    types::{message::Message, metadata::MessageIdMultisigIsmMetadata},
};

use crate::{
    error::ContractError,
    execute,
    state::{Config, CONFIG, THRESHOLD, VALIDATORS},
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
        addr_prefix: msg.addr_prefix,
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
        EnrollValidator { set: msg } => execute::enroll_validator(deps, info, msg),
        EnrollValidators { set: validators } => execute::enroll_validators(deps, info, validators),
        UnenrollValidator {
            domain,
            validator: vald,
        } => execute::unenroll_validator(deps, info, domain, vald),
        SetThreshold { set: threshold } => execute::set_threshold(deps, info, threshold),
        SetThresholds { set: thresholds } => execute::set_thresholds(deps, info, thresholds),
        InitTransferOwnership { owner: next_owner } => {
            execute::init_transfer_ownership(deps, info, next_owner)
        }
        FinishTransferOwnership() => execute::finish_transfer_ownership(deps, info),
        RevokeTransferOwnership() => execute::revoke_transfer_ownership(deps, info),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: ISMQueryMsg) -> Result<Binary, ContractError> {
    use ISMQueryMsg::*;

    match msg {
        ModuleType {} => Ok(to_binary(&ISMType::Owned)?),
        Verify {
            metadata: raw_metadata,
            message: raw_message,
        } => {
            let metadata: MessageIdMultisigIsmMetadata = raw_metadata.into();
            let message: Message = raw_message.into();

            let threshold = THRESHOLD.load(deps.storage, message.origin_domain.into())?;
            let validators = VALIDATORS.load(deps.storage, message.origin_domain.into())?;

            let mut signatures: Vec<Binary> = Vec::new();
            for i in 0..metadata.signatures_len().unwrap() {
                signatures.push(metadata.signature_at(i))
            }

            let unique_vali_pubkey: HashSet<_> =
                validators.0.into_iter().map(|v| v.signer_pubkey).collect();

            let unique_meta_pubkey: HashSet<_> = signatures
                .into_iter()
                .flat_map(|sig| {
                    [
                        deps.api
                            .secp256k1_recover_pubkey(&message.id(), sig.as_slice(), 0)
                            .unwrap(),
                        deps.api
                            .secp256k1_recover_pubkey(&message.id(), sig.as_slice(), 1)
                            .unwrap(),
                    ]
                })
                .map(Binary::from)
                .collect();

            let success = unique_vali_pubkey
                .intersection(&unique_meta_pubkey)
                .collect::<Vec<_>>()
                .len();

            Ok(to_binary(&VerifyResponse {
                verified: success >= usize::from(threshold),
            })?)
        }
    }
}
