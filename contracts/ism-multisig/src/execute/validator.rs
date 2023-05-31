use cosmwasm_std::{DepsMut, MessageInfo, Response};
use hpl_interface::ism::multisig::ValidatorSet as MsgValidatorSet;

use crate::{
    state::{ValidatorSet, Validators, CONFIG, VALIDATORS},
    verify::{self},
    ContractError,
};

pub fn enroll_validator(
    deps: DepsMut,
    info: MessageInfo,
    msg: MsgValidatorSet,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_eq!(info.sender, config.owner, "unauthorized");
    assert_eq!(
        msg.validator,
        verify::pub_to_addr(msg.validator_pubkey.clone(), &config.chain_hpl)?,
        "addr, pubkey mismatch"
    );

    let candidate = deps.api.addr_validate(&msg.validator)?;
    let mut validators = VALIDATORS.load(deps.storage, msg.domain)?;

    assert!(
        !validators.0.iter().any(|v| v.signer == candidate),
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

pub fn enroll_validators(
    deps: DepsMut,
    info: MessageInfo,
    validators: Vec<MsgValidatorSet>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_eq!(info.sender, config.owner);

    for msg in validators {
        assert_eq!(
            msg.validator,
            verify::pub_to_addr(msg.validator_pubkey.clone(), &config.chain_hpl)?,
            "addr, pubkey mismatch"
        );

        let candidate = deps.api.addr_validate(&msg.validator)?;
        let mut validators = VALIDATORS.load(deps.storage, msg.domain)?;

        assert!(
            !validators.0.iter().any(|v| v.signer == candidate),
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

pub fn unenroll_validator(
    deps: DepsMut,
    info: MessageInfo,
    domain: u64,
    validator: String,
) -> Result<Response, ContractError> {
    assert_eq!(info.sender, CONFIG.load(deps.storage)?.owner);

    let unenroll_target = deps.api.addr_validate(&validator)?;

    let validators = VALIDATORS.load(deps.storage, domain)?;

    let mut validator_list: Vec<ValidatorSet> = validators
        .0
        .into_iter()
        .filter(|v| v.signer != unenroll_target)
        .collect();

    validator_list.sort_by(|a, b| a.signer.cmp(&b.signer));

    VALIDATORS.save(deps.storage, domain, &Validators(validator_list))?;

    // TODO: define event
    Ok(Response::new())
}
