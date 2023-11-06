use cosmwasm_std::{ensure_eq, DepsMut, Event, HexBinary, MessageInfo, Response, StdResult};
use hpl_interface::ism::multisig::{ThresholdSet, ValidatorSet as MsgValidatorSet};
use hpl_ownable::get_owner;

use crate::{
    event::{emit_enroll_validator, emit_set_threshold, emit_unenroll_validator},
    state::{THRESHOLD, VALIDATORS},
    ContractError,
};

pub fn set_threshold(
    deps: DepsMut,
    info: MessageInfo,
    threshold: ThresholdSet,
) -> Result<Response, ContractError> {
    ensure_eq!(
        get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized
    );
    THRESHOLD.save(deps.storage, threshold.domain, &threshold.threshold)?;

    Ok(Response::new().add_event(emit_set_threshold(threshold.domain, threshold.threshold)))
}

pub fn set_thresholds(
    deps: DepsMut,
    info: MessageInfo,
    thresholds: Vec<ThresholdSet>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized
    );

    let events: Vec<Event> = thresholds
        .into_iter()
        .map(|v| {
            THRESHOLD.save(deps.storage, v.domain, &v.threshold)?;
            Ok(emit_set_threshold(v.domain, v.threshold))
        })
        .collect::<StdResult<_>>()?;

    Ok(Response::new().add_events(events))
}

pub fn enroll_validator(
    deps: DepsMut,
    info: MessageInfo,
    msg: MsgValidatorSet,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
        ContractError::Unauthorized {}
    );

    ensure_eq!(
        msg.validator.len(),
        20,
        ContractError::invalid_addr("length should be 20")
    );

    let validator_state = VALIDATORS.may_load(deps.storage, msg.domain)?;

    if let Some(mut validators) = validator_state {
        if validators.contains(&msg.validator) {
            return Err(ContractError::ValidatorDuplicate {});
        }

        validators.push(msg.validator.clone());
        validators.sort();

        VALIDATORS.save(deps.storage, msg.domain, &validators)?;
    } else {
        VALIDATORS.save(deps.storage, msg.domain, &vec![msg.validator.clone()])?;
    }

    Ok(Response::new().add_event(emit_enroll_validator(msg.domain, msg.validator.to_hex())))
}

pub fn enroll_validators(
    deps: DepsMut,
    info: MessageInfo,
    validators: Vec<MsgValidatorSet>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
        ContractError::Unauthorized {}
    );

    let mut events: Vec<Event> = Vec::new();

    for msg in validators.into_iter() {
        ensure_eq!(
            msg.validator.len(),
            20,
            ContractError::invalid_addr("length should be 20")
        );

        let validators_state = VALIDATORS.may_load(deps.storage, msg.domain)?;

        if let Some(mut validators) = validators_state {
            if validators.contains(&msg.validator) {
                return Err(ContractError::ValidatorDuplicate {});
            }

            validators.push(msg.validator.clone());
            validators.sort();

            VALIDATORS.save(deps.storage, msg.domain, &validators)?;
            events.push(emit_enroll_validator(msg.domain, msg.validator.to_hex()));
        } else {
            VALIDATORS.save(deps.storage, msg.domain, &vec![msg.validator.clone()])?;
            events.push(emit_enroll_validator(msg.domain, msg.validator.to_hex()));
        }
    }

    Ok(Response::new().add_events(events))
}

pub fn unenroll_validator(
    deps: DepsMut,
    info: MessageInfo,
    domain: u32,
    validator: HexBinary,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
        ContractError::Unauthorized {}
    );

    let validators = VALIDATORS
        .load(deps.storage, domain)
        .map_err(|_| ContractError::ValidatorNotExist {})?;

    if !validators.contains(&validator) {
        return Err(ContractError::ValidatorNotExist {});
    }

    let mut validator_list: Vec<HexBinary> =
        validators.into_iter().filter(|v| v != &validator).collect();

    validator_list.sort();

    VALIDATORS.save(deps.storage, domain, &validator_list)?;

    Ok(Response::new().add_event(emit_unenroll_validator(domain, validator.to_hex())))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr, HexBinary, Storage,
    };
    use hpl_interface::{
        build_test_executor, build_test_querier,
        ism::multisig::{ExecuteMsg, ValidatorSet},
    };
    use ibcx_test_utils::{addr, hex};
    use rstest::rstest;

    use crate::state::VALIDATORS;

    build_test_executor!(crate::contract::execute);
    build_test_querier!(crate::contract::query);

    use super::*;
    const ADDR1_VAULE: &str = "addr1";
    const ADDR2_VAULE: &str = "addr2";

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        hpl_ownable::initialize(storage, &owner).unwrap();
    }

    #[test]
    fn test_set_threshold() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked(ADDR1_VAULE);
        mock_owner(deps.as_mut().storage, owner.clone());

        let threshold = ThresholdSet {
            domain: 1u32,
            threshold: 8u8,
        };

        // set_threshold failure test
        let info = mock_info(ADDR2_VAULE, &[]);
        let fail_result = set_threshold(deps.as_mut(), info, threshold.clone()).unwrap_err();

        assert!(matches!(fail_result, ContractError::Unauthorized {}));

        // set_threshold success test
        let info = mock_info(owner.as_str(), &[]);
        let result = set_threshold(deps.as_mut(), info, threshold.clone()).unwrap();

        assert_eq!(
            result.events,
            vec![emit_set_threshold(threshold.domain, threshold.threshold)]
        );

        // check it actually saved
        let saved_threshold = THRESHOLD.load(&deps.storage, threshold.domain).unwrap();
        assert_eq!(saved_threshold, threshold.threshold);
    }

    #[test]
    fn test_set_thresholds() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked(ADDR1_VAULE);
        mock_owner(deps.as_mut().storage, owner.clone());

        let thresholds: Vec<ThresholdSet> = vec![
            ThresholdSet {
                domain: 1u32,
                threshold: 8u8,
            },
            ThresholdSet {
                domain: 2u32,
                threshold: 7u8,
            },
            ThresholdSet {
                domain: 3u32,
                threshold: 6u8,
            },
        ];

        // set_threshold failure test
        let info = mock_info(ADDR2_VAULE, &[]);
        let fail_result = set_thresholds(deps.as_mut(), info, thresholds.clone()).unwrap_err();

        assert!(matches!(fail_result, ContractError::Unauthorized {}));

        // set_threshold success test
        let info = mock_info(owner.as_str(), &[]);
        let result = set_thresholds(deps.as_mut(), info, thresholds.clone()).unwrap();

        assert_eq!(
            result.events,
            vec![
                emit_set_threshold(1u32, 8u8),
                emit_set_threshold(2u32, 7u8),
                emit_set_threshold(3u32, 6u8),
            ]
        );

        // check it actually saved
        for threshold in thresholds {
            let saved_threshold = THRESHOLD.load(&deps.storage, threshold.domain).unwrap();
            assert_eq!(saved_threshold, threshold.threshold);
        }
    }

    #[rstest]
    #[case("owner", vec![hex(&"deadbeef".repeat(5))])]
    #[should_panic(expected = "unauthorized")]
    #[case("someone", vec![hex(&"deadbeef".repeat(5))])]
    #[should_panic(expected = "duplicate validator")]
    #[case("owner", vec![hex(&"deadbeef".repeat(5)),hex(&"deadbeef".repeat(5))])]
    fn test_enroll(#[case] sender: &str, #[case] validators: Vec<HexBinary>) {
        let mut deps = mock_dependencies();

        hpl_ownable::initialize(deps.as_mut().storage, &addr("owner")).unwrap();

        for validator in validators.clone() {
            test_execute(
                deps.as_mut(),
                &addr(sender),
                ExecuteMsg::EnrollValidator {
                    set: ValidatorSet {
                        domain: 1,
                        validator,
                    },
                },
                vec![],
            );
        }

        assert_eq!(
            VALIDATORS.load(deps.as_ref().storage, 1).unwrap(),
            validators
        );
    }

    #[rstest]
    #[case("owner", hex("deadbeef"))]
    #[should_panic(expected = "unauthorized")]
    #[case("someone", hex("deadbeef"))]
    #[should_panic(expected = "validator not exist")]
    #[case("owner", hex("debeefed"))]
    fn test_unenroll(#[case] sender: &str, #[case] target: HexBinary) {
        let mut deps = mock_dependencies();

        hpl_ownable::initialize(deps.as_mut().storage, &addr("owner")).unwrap();

        VALIDATORS
            .save(deps.as_mut().storage, 1, &vec![hex("deadbeef")])
            .unwrap();

        test_execute(
            deps.as_mut(),
            &addr(sender),
            ExecuteMsg::UnenrollValidator {
                domain: 1,
                validator: target,
            },
            vec![],
        );

        assert!(VALIDATORS
            .load(deps.as_ref().storage, 1)
            .unwrap()
            .is_empty());
    }
}
