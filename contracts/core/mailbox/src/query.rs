use cosmwasm_std::{to_binary, Deps, HexBinary, QueryResponse};
use hpl_interface::mailbox::{
    CheckPointResponse, CountResponse, DefaultIsmResponse, MerkleTreeResponse,
    MessageDeliveredResponse, NonceResponse, PausedResponse, RootResponse,
};

use crate::{
    merkle::ZERO_BYTES,
    state::{CONFIG, DELIVERY, MESSAGE_TREE, NONCE, PAUSE},
    verify, ContractError,
};

pub fn get_delivered(deps: Deps, id: HexBinary) -> Result<QueryResponse, ContractError> {
    let delivered = DELIVERY.has(deps.storage, id.into());

    if !delivered {
        return Err(ContractError::MessageNotFound {});
    }

    Ok(to_binary(&MessageDeliveredResponse { delivered })?)
}

pub fn get_root(deps: Deps) -> Result<QueryResponse, ContractError> {
    let root = MESSAGE_TREE.load(deps.storage)?.root()?.into();

    Ok(to_binary(&RootResponse { root })?)
}

pub fn get_count(deps: Deps) -> Result<QueryResponse, ContractError> {
    let count = MESSAGE_TREE.load(deps.storage)?.count;

    Ok(to_binary(&CountResponse {
        count: count as u32,
    })?)
}

pub fn get_checkpoint(deps: Deps) -> Result<QueryResponse, ContractError> {
    let tree = MESSAGE_TREE.load(deps.storage)?;

    Ok(to_binary(&CheckPointResponse {
        root: tree.root()?.into(),
        count: tree.count as u32,
    })?)
}

pub fn get_tree(deps: Deps) -> Result<QueryResponse, ContractError> {
    let tree = MESSAGE_TREE.load(deps.storage)?;
    let branch: Vec<HexBinary> = tree
        .branch
        .into_iter()
        .map(|x| {
            if x.is_empty() {
                HexBinary::from_hex(ZERO_BYTES).unwrap()
            } else {
                x.into()
            }
        })
        .collect();

    Ok(to_binary(&MerkleTreeResponse {
        branch: branch.try_into().unwrap(),
        count: tree.count as u32,
    })?)
}

pub fn get_paused(deps: Deps) -> Result<QueryResponse, ContractError> {
    let paused = PAUSE.load(deps.storage)?;
    Ok(to_binary(&PausedResponse { paused })?)
}

pub fn get_nonce(deps: Deps) -> Result<QueryResponse, ContractError> {
    let nonce = NONCE.load(deps.storage)?;
    Ok(to_binary(&NonceResponse { nonce })?)
}

pub fn get_default_ism(deps: Deps) -> Result<QueryResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    Ok(to_binary(&DefaultIsmResponse {
        default_ism: verify::bech32_decode(config.default_ism).into(),
    })?)
}

#[cfg(test)]
mod test {

    use crate::{
        merkle::{MerkleTree, ZERO_HASHES},
        state::Delivery,
    };

    use super::*;
    use cosmwasm_std::{from_binary, testing::mock_dependencies, Addr, HexBinary};

    #[test]
    fn test_query_tree() {
        let mut deps = mock_dependencies();

        MESSAGE_TREE
            .save(deps.as_mut().storage, &MerkleTree::default())
            .unwrap();

        let tree: MerkleTreeResponse = from_binary(&get_tree(deps.as_ref()).unwrap()).unwrap();

        assert_eq!(tree.branch.len(), 32);
        for (i, branch) in tree.branch.iter().enumerate() {
            assert_eq!(branch, &HexBinary::from_hex(ZERO_HASHES[i]).unwrap());
        }
        assert_eq!(tree.count, 0);
    }

    #[test]
    fn test_get_delivery() {
        let mut deps = mock_dependencies();
        let id = HexBinary::from_hex("c0ffeedeadbeef").unwrap();
        let ism = Addr::unchecked("ism");
        // cannot find deps delivery
        let notfound_resp = get_delivered(deps.as_ref(), id.clone()).unwrap_err();
        assert!(matches!(notfound_resp, ContractError::MessageNotFound {}));

        // set delivery
        DELIVERY
            .save(deps.as_mut().storage, id.clone().into(), &Delivery { ism })
            .unwrap();

        let resp = get_delivered(deps.as_ref(), id).unwrap();
        assert_eq!(
            resp,
            to_binary(&MessageDeliveredResponse { delivered: true }).unwrap()
        );
    }
}
