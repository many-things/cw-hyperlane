use cosmwasm_std::{to_binary, Deps, HexBinary, QueryResponse};
use hpl_interface::mailbox::MessageDeliveredResponse;

use crate::{state::MESSAGE_PROCESSED, ContractError};

pub fn get_delivered(deps: Deps, id: HexBinary) -> Result<QueryResponse, ContractError> {
    let delivered = MESSAGE_PROCESSED
        .load(deps.storage, id.into())
        .map_err(|_| ContractError::MessageNotFound {})?;

    Ok(to_binary(&MessageDeliveredResponse { delivered })?)
}
