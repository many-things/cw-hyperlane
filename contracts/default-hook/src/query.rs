use cosmwasm_std::{to_binary, Deps, Env, QueryResponse};
use hpl_interface::{
    domain_routing_hook::{OwnerResponse, PauseInfoResponse},
    igp_core::QuoteGasPaymentResponse,
    post_dispatch_hook::PostDispatchQueryMsg,
    types::message::Message,
};

use crate::{
    state::{HOOK_CONFIG, PAUSE},
    ContractError,
};

pub fn get_pause_info(deps: Deps) -> Result<QueryResponse, ContractError> {
    Ok(to_binary(&PauseInfoResponse {
        paused: PAUSE.load(deps.storage)?,
    })?)
}

pub fn get_owner_info(deps: Deps) -> Result<QueryResponse, ContractError> {
    Ok(to_binary(&OwnerResponse {
        owner: hpl_ownable::OWNER.load(deps.storage)?.to_string(),
    })?)
}

pub fn quote_dispatch(
    deps: Deps,
    _env: Env,
    msg: PostDispatchQueryMsg,
) -> Result<QueryResponse, ContractError> {
    match msg {
        PostDispatchQueryMsg::QuoteDispatch { metadata, message } => {
            let hpl_msg: Message = message.clone().into();
            let target_contract = HOOK_CONFIG
                .load(deps.storage, hpl_msg.dest_domain)
                .map_err(|_| ContractError::HookNotRegistered(hpl_msg.dest_domain))?;

            let result: QuoteGasPaymentResponse = deps.querier.query_wasm_smart(
                target_contract.hook,
                &PostDispatchQueryMsg::QuoteDispatch { metadata, message },
            )?;

            Ok(to_binary(&QuoteGasPaymentResponse {
                gas_needed: result.gas_needed,
            })?)
        }
    }
}
