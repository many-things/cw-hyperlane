use cosmwasm_std::{Binary, DepsMut, Event, MessageInfo, Response};
use hpl_interface::ism::multisig::ValidatorSet as MsgValidatorSet;

use crate::{
    event::{emit_enroll_validator, emit_unenroll_validator},
    state::{assert_owned, ValidatorSet, Validators, CONFIG, VALIDATORS},
    verify::{self},
    ContractError,
};

fn assert_pubkey_validate(
    validator: String,
    pubkey: Binary,
    addr_prefix: String,
) -> Result<(), ContractError> {
    let pub_to_addr = verify::pub_to_addr(pubkey, &addr_prefix)?;

    if validator != pub_to_addr {
        return Err(ContractError::ValidatorPubKeyMismatched {});
    }

    Ok(())
}

pub fn enroll_validator(
    deps: DepsMut,
    info: MessageInfo,
    msg: MsgValidatorSet,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender)?;

    let config = CONFIG.load(deps.storage)?;
    assert_pubkey_validate(
        msg.validator.clone(),
        msg.validator_pubkey.clone(),
        config.addr_prefix,
    )?;

    let candidate = deps.api.addr_validate(&msg.validator)?;
    let mut validators = VALIDATORS.load(deps.storage, msg.domain)?;

    if !validators.0.iter().any(|v| v.signer == candidate) {
        return Err(ContractError::ValidatorDuplicate {});
    }

    validators.0.push(ValidatorSet {
        signer: candidate,
        signer_pubkey: msg.validator_pubkey,
    });
    validators.0.sort_by(|a, b| a.signer.cmp(&b.signer));

    VALIDATORS.save(deps.storage, msg.domain, &validators)?;
    Ok(Response::new().add_event(emit_enroll_validator(msg.domain, msg.validator)))
}

pub fn enroll_validators(
    deps: DepsMut,
    info: MessageInfo,
    validators: Vec<MsgValidatorSet>,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender)?;

    let config = CONFIG.load(deps.storage)?;
    let mut events: Vec<Event> = Vec::new();

    for msg in validators.into_iter() {
        assert_pubkey_validate(
            msg.validator.clone(),
            msg.validator_pubkey.clone(),
            config.addr_prefix.clone(),
        )?;

        let candidate = deps.api.addr_validate(&msg.validator)?;
        let mut validators = VALIDATORS.load(deps.storage, msg.domain)?;

        if !validators.0.iter().any(|v| v.signer == candidate) {
            return Err(ContractError::ValidatorDuplicate {});
        }

        validators.0.push(ValidatorSet {
            signer: candidate,
            signer_pubkey: msg.validator_pubkey,
        });
        validators.0.sort_by(|a, b| a.signer.cmp(&b.signer));

        VALIDATORS.save(deps.storage, msg.domain, &validators)?;
        events.push(emit_enroll_validator(msg.domain, msg.validator))
    }

    Ok(Response::new().add_events(events.into_iter()))
}

pub fn unenroll_validator(
    deps: DepsMut,
    info: MessageInfo,
    domain: u64,
    validator: String,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender)?;

    let unenroll_target = deps.api.addr_validate(&validator)?;

    let validators = VALIDATORS.load(deps.storage, domain)?;

    let mut validator_list: Vec<ValidatorSet> = validators
        .0
        .into_iter()
        .filter(|v| v.signer != unenroll_target)
        .collect();

    validator_list.sort_by(|a, b| a.signer.cmp(&b.signer));

    VALIDATORS.save(deps.storage, domain, &Validators(validator_list))?;
    Ok(Response::new().add_event(emit_unenroll_validator(domain, validator)))
}

#[cfg(test)]
mod test {}
