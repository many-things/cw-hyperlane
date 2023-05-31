use cosmwasm_std::{DepsMut, MessageInfo, Response};
use hpl_interface::ism::multisig::ThresholdSet;

use crate::ContractError;

pub fn set_threshold(
    deps: DepsMut,
    info: MessageInfo,
    threshold: ThresholdSet,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn set_thresholds(
    deps: DepsMut,
    info: MessageInfo,
    thresholds: Vec<ThresholdSet>,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
