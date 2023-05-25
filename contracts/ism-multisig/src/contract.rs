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
    state::{Config, ValidatorSet, Validators, CONFIG, PENDING_OWNER, VALIDATORS},
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
        EnrollValidator(msg) => {
            let config = CONFIG.load(deps.storage)?;
            assert_eq!(info.sender, config.owner, "unauthorized");
            assert_eq!(
                msg.validator,
                verify::pub_to_addr(msg.validator_pubkey, &config.chain_hpl)?,
                "addr, pubkey mismatch"
            );

            let candidate = deps.api.addr_validate(&msg.validator)?;
            let mut validators = VALIDATORS.load(deps.storage, msg.domain)?;

            assert!(
                validators
                    .0
                    .iter()
                    .find(|v| v.signer == candidate)
                    .is_none(),
                "duplicate validator"
            );

            validators.0.push(ValidatorSet {
                signer: candidate,
                signer_pubkey: msg.validator_pubkey,
            });
            validators.0.sort_by(|a, b| a.signer.cmp(&b.signer));

            VALIDATORS.save(deps.storage, msg.domain, &validators)?;

            // TODO: define event
            Ok(Response::new())
        }
        EnrollValidators(validators) => {
            let config = CONFIG.load(deps.storage)?;
            assert_eq!(info.sender, config.owner);

            for msg in validators {
                assert_eq!(
                    msg.validator,
                    verify::pub_to_addr(msg.validator_pubkey, &config.chain_hpl)?,
                    "addr, pubkey mismatch"
                );

                let candidate = deps.api.addr_validate(&msg.validator)?;
                let mut validators = VALIDATORS.load(deps.storage, msg.domain)?;

                assert!(
                    validators
                        .0
                        .iter()
                        .find(|v| v.signer == candidate)
                        .is_none(),
                    "duplicate validator"
                );

                validators.0.push(ValidatorSet {
                    signer: candidate,
                    signer_pubkey: msg.validator_pubkey,
                });
                validators.0.sort_by(|a, b| a.signer.cmp(&b.signer));

                VALIDATORS.save(deps.storage, msg.domain, &validators)?;
            }

            // TODO: define event
            Ok(Response::new())
        }
        UnenrollValidator { domain, validator } => {
            assert_eq!(info.sender, CONFIG.load(deps.storage)?.owner);

            let unenroll_target = deps.api.addr_validate(&validator)?;

            let validators = VALIDATORS.load(deps.storage, domain)?;

            validators.

            let mut validators: Validators = validators
                .0
                .into_iter()
                .filter(|v| v.signer != unenroll_target)
                .collect();

            validators.0.sort_by(|a, b| a.signer.cmp(&b.signer));

            VALIDATORS.save(deps.storage, domain, &validators)?;

            // TODO: define event
            Ok(Response::new())
        }

        SetThreshold(threshold) => {
            assert_eq!(info.sender, CONFIG.load(deps.storage)?.owner);

            // TODO: define event
            Ok(Response::new())
        }
        SetThresholds(thresholds) => {
            assert_eq!(info.sender, CONFIG.load(deps.storage)?.owner);

            // TODO: define event
            Ok(Response::new())
        }

        InitTransferOwnership(next_owner) => {
            assert_eq!(info.sender, CONFIG.load(deps.storage)?.owner);
            assert!(PENDING_OWNER.may_load(deps.storage)?.is_none());

            PENDING_OWNER.save(deps.storage, &deps.api.addr_validate(&next_owner)?)?;

            // TODO: define event
            Ok(Response::new())
        }
        FinishTransferOwnership() => {
            let pending_owner = PENDING_OWNER.may_load(deps.storage)?;

            assert!(pending_owner.is_some());
            assert_eq!(info.sender, PENDING_OWNER.load(deps.storage)?);

            let config = CONFIG.load(deps.storage)?;

            CONFIG.save(
                deps.storage,
                &Config {
                    owner: pending_owner.unwrap(),
                    ..config
                },
            )?;

            // TODO: define event
            Ok(Response::new())
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
