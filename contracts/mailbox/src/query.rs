use cosmwasm_std::{to_binary, Deps, HexBinary, QueryResponse};
use hpl_interface::mailbox::{CountResponse, MessageDeliveredResponse, RootResponse};

use crate::{
    state::{MESSAGE_PROCESSED, MESSAGE_TREE},
    ContractError,
};

pub fn get_delivered(deps: Deps, id: HexBinary) -> Result<QueryResponse, ContractError> {
    let delivered = MESSAGE_PROCESSED
        .load(deps.storage, id.into())
        .map_err(|_| ContractError::MessageNotFound {})?;

    Ok(to_binary(&MessageDeliveredResponse { delivered })?)
}

pub fn get_root(deps: Deps) -> Result<QueryResponse, ContractError> {
    let root = MESSAGE_TREE.load(deps.storage)?.root()?.into();

    Ok(to_binary(&RootResponse { root })?)
}

pub fn get_count(deps: Deps) -> Result<QueryResponse, ContractError> {
    let count = MESSAGE_TREE.load(deps.storage)?.count;

    Ok(to_binary(&CountResponse { count })?)
}
