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
    match VALIDATORS.may_load(deps.storage, msg.domain)? {
        // handle VALIDATORS domain not exists
        None => {
            let validators = Validators(vec![ValidatorSet {
                signer: candidate,
                signer_pubkey: msg.validator_pubkey,
            }]);

            VALIDATORS.save(deps.storage, msg.domain, &validators)?;
            Ok(Response::new().add_event(emit_enroll_validator(msg.domain, msg.validator)))
        }
        Some(mut validators) => {
            // TODO: exists
            if validators.0.iter().any(|v| v.signer == candidate) {
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
    }
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
        let validators_state = VALIDATORS.may_load(deps.storage, msg.domain)?;

        if let Some(mut validators) = validators_state {
            if validators.0.iter().any(|v| v.signer == candidate) {
                return Err(ContractError::ValidatorDuplicate {});
            }

            validators.0.push(ValidatorSet {
                signer: candidate,
                signer_pubkey: msg.validator_pubkey,
            });
            validators.0.sort_by(|a, b| a.signer.cmp(&b.signer));

            VALIDATORS.save(deps.storage, msg.domain, &validators)?;
            events.push(emit_enroll_validator(msg.domain, msg.validator));
        } else {
            let validators = Validators(vec![ValidatorSet {
                signer: candidate,
                signer_pubkey: msg.validator_pubkey,
            }]);

            VALIDATORS.save(deps.storage, msg.domain, &validators)?;
            events.push(emit_enroll_validator(msg.domain, msg.validator));
        }
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
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr, Storage,
    };

    use crate::state::{Config, CONFIG};

    use super::*;
    const ADDR1_VAULE: &str = "addr1";
    const ADDR2_VAULE: &str = "addr2";

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        let config = Config {
            owner,
            addr_prefix: "osmo".to_string(),
        };

        CONFIG.save(storage, &config).unwrap();
    }

    #[test]
    fn test_enroll_validator_failure() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked(ADDR1_VAULE);

        mock_owner(deps.as_mut().storage, owner);

        let msg = MsgValidatorSet {
            domain: 1u64,
            validator: "test".to_string(),
            validator_pubkey: Binary::from_base64("AzpZu8TLfx5xEFQeVL4f+N5qu3X+Fq2uokLFLQ16OEuv")
                .unwrap(),
        };

        // unauthorized
        let info = mock_info(ADDR2_VAULE, &[]);
        let unauthorize_resp = enroll_validator(deps.as_mut(), info, msg.clone()).unwrap_err();

        assert!(matches!(unauthorize_resp, ContractError::Unauthorized {}));

        // wrong pubkey
        let info = mock_info(ADDR1_VAULE, &[]);
        let wrong_pubkey_resp = enroll_validator(deps.as_mut(), info.clone(), msg).unwrap_err();
        assert!(matches!(
            wrong_pubkey_resp,
            ContractError::ValidatorPubKeyMismatched {}
        ));

        // already exist pubkey
        let valid_message = MsgValidatorSet {
            domain: 1u64,
            validator: "osmo1q28uzwtvvvlkz6k84gd7flu576x2l2ry9506p5".to_string(),
            validator_pubkey: Binary::from_base64("AzpZu8TLfx5xEFQeVL4f+N5qu3X+Fq2uokLFLQ16OEuv")
                .unwrap(),
        };
        VALIDATORS
            .save(
                deps.as_mut().storage,
                1u64,
                &Validators(vec![ValidatorSet {
                    signer: Addr::unchecked(valid_message.validator.clone()),
                    signer_pubkey: valid_message.validator_pubkey.clone(),
                }]),
            )
            .unwrap();

        let duplicate_pubkey = enroll_validator(deps.as_mut(), info, valid_message).unwrap_err();
        assert!(matches!(
            duplicate_pubkey,
            ContractError::ValidatorDuplicate {}
        ))
    }

    #[test]
    fn test_enroll_validator_success() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked(ADDR1_VAULE);
        let validator: String = "osmo1q28uzwtvvvlkz6k84gd7flu576x2l2ry9506p5".to_string();
        let domain: u64 = 1;

        mock_owner(deps.as_mut().storage, owner);
        let msg = MsgValidatorSet {
            domain,
            validator: validator.clone(),
            validator_pubkey: Binary::from_base64("AzpZu8TLfx5xEFQeVL4f+N5qu3X+Fq2uokLFLQ16OEuv")
                .unwrap(),
        };

        // validators not exist
        let info = mock_info(ADDR1_VAULE, &[]);
        let result = enroll_validator(deps.as_mut(), info, msg.clone()).unwrap();

        assert_eq!(
            result.events,
            vec![emit_enroll_validator(1u64, validator.clone())]
        );

        // check it actually save
        let saved_validators = VALIDATORS.load(&deps.storage, domain).unwrap();
        assert_eq!(validator, saved_validators.0[0].signer);

        // validator is exist already
        VALIDATORS
            .save(
                deps.as_mut().storage,
                1u64,
                &Validators(vec![ValidatorSet {
                    signer: Addr::unchecked(ADDR2_VAULE),
                    signer_pubkey: msg.validator_pubkey.clone(),
                }]),
            )
            .unwrap();

        let info = mock_info(ADDR1_VAULE, &[]);
        let result = enroll_validator(deps.as_mut(), info, msg).unwrap();

        assert_eq!(
            result.events,
            vec![emit_enroll_validator(1u64, validator.clone())]
        );
        let saved_validators = VALIDATORS.load(&deps.storage, domain).unwrap();
        assert_eq!(validator, saved_validators.0.last().unwrap().signer);
    }

    #[test]
    fn test_enroll_validators_failure() {}
}
