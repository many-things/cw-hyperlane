use cosmwasm_std::{Addr, Deps, HexBinary, QuerierWrapper};
use hpl_interface::ism;

use crate::{
    state::{assert_verify_response, CONFIG},
    ContractError,
};

/// Verifies the message using the ISM of the recipient, or the default ISM if the recipient does not have one.
pub fn ism_verify(
    querier: &QuerierWrapper,
    ism: &Addr,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<(), ContractError> {
    let verify_resp: ism::VerifyResponse =
        querier.query_wasm_smart(ism, &ism::ISMQueryMsg::Verify { metadata, message })?;

    assert_verify_response(verify_resp.verified)?;

    Ok(())
}

/// Returns the ISM address of the recipient, or the default ISM if the recipient does not have one.
pub fn recipient_ism(deps: Deps, receipient: &Addr) -> Result<Addr, ContractError> {
    let ism_resp: ism::InterchainSecurityModuleResponse = deps.querier.query_wasm_smart(
        receipient,
        &ism::ISMSpecifierQueryMsg::InterchainSecurityModule(),
    )?;

    let default_ism: Addr = CONFIG.load(deps.storage)?.default_ism;

    Ok(ism_resp.ism.unwrap_or(default_ism))
}
