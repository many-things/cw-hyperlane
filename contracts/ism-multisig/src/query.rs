use std::collections::HashSet;

use cosmwasm_std::{to_binary, Binary, Deps, HexBinary, QueryResponse};
use hpl_interface::{
    ism::{ISMType, VerifyResponse},
    types::{message::Message, metadata::MessageIdMultisigIsmMetadata},
};

use crate::{
    state::{THRESHOLD, VALIDATORS},
    verify::uncompress_pubkey,
    ContractError,
};

pub fn get_module_type() -> Result<QueryResponse, ContractError> {
    Ok(to_binary(&ISMType::Owned)?)
}

pub fn verify_message(
    deps: Deps,
    raw_metadata: HexBinary,
    raw_message: HexBinary,
) -> Result<QueryResponse, ContractError> {
    let metadata: MessageIdMultisigIsmMetadata = raw_metadata.into();
    let message: Message = raw_message.into();

    let threshold = THRESHOLD.load(deps.storage, message.origin_domain.into())?;
    let validators = VALIDATORS.load(deps.storage, message.origin_domain.into())?;

    let mut signatures: Vec<Binary> = Vec::new();
    for i in 0..metadata.signatures_len().unwrap() {
        signatures.push(metadata.signature_at(i))
    }

    let unique_vali_pubkey: HashSet<_> = validators
        .0
        .into_iter()
        .map(|v| uncompress_pubkey(v.signer_pubkey).unwrap())
        .collect();

    let unique_meta_pubkey: HashSet<_> = signatures
        .into_iter()
        .flat_map(|sig| {
            [
                deps.api
                    .secp256k1_recover_pubkey(&message.id(), &sig[0..64], 0)
                    .unwrap(),
                deps.api
                    .secp256k1_recover_pubkey(&message.id(), &sig[0..64], 1)
                    .unwrap(),
            ]
        })
        .map(Binary::from)
        .collect();

    let success = unique_vali_pubkey
        .intersection(&unique_meta_pubkey)
        .collect::<Vec<_>>()
        .len();

    Ok(to_binary(&VerifyResponse {
        verified: success >= usize::from(threshold),
    })?)
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::mock_dependencies, to_binary, Addr, Binary, HexBinary};
    use hpl_interface::{
        ism::{ISMType, VerifyResponse},
        types::{message::Message, metadata::MessageIdMultisigIsmMetadata},
    };

    use crate::state::{ValidatorSet, Validators, THRESHOLD, VALIDATORS};

    use super::{get_module_type, verify_message};

    #[test]
    fn test_get_module_type() {
        let result = get_module_type().unwrap();

        assert_eq!(result, to_binary(&ISMType::Owned).unwrap());
    }

    #[test]
    fn test_get_verify_failure() {
        let hex = |v: &str| -> Binary { HexBinary::from_hex(v).unwrap().into() };

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
                message.origin_domain.into(),
                &Validators(vec![
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1pql3lj3kftaf5pn507y74xfxlew0tufs8tey2k"),
                        signer_pubkey: Binary::from_base64(
                            "ArU5zD28GiZu6HZIonZP9thauVOERZ7y5dR4fIhyT1gc",
                        )
                        .unwrap(),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo13t2lcawapgppddj9hf0qk5yrrcvrre5gkslkat"),
                        signer_pubkey: Binary::from_base64(
                            "Av7oSL6LjqONDrrp+xGozb6pPyDErM9A/eT4f/IT0Jgh",
                        )
                        .unwrap(),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1wjfete3kxrhyzcuhdp3lc6g3a8r275dp80w9xd"),
                        signer_pubkey: Binary::from_base64(
                            "Ang2D1Fu8PkMF1/OXeN8xyHxIngO+pVvF+4Iu/y+XLEB",
                        )
                        .unwrap(),
                    },
                ]),
            )
            .unwrap();

        THRESHOLD
            .save(deps.as_mut().storage, message.origin_domain.into(), &2u8)
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
        assert_eq!(
            fail_result,
            to_binary(&VerifyResponse { verified: false }).unwrap()
        );
    }

    #[test]
    fn test_get_verify_success() {
        let hex = |v: &str| -> Binary { HexBinary::from_hex(v).unwrap().into() };

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
                message.origin_domain.into(),
                &Validators(vec![
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1pql3lj3kftaf5pn507y74xfxlew0tufs8tey2k"),
                        signer_pubkey: Binary::from_base64(
                            "ArU5zD28GiZu6HZIonZP9thauVOERZ7y5dR4fIhyT1gc",
                        )
                        .unwrap(),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo13t2lcawapgppddj9hf0qk5yrrcvrre5gkslkat"),
                        signer_pubkey: Binary::from_base64(
                            "Av7oSL6LjqONDrrp+xGozb6pPyDErM9A/eT4f/IT0Jgh",
                        )
                        .unwrap(),
                    },
                    ValidatorSet {
                        signer: Addr::unchecked("osmo1wjfete3kxrhyzcuhdp3lc6g3a8r275dp80w9xd"),
                        signer_pubkey: Binary::from_base64(
                            "Ang2D1Fu8PkMF1/OXeN8xyHxIngO+pVvF+4Iu/y+XLEB",
                        )
                        .unwrap(),
                    },
                ]),
            )
            .unwrap();

        THRESHOLD
            .save(deps.as_mut().storage, message.origin_domain.into(), &2u8)
            .unwrap();

        // success
        let success_signatures = hex("65a7fc45f77bb968620bf8f1f1845a1d2555f392c7c6ec0eb712429dd52cbea932dbe005e7e0c1e48ff17554f165bb4914b706a13d5cba1f5d41b7b3b142293300cbc3e12bb30133a5126d47b479e3da529c770b413d6e8e3a8f769055352efbfb36a5b950a2c6d129ef847f7f3f609fe1bd93fa6b3e2e656fb1056c583594fe1f00c7a687530639a9043bc4c39153332afc59b5bc3db2026423b62aab03f9cf7c7f4d28b9895aa9f5a16741702449d2478c776d64927609c30bb55119850b9878e700");

        let success_metadata = MessageIdMultisigIsmMetadata {
            origin_mailbox: hex("0000000000000000000000000000000000000000000000000000000000000000"),
            merkle_root: hex("0000000000000000000000000000000000000000000000000000000000000000"),
            signatures: success_signatures,
        };

        let success_result =
            verify_message(deps.as_ref(), success_metadata.into(), message.into()).unwrap();
        assert_eq!(
            success_result,
            to_binary(&VerifyResponse { verified: true }).unwrap()
        );
    }
}
