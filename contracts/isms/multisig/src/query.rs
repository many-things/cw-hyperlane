use cosmwasm_std::{Deps, HexBinary};
use hpl_interface::{
    ism::{IsmType, ModuleTypeResponse, ModulesAndThresholdResponse, VerifyResponse},
    types::{eth_addr, eth_hash, Message, MessageIdMultisigIsmMetadata},
};

use crate::{
    domain_hash, multisig_hash,
    state::{THRESHOLD, VALIDATORS},
    ContractError,
};

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
    let validators = VALIDATORS.load(deps.storage, message.origin_domain)?;
    let mut threshold = THRESHOLD.load(deps.storage, message.origin_domain)?;

    for signature in metadata.signatures {
        let signature = signature.as_slice();
        let pubkey = deps.api.secp256k1_recover_pubkey(
            &hashed_message,
            &signature[..64],
            signature[64] - 27,
        )?;

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

pub fn modules_and_threshold(
    deps: Deps,
    raw_message: HexBinary,
) -> Result<ModulesAndThresholdResponse, ContractError> {
    let message: Message = raw_message.into();

    let threshold = THRESHOLD.load(deps.storage, message.origin_domain)?;
    let validators = VALIDATORS.load(deps.storage, message.origin_domain)?;

    Ok(ModulesAndThresholdResponse {
        threshold,
        modules: validators,
    })
}

#[cfg(test)]
mod test {
    use crate::state::{THRESHOLD, VALIDATORS};
    use cosmwasm_std::{testing::mock_dependencies, HexBinary};
    use hpl_interface::{
        ism::{IsmType, ModuleTypeResponse, VerifyResponse},
        types::{eth_addr, Message},
    };
    use ibcx_test_utils::hex;
    use k256::{ecdsa::SigningKey, elliptic_curve::rand_core::OsRng};
    use rstest::rstest;

    use super::{get_module_type, modules_and_threshold, verify_message};

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

    #[rstest]
    #[case(
        hex("0000000000000068220000000000000000000000000d1255b09d94659bb0888e0aa9fca60245ce402a0000682155208cd518cffaac1b5d8df216a9bd050c9a03f0d4f3ba88e5268ac4cd12ee2d68656c6c6f"),
        hex("986a1625d44e4b3969b08a5876171b2b4fcdf61b3e5c70a86ad17b304f17740a9f45d99ea6bec61392a47684f4e5d1416ddbcb5fdef0f132c27d7034e9bbff1c00000000ba9911d78ec6d561413e3589f920388cbd7554fbddd8ce50739337250853ec3577a51fa40e727c05b50f15db13f5aad5857c89d432644be48d70325ea83fdb6c1c"),        
        vec![
            hex("122e0663ccc190266427e7fc0ed6589b5d7d36db"),
            hex("01d7525e91dfc3f594fd366aad70f956b398de9e"),
        ]
    )]
    #[case(
        hex("03000000240001388100000000000000000000000004980c17e2ce26578c82f81207e706e4505fae3b0000a8690000000000000000000000000b1c1b54f45e02552331d3106e71f5e0b573d5d448656c6c6f21"),
        hex("0000000000000000000000009af85731edd41e2e50f81ef8a0a69d2fb836edf9a84430f822e0e9b5942faace72bd5b97f0b59a58a9b8281231d9e5c393b5859c00000024539feceace17782697e29e74151006dc7b47227cf48aba02926336cb5f7fa38b3d05e8293045f7b5811eda3ae8aa070116bb5fbf57c79e143a69e909df90cefa1b6e6ead7180e0415c36642ee4bc5454bc4f5ca250ca77a1a83562035544e0e898734d6541a20404e05fd53eb1c75b0bd21851c3bd8122cfa3550d7b6fb94d7cee1b"),
        vec![
            hex("ebc301013b6cd2548e347c28d2dc43ec20c068f2"),
            hex("315db9868fc8813b221b1694f8760ece39f45447"),
            hex("17517c98358c5937c5d9ee47ce1f5b4c2b7fc9f5"),
        ]
    )]
    fn test_verify_with_e2e_data(
        #[case] raw_message: HexBinary,
        #[case] raw_metadata: HexBinary,
        #[case] validators: Vec<HexBinary>,
    ) {
        let mut deps = mock_dependencies();

        let message: Message = raw_message.clone().into();

        VALIDATORS
            .save(deps.as_mut().storage, message.origin_domain, &validators)
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

        let info = modules_and_threshold(deps.as_ref(), raw_message).unwrap();

        assert_eq!(info.modules, vec![addr]);
        assert_eq!(info.threshold, 1);
    }
}
