use crate::error::ContractError;
use crate::{DEFAULT_GAS_USAGE, TOKEN_EXCHANGE_RATE_SCALE};

use cosmwasm_std::{to_binary, Addr, Deps, QuerierWrapper, QueryResponse, Storage, Uint256};
use hpl_interface::hook::QuoteDispatchMsg;
use hpl_interface::igp::core::QuoteGasPaymentResponse;
use hpl_interface::igp::oracle::{GetExchangeRateAndGasPriceResponse, IgpGasOracleQueryMsg};
use hpl_interface::types::{IGPMetadata, Message};

pub fn quote_gas_price(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    dest_domain: u32,
    gas_amount: Uint256,
) -> Result<Uint256, ContractError> {
    let gas_oracle_set = hpl_router::get_route::<Addr>(storage, dest_domain)?;
    let gas_oracle = gas_oracle_set
        .route
        .ok_or(ContractError::GasOracleNotFound {})?;

    let gas_price_resp: GetExchangeRateAndGasPriceResponse = querier.query_wasm_smart(
        gas_oracle,
        &IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { dest_domain },
    )?;

    let dest_gas_cost = gas_amount * Uint256::from(gas_price_resp.gas_price);
    let gas_needed = (dest_gas_cost * Uint256::from(gas_price_resp.exchange_rate))
        / Uint256::from(TOKEN_EXCHANGE_RATE_SCALE);

    Ok(gas_needed)
}

pub fn quote_gas_payment(
    deps: Deps,
    dest_domain: u32,
    gas_amount: Uint256,
) -> Result<QueryResponse, ContractError> {
    let gas_needed = quote_gas_price(deps.storage, &deps.querier, dest_domain, gas_amount)?;

    Ok(to_binary(&QuoteGasPaymentResponse { gas_needed })?)
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
