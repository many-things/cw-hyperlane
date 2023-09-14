use crate::constant::DEFAULT_GAS_USAGE;
use crate::error::ContractError;
use crate::execute::quote_gas_price;
use crate::state::GAS_ORACLE;
use cosmwasm_std::{to_binary, Deps, QueryResponse, Uint256};
use hpl_interface::igp_core::{GetExchangeRateAndGasPriceResponse, QuoteGasPaymentResponse};
use hpl_interface::igp_gas_oracle;
use hpl_interface::post_dispatch_hook::PostDispatchQueryMsg;
use hpl_interface::types::message::Message;
use hpl_interface::types::metadata::IGPMetadata;

pub fn quote_gas_payment(
    deps: Deps,
    dest_domain: u32,
    gas_amount: Uint256,
) -> Result<QueryResponse, ContractError> {
    let gas_needed = quote_gas_price(deps.storage, &deps.querier, dest_domain, gas_amount)?;

    Ok(to_binary(&QuoteGasPaymentResponse { gas_needed })?)
}

pub fn get_exchange_rate_and_gas_price(
    deps: Deps,
    dest_domain: u32,
) -> Result<QueryResponse, ContractError> {
    let gas_oracle = GAS_ORACLE
        .may_load(deps.storage, dest_domain)?
        .ok_or(ContractError::GasOracleNotFound {})?;

    let gas_price_resp: igp_gas_oracle::GetExchangeRateAndGasPriceResponse =
        deps.querier.query_wasm_smart(
            gas_oracle,
            &igp_gas_oracle::QueryMsg::GetExchangeRateAndGasPrice { dest_domain },
        )?;

    Ok(to_binary(&GetExchangeRateAndGasPriceResponse {
        gas_price: gas_price_resp.gas_price,
        exchange_rate: gas_price_resp.exchange_rate,
    })?)
}

pub fn quote_dispatch(
    deps: Deps,
    msg: PostDispatchQueryMsg,
) -> Result<QueryResponse, ContractError> {
    match msg {
        PostDispatchQueryMsg::QuoteDispatch { metadata, message } => {
            let igp_metadata: IGPMetadata = metadata.clone().into();
            let gas_limit = match metadata.len() < 32 {
                true => Uint256::from(DEFAULT_GAS_USAGE),
                false => igp_metadata.gas_limit,
            };
            let igp_message: Message = message.into();

            quote_gas_payment(deps, igp_message.dest_domain, gas_limit)
        }
    }
}
