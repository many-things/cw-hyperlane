use crate::error::ContractError;
use crate::execute::quote_gas_price;
use crate::DEFAULT_GAS_USAGE;

use cosmwasm_std::{to_binary, Addr, Deps, QueryResponse, Uint256};
use hpl_interface::hook::QuoteDispatchMsg;
use hpl_interface::igp::core::QuoteGasPaymentResponse;
use hpl_interface::igp::gas_oracle::{GetExchangeRateAndGasPriceResponse, IgpGasOracleQueryMsg};
use hpl_interface::types::{IGPMetadata, Message};

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
    let gas_oracle_set = hpl_router::get_route::<Addr>(deps.storage, dest_domain)?;
    let gas_oracle = gas_oracle_set
        .route
        .ok_or(ContractError::GasOracleNotFound {})?;

    let gas_price_resp: GetExchangeRateAndGasPriceResponse = deps.querier.query_wasm_smart(
        gas_oracle,
        &IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { dest_domain }.wrap(),
    )?;

    Ok(to_binary(&GetExchangeRateAndGasPriceResponse {
        gas_price: gas_price_resp.gas_price,
        exchange_rate: gas_price_resp.exchange_rate,
    })?)
}

pub fn quote_dispatch(deps: Deps, req: QuoteDispatchMsg) -> Result<QueryResponse, ContractError> {
    let igp_metadata: IGPMetadata = req.metadata.clone().into();
    let gas_limit = match req.metadata.len() < 32 {
        true => Uint256::from(DEFAULT_GAS_USAGE),
        false => igp_metadata.gas_limit,
    };
    let igp_message: Message = req.message.into();

    quote_gas_payment(deps, igp_message.dest_domain, gas_limit)
}
