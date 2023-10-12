use cosmwasm_std::{Deps, HexBinary};
use hpl_interface::{
    core::mailbox::{
        DefaultHookResponse, DefaultIsmResponse, HrpResponse, LocalDomainResponse,
        MessageDeliveredResponse, RecipientIsmResponse,
    },
    ism,
};

use crate::{
    state::{CONFIG, DELIVERIES},
    ContractError,
};

pub fn get_hrp(deps: Deps) -> Result<HrpResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(HrpResponse { hrp: config.hrp })
}

pub fn get_local_domain(deps: Deps) -> Result<LocalDomainResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(LocalDomainResponse {
        local_domain: config.local_domain,
    })
}

pub fn get_default_ism(deps: Deps) -> Result<DefaultIsmResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(DefaultIsmResponse {
        default_ism: config.get_default_ism().into(),
    })
}

pub fn get_default_hook(deps: Deps) -> Result<DefaultHookResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(DefaultHookResponse {
        default_hook: config.get_default_hook().into(),
    })
}

pub fn get_delivered(deps: Deps, id: HexBinary) -> Result<MessageDeliveredResponse, ContractError> {
    let delivered = DELIVERIES.has(deps.storage, id.to_vec());

    if !delivered {
        return Err(ContractError::MessageNotFound {});
    }

    Ok(MessageDeliveredResponse { delivered })
}

pub fn get_recipient_ism(
    deps: Deps,
    recipient: String,
) -> Result<RecipientIsmResponse, ContractError> {
    let recipient = deps.api.addr_validate(&recipient)?;

    let ism_resp: ism::InterchainSecurityModuleResponse = deps.querier.query_wasm_smart(
        recipient,
        &ism::ISMSpecifierQueryMsg::InterchainSecurityModule(),
    )?;

    let config = CONFIG.load(deps.storage)?;
    let default_ism = config.get_default_ism();

    Ok(RecipientIsmResponse {
        ism: ism_resp.ism.unwrap_or(default_ism).into(),
    })
}
