use cosmwasm_std::{Deps, HexBinary};
use hpl_interface::{
    ism::{IsmType, ModuleTypeResponse, VerifyInfoResponse, VerifyResponse},
    types::{Message, MessageIdMultisigIsmMetadata},
};

use crate::{
    domain_hash, eth_hash, multisig_hash,
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

    let threshold = THRESHOLD.load(deps.storage, message.origin_domain)?;
    let validators = VALIDATORS.load(deps.storage, message.origin_domain)?;

    let merkle_index = metadata.merkle_index();

    let verifiable_cases = product(
        validators.0.into_iter().map(|v| v.signer_pubkey).collect(),
        metadata.signatures,
    );

    let multisig_hash = multisig_hash(
        domain_hash(message.origin_domain, metadata.origin_merkle_tree)?.to_vec(),
        metadata.merkle_root.to_vec(),
        merkle_index,
        message.id().to_vec(),
    )?;

    let hashed_message = eth_hash(multisig_hash)?;

    let success: u8 = verifiable_cases
        .into_iter()
        .map(|v| {
            deps.api
                .secp256k1_verify(&hashed_message, &v.1[0..64], &v.0)
                .unwrap() as u8
        })
        .sum();

    Ok(VerifyResponse {
        verified: success >= threshold,
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
        validators: validators
            .0
            .into_iter()
            .map(|v| v.signer.to_string())
            .collect(),
    })
}

#[cfg(test)]
mod test {
    use crate::{
        query::get_verify_info,
        state::{ValidatorSet, Validators, THRESHOLD, VALIDATORS},
    };
    use cosmwasm_std::testing::mock_dependencies;
    use hpl_interface::{
        ism::{IsmType, ModuleTypeResponse, VerifyInfoResponse, VerifyResponse},
        types::{bech32_encode, Message},
    };
    use ibcx_test_utils::{addr, hex};

    use super::{get_module_type, verify_message};

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

        let signer = hex("f9e25a6be80f6d48727e42381fc3c3b7834c0cb4");
        let signer = bech32_encode("osmo", signer.as_slice()).unwrap();
        let signer_pubkey =
            hex("039cdc58e622e25767cfa565802534b2f777107a3382f46a88323471bbd3d84c22");

        let mut deps = mock_dependencies();

        let message: Message = raw_message.clone().into();

        VALIDATORS
            .save(
                deps.as_mut().storage,
                message.origin_domain,
                &Validators(vec![ValidatorSet {
                    signer,
                    signer_pubkey,
                }]),
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
        let mut deps = mock_dependencies();

        let message = Message {
            version: 0,
            nonce: 8528,
            origin_domain: 44787,
            sender: hex("000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f16"),
            dest_domain: 11155111,
            recipient: hex("0000000000000000000000005d56b8a669f50193b54319442c6eee5edd662381"),
            body: hex("48656c6c6f21"),
        };

        VALIDATORS
            .save(
                deps.as_mut().storage,
                message.origin_domain,
                &Validators(vec![
                    ValidatorSet {
                        signer: addr("osmo1pql3lj3kftaf5pn507y74xfxlew0tufs8tey2k"),
                        signer_pubkey: hex(
                            "02b539cc3dbc1a266ee87648a2764ff6d85ab95384459ef2e5d4787c88724f581c",
                        ),
                    },
                    ValidatorSet {
                        signer: addr("osmo13t2lcawapgppddj9hf0qk5yrrcvrre5gkslkat"),
                        signer_pubkey: hex(
                            "02fee848be8b8ea38d0ebae9fb11a8cdbea93f20c4accf40fde4f87ff213d09821",
                        ),
                    },
                    ValidatorSet {
                        signer: addr("osmo1wjfete3kxrhyzcuhdp3lc6g3a8r275dp80w9xd"),
                        signer_pubkey: hex(
                            "0278360f516ef0f90c175fce5de37cc721f122780efa956f17ee08bbfcbe5cb101",
                        ),
                    },
                ]),
            )
            .unwrap();

        THRESHOLD
            .save(deps.as_mut().storage, message.origin_domain, &2u8)
            .unwrap();

        let success_result = get_verify_info(deps.as_ref(), message.into()).unwrap();
        assert_eq!(
            success_result,
            VerifyInfoResponse {
                threshold: 2,
                validators: vec![
                    "osmo1pql3lj3kftaf5pn507y74xfxlew0tufs8tey2k".to_string(),
                    "osmo13t2lcawapgppddj9hf0qk5yrrcvrre5gkslkat".to_string(),
                    "osmo1wjfete3kxrhyzcuhdp3lc6g3a8r275dp80w9xd".to_string(),
                ]
            }
        );
    }
}
