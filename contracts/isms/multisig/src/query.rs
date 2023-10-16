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

pub fn get_module_type() -> Result<ModuleTypeResponse, ContractError> {
    Ok(ModuleTypeResponse {
        typ: IsmType::LegacyMultisig,
    })
}

pub fn verify_message(
    deps: Deps,
    raw_metadata: HexBinary,
    raw_message: HexBinary,
) -> Result<VerifyResponse, ContractError> {
    let metadata: MessageIdMultisigIsmMetadata = raw_metadata.into();
    let message: Message = raw_message.into();

    let threshold = THRESHOLD.load(deps.storage, message.origin_domain)?;
    let validators = VALIDATORS.load(deps.storage, message.origin_domain)?;

    let mut signatures: Vec<HexBinary> = Vec::new();
    for i in 0..metadata.signatures_len().unwrap() {
        signatures.push(metadata.signature_at(i))
    }

    let verifiable_cases = validators
        .0
        .into_iter()
        .map(|v| {
            signatures
                .clone()
                .into_iter()
                .map(|s| (v.signer_pubkey.clone(), s))
                .collect::<Vec<(HexBinary, HexBinary)>>()
        })
        .fold(vec![], |acc, item| acc.into_iter().chain(item).collect());

    let multisig_hash = multisig_hash(
        domain_hash(message.origin_domain, metadata.origin_mailbox)?.to_vec(),
        metadata.merkle_root.to_vec(),
        0,
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

    println!("success: {}", success);

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
    use crate::state::{ValidatorSet, Validators, THRESHOLD, VALIDATORS};
    use cosmwasm_std::{testing::mock_dependencies, Addr, Binary, HexBinary};
    use hpl_interface::{
        ism::{IsmType, ModuleTypeResponse, VerifyInfoResponse, VerifyResponse},
        types::{Message, MessageIdMultisigIsmMetadata},
    };

    use super::{get_module_type, get_verify_info, verify_message};

    #[test]
    fn test_get_module_type() {
        let result = get_module_type().unwrap();

        assert_eq!(
            result,
            ModuleTypeResponse {
                typ: IsmType::LegacyMultisig
            }
        );
    }

    fn hex(v: &str) -> HexBinary {
        HexBinary::from_hex(v).unwrap()
    }

    fn base64(v: &str) -> HexBinary {
        Binary::from_base64(v).unwrap().to_vec().into()
    }

    #[test]
    fn test_get_verify_failure() {
        let message = Message {
            version: 0,
            nonce: 8528,
            origin_domain: 44787,
            sender: hex("000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f16"),
            dest_domain: 11155111,
            recipient: hex("0000000000000000000000005d56b8a669f50193b54319442c6eee5edd662381"),
            body: hex("48656c6c6f21"),
        };

        let mut deps = mock_dependencies();
        VALIDATORS
            .save(
                deps.as_mut().storage,
                message.origin_domain,
                &Validators(vec![
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1pql3lj3kftaf5pn507y74xfxlew0tufs8tey2k"),
                        signer_pubkey: base64("ArU5zD28GiZu6HZIonZP9thauVOERZ7y5dR4fIhyT1gc"),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo13t2lcawapgppddj9hf0qk5yrrcvrre5gkslkat"),
                        signer_pubkey: base64("Av7oSL6LjqONDrrp+xGozb6pPyDErM9A/eT4f/IT0Jgh"),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1wjfete3kxrhyzcuhdp3lc6g3a8r275dp80w9xd"),
                        signer_pubkey: base64("Ang2D1Fu8PkMF1/OXeN8xyHxIngO+pVvF+4Iu/y+XLEB"),
                    },
                ]),
            )
            .unwrap();

        THRESHOLD
            .save(deps.as_mut().storage, message.origin_domain, &2u8)
            .unwrap();

        // fail
        let fail_signatures = hex("65a7fc45f77bb968620bf8f1f1845a1d2555f392c7c6ec0eb712429dd52cbea932dbe005e7e0c1e48ff17554f165bb4914b706a13d5cba1f5d41b7b3b142293300");

        let fail_metadata = MessageIdMultisigIsmMetadata {
            origin_mailbox: hex("0000000000000000000000000000000000000000000000000000000000000000"),
            merkle_root: hex("0000000000000000000000000000000000000000000000000000000000000000"),
            signatures: fail_signatures,
        };

        let fail_result =
            verify_message(deps.as_ref(), fail_metadata.into(), message.into()).unwrap();
        assert_eq!(fail_result, VerifyResponse { verified: false });
    }

    #[test]
    fn test_get_verify_success() {
        let raw_metadata: HexBinary = HexBinary::from_hex("0736a58fd7bd49e1f059768f8d57649670f6815054e49d3bb2ed16f71fc5ff16855a91bc5ca0e6853d2331e52759c8e8683da8bede7ac84b11b658af70dbdf1f494ddcd3802fd4934e7f3e89366c8761b57f60ebd438384d027512c36643c6b5413d05aa290d524414ec3e698eed65602204a25ed0b7ede6cca03a6370bd6b201c").unwrap();
        let raw_message: HexBinary = HexBinary::from_hex("0000000000000068210000000000000000000000000d1255b09d94659bb0888e0aa9fca60245ce402a000068226c29bd39dce3f038d2dbb9b608f02a73d7ddc03f9cfb176dedb1a64e50c7b3e368656c6c6f").unwrap();

        let message: Message = raw_message.clone().into();
        let mut deps = mock_dependencies();
        VALIDATORS
            .save(
                deps.as_mut().storage,
                message.origin_domain,
                &Validators(vec![
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1l83956lgpak5sun7ggupls7rk7p5cr95499jdf"),
                        signer_pubkey: base64("A5zcWOYi4ldnz6VlgCU0svd3EHozgvRqiDI0cbvT2Ewi"),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo13t2lcawapgppddj9hf0qk5yrrcvrre5gkslkat"),
                        signer_pubkey: base64("Av7oSL6LjqONDrrp+xGozb6pPyDErM9A/eT4f/IT0Jgh"),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1wjfete3kxrhyzcuhdp3lc6g3a8r275dp80w9xd"),
                        signer_pubkey: base64("Ang2D1Fu8PkMF1/OXeN8xyHxIngO+pVvF+4Iu/y+XLEB"),
                    },
                ]),
            )
            .unwrap();

        THRESHOLD
            .save(deps.as_mut().storage, message.origin_domain, &1u8)
            .unwrap();

        let success_result = verify_message(deps.as_ref(), raw_metadata, raw_message).unwrap();
        assert_eq!(success_result, VerifyResponse { verified: true });
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
                        signer: Addr::unchecked("osmo1pql3lj3kftaf5pn507y74xfxlew0tufs8tey2k"),
                        signer_pubkey: base64("ArU5zD28GiZu6HZIonZP9thauVOERZ7y5dR4fIhyT1gc"),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo13t2lcawapgppddj9hf0qk5yrrcvrre5gkslkat"),
                        signer_pubkey: base64("Av7oSL6LjqONDrrp+xGozb6pPyDErM9A/eT4f/IT0Jgh"),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1wjfete3kxrhyzcuhdp3lc6g3a8r275dp80w9xd"),
                        signer_pubkey: base64("Ang2D1Fu8PkMF1/OXeN8xyHxIngO+pVvF+4Iu/y+XLEB"),
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
