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

// #[cfg(test)]
// mod test {
//     use cosmwasm_std::{
//         testing::{mock_dependencies, mock_info},
//         Addr, Storage,
//     };

//     use super::*;
//     const ADDR1_VAULE: &str = "addr1";
//     const ADDR2_VAULE: &str = "addr2";

//     const VALIDATOR: &str = "E000fA4E466831dB288290Dd97e66560fb3d7d28";

//     fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
//         hpl_ownable::initialize(storage, &owner).unwrap();
//     }

//     #[test]
//     fn test_enroll_validator_failure() {
//         let mut deps = mock_dependencies();
//         let owner = Addr::unchecked(ADDR1_VAULE);

//         mock_owner(deps.as_mut().storage, owner.clone());

//         HRP.save(deps.as_mut().storage, &VAL_HRP.into()).unwrap();

//         let msg = MsgValidatorSet {
//             domain: 1u32,
//             validator: "test".to_string(),
//             validator_pubkey: HexBinary::from_hex(VALIDATOR_PUBKEY).unwrap(),
//         };

//         // unauthorized
//         let info = mock_info(ADDR2_VAULE, &[]);
//         let unauthorize_resp = enroll_validator(deps.as_mut(), info, msg).unwrap_err();
//         assert!(matches!(unauthorize_resp, ContractError::Unauthorized {}));

//         // already exist pubkey
//         let valid_message = MsgValidatorSet {
//             domain: 1u32,
//             validator: VALIDATOR_ADDR.to_string(),
//             validator_pubkey: HexBinary::from_hex(VALIDATOR_PUBKEY).unwrap(),
//         };
//         VALIDATORS
//             .save(
//                 deps.as_mut().storage,
//                 1u32,
//                 &Validators(vec![ValidatorSet {
//                     signer: Addr::unchecked(valid_message.validator.clone()),
//                     signer_pubkey: valid_message.validator_pubkey.clone(),
//                 }]),
//             )
//             .unwrap();

//         let info = mock_info(owner.as_str(), &[]);
//         let duplicate_pubkey = enroll_validator(deps.as_mut(), info, valid_message).unwrap_err();
//         assert!(matches!(
//             duplicate_pubkey,
//             ContractError::ValidatorDuplicate {}
//         ))
//     }

//     #[test]
//     fn test_enroll_validator_success() {
//         let mut deps = mock_dependencies();
//         let owner = Addr::unchecked(ADDR1_VAULE);
//         let validator: String = VALIDATOR_ADDR.to_string();
//         let domain: u32 = 1;

//         HRP.save(deps.as_mut().storage, &VAL_HRP.into()).unwrap();

//         mock_owner(deps.as_mut().storage, owner.clone());
//         let msg = MsgValidatorSet {
//             domain,
//             validator: validator.clone(),
//             validator_pubkey: HexBinary::from_hex(VALIDATOR_PUBKEY).unwrap(),
//         };

//         // validators not exist
//         let info = mock_info(ADDR1_VAULE, &[]);
//         let result = enroll_validator(deps.as_mut(), info, msg.clone()).unwrap();

//         assert_eq!(
//             result.events,
//             vec![emit_enroll_validator(1u32, validator.clone())]
//         );

//         // check it actually save
//         let saved_validators = VALIDATORS.load(&deps.storage, domain).unwrap();
//         assert_eq!(validator, saved_validators.0[0].signer);

//         // validator is exist already
//         VALIDATORS
//             .save(
//                 deps.as_mut().storage,
//                 1u32,
//                 &Validators(vec![ValidatorSet {
//                     signer: Addr::unchecked(ADDR2_VAULE),
//                     signer_pubkey: msg.validator_pubkey.clone(),
//                 }]),
//             )
//             .unwrap();

//         let info = mock_info(owner.as_str(), &[]);
//         let result = enroll_validator(deps.as_mut(), info, msg).unwrap();

//         assert_eq!(
//             result.events,
//             vec![emit_enroll_validator(1u32, validator.clone())]
//         );
//         let saved_validators = VALIDATORS.load(&deps.storage, domain).unwrap();
//         assert_eq!(validator, saved_validators.0.last().unwrap().signer);
//     }

//     #[test]
//     fn test_enroll_validators_failure() {
//         let mut deps = mock_dependencies();
//         let owner = Addr::unchecked(ADDR1_VAULE);

//         mock_owner(deps.as_mut().storage, owner);

//         HRP.save(deps.as_mut().storage, &VAL_HRP.into()).unwrap();

//         let msg = vec![
//             MsgValidatorSet {
//                 domain: 1u32,
//                 validator: String::from(VALIDATOR_ADDR),
//                 validator_pubkey: HexBinary::from_hex(VALIDATOR_PUBKEY).unwrap(),
//             },
//             MsgValidatorSet {
//                 domain: 1u32,
//                 validator: String::from(VALIDATOR_ADDR),
//                 validator_pubkey: HexBinary::from_hex(VALIDATOR_PUBKEY).unwrap(),
//             },
//         ];

//         let info = mock_info(ADDR2_VAULE, &[]);
//         let unauthorized = enroll_validators(deps.as_mut(), info, msg.clone()).unwrap_err();
//         assert!(matches!(unauthorized, ContractError::Unauthorized {}));

//         let info = mock_info(ADDR1_VAULE, &[]);
//         let duplicated = enroll_validators(deps.as_mut(), info, msg).unwrap_err();
//         assert!(matches!(duplicated, ContractError::ValidatorDuplicate {}));
//     }

//     #[test]
//     fn test_enroll_validators_success() {
//         let mut deps = mock_dependencies();
//         let owner = Addr::unchecked(ADDR1_VAULE);
//         let validator = String::from(VALIDATOR_ADDR);
//         let validator_pubkey = HexBinary::from_hex(VALIDATOR_PUBKEY).unwrap();
//         mock_owner(deps.as_mut().storage, owner.clone());

//         HRP.save(deps.as_mut().storage, &VAL_HRP.into()).unwrap();

//         let msg = vec![
//             MsgValidatorSet {
//                 domain: 1u32,
//                 validator: validator.clone(),
//                 validator_pubkey: validator_pubkey.clone(),
//             },
//             MsgValidatorSet {
//                 domain: 2u32,
//                 validator: validator.clone(),
//                 validator_pubkey: validator_pubkey.clone(),
//             },
//         ];

//         VALIDATORS
//             .save(
//                 deps.as_mut().storage,
//                 2u32,
//                 &Validators(vec![ValidatorSet {
//                     signer: Addr::unchecked(ADDR2_VAULE),
//                     signer_pubkey: validator_pubkey,
//                 }]),
//             )
//             .unwrap();

//         let info = mock_info(owner.as_str(), &[]);
//         let result = enroll_validators(deps.as_mut(), info, msg).unwrap();

//         assert_eq!(
//             result.events,
//             vec![
//                 emit_enroll_validator(1u32, validator.clone()),
//                 emit_enroll_validator(2u32, validator.clone())
//             ]
//         );

//         // check it actually saved
//         assert_eq!(
//             validator,
//             VALIDATORS
//                 .load(&deps.storage, 1u32)
//                 .unwrap()
//                 .0
//                 .last()
//                 .unwrap()
//                 .signer
//         );
//         assert_eq!(
//             validator,
//             VALIDATORS
//                 .load(&deps.storage, 2u32)
//                 .unwrap()
//                 .0
//                 .last()
//                 .unwrap()
//                 .signer
//         );
//     }

//     #[test]
//     fn test_unenroll_validator_failure() {
//         let mut deps = mock_dependencies();
//         let owner = Addr::unchecked(ADDR1_VAULE);
//         let validator = String::from(VALIDATOR_ADDR);
//         let domain: u32 = 1;

//         mock_owner(deps.as_mut().storage, owner.clone());

//         // unauthorization
//         let info = mock_info(ADDR2_VAULE, &[]);
//         let unauthorized =
//             unenroll_validator(deps.as_mut(), info, domain, validator.clone()).unwrap_err();
//         assert!(matches!(unauthorized, ContractError::Unauthorized {}));

//         // not exists
//         let info = mock_info(owner.as_str(), &[]);
//         let not_exist_state =
//             unenroll_validator(deps.as_mut(), info.clone(), domain, validator.clone()).unwrap_err();
//         assert!(matches!(
//             not_exist_state,
//             ContractError::ValidatorNotExist {}
//         ));

//         // not exists in exist state
//         VALIDATORS
//             .save(
//                 deps.as_mut().storage,
//                 1u32,
//                 &Validators(vec![ValidatorSet {
//                     signer: Addr::unchecked(ADDR2_VAULE),
//                     signer_pubkey: HexBinary::from_hex(VALIDATOR_PUBKEY).unwrap(),
//                 }]),
//             )
//             .unwrap();
//         let not_exist_state =
//             unenroll_validator(deps.as_mut(), info, domain, validator).unwrap_err();
//         assert!(matches!(
//             not_exist_state,
//             ContractError::ValidatorNotExist {}
//         ));
//     }

//     #[test]
//     fn test_unenroll_validator_success() {
//         let mut deps = mock_dependencies();
//         let owner = Addr::unchecked(ADDR1_VAULE);
//         let validator = String::from(VALIDATOR_ADDR);
//         let domain: u32 = 1;

//         mock_owner(deps.as_mut().storage, owner.clone());

//         let info = mock_info(owner.as_str(), &[]);
//         VALIDATORS
//             .save(
//                 deps.as_mut().storage,
//                 domain,
//                 &Validators(vec![ValidatorSet {
//                     signer: Addr::unchecked(validator.clone()),
//                     signer_pubkey: HexBinary::from_hex(VALIDATOR_PUBKEY).unwrap(),
//                 }]),
//             )
//             .unwrap();
//         let result = unenroll_validator(deps.as_mut(), info, domain, validator.clone()).unwrap();

//         assert_eq!(
//             result.events,
//             vec![emit_unenroll_validator(domain, validator)]
//         );
//         assert_eq!(VALIDATORS.load(&deps.storage, domain).unwrap().0.len(), 0)
//     }
// }
