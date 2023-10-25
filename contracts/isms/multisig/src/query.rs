use cosmwasm_std::{Deps, HexBinary};
use hpl_interface::{
    ism::{IsmType, ModuleTypeResponse, VerifyInfoResponse, VerifyResponse},
    types::{eth_addr, eth_hash, Message, MessageIdMultisigIsmMetadata},
};

use crate::{
    domain_hash, multisig_hash,
    state::{THRESHOLD, VALIDATORS},
    ContractError,
};

fn product<T: Clone, U: Clone>(x: Vec<T>, y: Vec<U>) -> Vec<(T, U)> {
    x.iter()
        .flat_map(|item_x| y.iter().map(move |item_y| (item_x.clone(), item_y.clone())))
        .collect()
}

pub fn get_module_type() -> Result<ModuleTypeResponse, ContractError> {
    Ok(ModuleTypeResponse {
        typ: IsmType::MessageIdMultisig,
    })
}

pub fn verify_message(
    deps: Deps,
    raw_metadata: HexBinary,
    raw_message: HexBinary,
) -> Result<VerifyResponse, ContractError> {
    deps.api.debug(&format!(
        "ism_multisig::verify: metadata: {:?}, message: {:?}",
        raw_metadata, raw_message
    ));

    let metadata: MessageIdMultisigIsmMetadata = raw_metadata.into();
    let message: Message = raw_message.into();

    let merkle_index = metadata.merkle_index();

    let multisig_hash = multisig_hash(
        domain_hash(message.origin_domain, metadata.origin_merkle_tree)?.to_vec(),
        metadata.merkle_root.to_vec(),
        merkle_index,
        message.id().to_vec(),
    )?;

    let hashed_message = eth_hash(multisig_hash)?;

    // pizza :)
    let comb = product(metadata.signatures, vec![0u8, 1u8]);

    let validators = VALIDATORS.load(deps.storage, message.origin_domain)?;
    let mut threshold = THRESHOLD.load(deps.storage, message.origin_domain)?;

    for (signature, recovery_param) in comb {
        let signature = signature.as_slice();
        let pubkey =
            deps.api
                .secp256k1_recover_pubkey(&hashed_message, &signature[..64], recovery_param)?;

        if validators.contains(&eth_addr(pubkey.into())?) {
            threshold -= 1;
            if threshold == 0 {
                break;
            }
        }
    }

    Ok(VerifyResponse {
        verified: threshold == 0,
    })
}

pub fn get_verify_info(
    deps: Deps,
    raw_message: HexBinary,
) -> Result<VerifyInfoResponse, ContractError> {
    let message: Message = raw_message.into();

    let threshold = THRESHOLD.load(deps.storage, message.origin_domain)?;
    let validators = VALIDATORS.load(deps.storage, message.origin_domain)?;

    Ok(VerifyInfoResponse {
        threshold,
        validators,
    })
}

#[cfg(test)]
mod test {
    use crate::state::{THRESHOLD, VALIDATORS};
    use cosmwasm_std::testing::mock_dependencies;
    use hpl_interface::{
        ism::{IsmType, ModuleTypeResponse, VerifyResponse},
        types::{eth_addr, Message},
    };
    use ibcx_test_utils::hex;
    use k256::{ecdsa::SigningKey, elliptic_curve::rand_core::OsRng};

    use super::{get_module_type, get_verify_info, verify_message};

    #[test]
    fn test_get_module_type() {
        let result = get_module_type().unwrap();

        assert_eq!(
            result,
            ModuleTypeResponse {
                typ: IsmType::MessageIdMultisig
            }
        );
    }

    #[test]
    fn test_verify_with_e2e_data() {
        let raw_message = hex("0000000000000068220000000000000000000000000d1255b09d94659bb0888e0aa9fca60245ce402a0000682155208cd518cffaac1b5d8df216a9bd050c9a03f0d4f3ba88e5268ac4cd12ee2d68656c6c6f");
        let raw_metadata = hex("986a1625d44e4b3969b08a5876171b2b4fcdf61b3e5c70a86ad17b304f17740a9f45d99ea6bec61392a47684f4e5d1416ddbcb5fdef0f132c27d7034e9bbff1c00000000ba9911d78ec6d561413e3589f920388cbd7554fbddd8ce50739337250853ec3577a51fa40e727c05b50f15db13f5aad5857c89d432644be48d70325ea83fdb6c1c");

        let mut deps = mock_dependencies();

        let message: Message = raw_message.clone().into();

        VALIDATORS
            .save(
                deps.as_mut().storage,
                message.origin_domain,
                &vec![
                    hex("122e0663ccc190266427e7fc0ed6589b5d7d36db"),
                    hex("01d7525e91dfc3f594fd366aad70f956b398de9e"),
                ],
            )
            .unwrap();
        THRESHOLD
            .save(deps.as_mut().storage, message.origin_domain, &1u8)
            .unwrap();

        let res = verify_message(deps.as_ref(), raw_metadata, raw_message).unwrap();
        assert_eq!(res, VerifyResponse { verified: true });
    }

    #[test]
    fn test_get_verify_info() {
        let raw_message = hex("0000000000000068220000000000000000000000000d1255b09d94659bb0888e0aa9fca60245ce402a0000682155208cd518cffaac1b5d8df216a9bd050c9a03f0d4f3ba88e5268ac4cd12ee2d68656c6c6f");

        let mut deps = mock_dependencies();

        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let addr = eth_addr(verifying_key.to_encoded_point(false).as_bytes().into()).unwrap();

        VALIDATORS
            .save(deps.as_mut().storage, 26658, &vec![addr.clone()])
            .unwrap();
        THRESHOLD.save(deps.as_mut().storage, 26658, &1u8).unwrap();

        let info = get_verify_info(deps.as_ref(), raw_message).unwrap();

        assert_eq!(info.validators, vec![addr]);
        assert_eq!(info.threshold, 1);
    }
}
