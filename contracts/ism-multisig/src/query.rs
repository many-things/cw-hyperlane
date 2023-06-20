use std::collections::HashSet;

use cosmwasm_std::{to_binary, Binary, Deps, HexBinary, QueryResponse};
use hpl_interface::{
    ism::{ISMType, VerifyResponse},
    types::{message::Message, metadata::MessageIdMultisigIsmMetadata},
};

use crate::{
    state::{THRESHOLD, VALIDATORS},
    ContractError,
};

pub fn get_module_type() -> Result<QueryResponse, ContractError> {
    Ok(to_binary(&ISMType::Owned)?)
}

pub fn get_verify(
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

    let unique_vali_pubkey: HashSet<_> =
        validators.0.into_iter().map(|v| v.signer_pubkey).collect();

    let unique_meta_pubkey: HashSet<_> = signatures
        .into_iter()
        .flat_map(|sig| {
            [
                deps.api
                    .secp256k1_recover_pubkey(&message.id(), sig.as_slice(), 0)
                    .unwrap(),
                deps.api
                    .secp256k1_recover_pubkey(&message.id(), sig.as_slice(), 1)
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
    use cosmwasm_std::to_binary;
    use hpl_interface::ism::ISMType;

    use super::get_module_type;

    #[test]
    fn test_get_module_type() {
        let result = get_module_type().unwrap();

        assert_eq!(result, to_binary(&ISMType::Owned).unwrap());
    }
}
