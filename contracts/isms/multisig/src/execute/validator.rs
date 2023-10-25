use cosmwasm_std::{ensure_eq, DepsMut, Event, HexBinary, MessageInfo, Response};
use hpl_interface::ism::multisig::ValidatorSet as MsgValidatorSet;
use hpl_ownable::get_owner;

use crate::{
    event::{emit_enroll_validator, emit_unenroll_validator},
    state::VALIDATORS,
    ContractError,
};

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
    use cosmwasm_std::{testing::mock_dependencies, HexBinary};
    use hpl_interface::{
        build_test_executor, build_test_querier,
        ism::multisig::{ExecuteMsg, ValidatorSet},
    };
    use ibcx_test_utils::{addr, hex};
    use rstest::rstest;

    use crate::state::VALIDATORS;

    build_test_executor!(crate::contract::execute);
    build_test_querier!(crate::contract::query);

    #[rstest]
    #[case("owner", vec![hex("deadbeef")])]
    #[should_panic(expected = "unauthorized")]
    #[case("someone", vec![hex("deadbeef")])]
    #[should_panic(expected = "duplicate validator")]
    #[case("owner", vec![hex("deadbeef"),hex("deadbeef")])]
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
