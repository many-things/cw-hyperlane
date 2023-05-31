use cosmwasm_std::{DepsMut, Event, MessageInfo, Response};
use hpl_interface::ism::multisig::ThresholdSet;

use crate::{
    event::emit_set_threshold,
    state::{CONFIG, THRESHOLD},
    ContractError,
};

pub fn set_threshold(
    deps: DepsMut,
    info: MessageInfo,
    threshold: ThresholdSet,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_eq!(info.sender, config.owner, "unauthorized");

    THRESHOLD.save(deps.storage, threshold.domain, &threshold.threshold)?;

    Ok(Response::new().add_event(emit_set_threshold(threshold.domain, threshold.threshold)))
}

pub fn set_thresholds(
    deps: DepsMut,
    info: MessageInfo,
    thresholds: Vec<ThresholdSet>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_eq!(info.sender, config.owner, "unauthorized");

    let mut events: Vec<Event> = Vec::new();

    for threshold in thresholds.into_iter() {
        THRESHOLD.save(deps.storage, threshold.domain, &threshold.threshold)?;
        events.push(emit_set_threshold(threshold.domain, threshold.threshold))
    }

    Ok(Response::new().add_events(events.into_iter()))
}
