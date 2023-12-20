use crate::error::ContractError;
use crate::{BENEFICIARY, DEFAULT_GAS_USAGE, GAS_FOR_DOMAIN, GAS_TOKEN, TOKEN_EXCHANGE_RATE_SCALE};

use cosmwasm_std::{coins, Addr, Deps, QuerierWrapper, StdResult, Storage, Uint256};
use hpl_interface::hook::{MailboxResponse, QuoteDispatchMsg, QuoteDispatchResponse};
use hpl_interface::igp::core::{
    BeneficiaryResponse, DefaultGasResponse, GasForDomainResponse, QuoteGasPaymentResponse,
};
use hpl_interface::igp::oracle::{self, GetExchangeRateAndGasPriceResponse, IgpGasOracleQueryMsg};
use hpl_interface::types::{IGPMetadata, Message};
use hpl_interface::Order;

pub fn get_mailbox(_deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: "unrestricted".to_string(),
    })
}

pub fn get_default_gas(deps: Deps) -> Result<DefaultGasResponse, ContractError> {
    let default_gas = DEFAULT_GAS_USAGE.load(deps.storage)?;

    Ok(DefaultGasResponse { gas: default_gas })
}

pub fn get_gas_for_domain(
    deps: Deps,
    domains: Vec<u32>,
) -> Result<GasForDomainResponse, ContractError> {
    Ok(GasForDomainResponse {
        gas: domains
            .into_iter()
            .map(|v| Ok((v, GAS_FOR_DOMAIN.load(deps.storage, v)?)))
            .collect::<StdResult<_>>()?,
    })
}

pub fn list_gas_for_domains(
    deps: Deps,
    offset: Option<u32>,
    limit: Option<u32>,
    order: Option<Order>,
) -> Result<GasForDomainResponse, ContractError> {
    let ((min, max), limit, order) = hpl_interface::range_option(offset, limit, order)?;

    let gas = GAS_FOR_DOMAIN
        .range(deps.storage, min, max, order.into())
        .take(limit)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(GasForDomainResponse { gas })
}

pub fn get_beneficiary(deps: Deps) -> Result<BeneficiaryResponse, ContractError> {
    let beneficiary = BENEFICIARY.load(deps.storage)?;

    Ok(BeneficiaryResponse {
        beneficiary: beneficiary.into(),
    })
}

pub fn quote_gas_price(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    dest_domain: u32,
    fees: Uint256,
) -> Result<Uint256, ContractError> {
    let gas_oracle_set = hpl_router::get_route::<Addr>(storage, dest_domain)?;
    let gas_oracle = gas_oracle_set
        .route
        .ok_or(ContractError::GasOracleNotFound(dest_domain))?;

    let gas_price_resp: GetExchangeRateAndGasPriceResponse = querier.query_wasm_smart(
        gas_oracle,
        &oracle::QueryMsg::Oracle(IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { dest_domain }),
    )?;

    let dest_gas_cost = fees * Uint256::from(gas_price_resp.gas_price);
    let gas_needed = (dest_gas_cost * Uint256::from(gas_price_resp.exchange_rate))
        / Uint256::from(TOKEN_EXCHANGE_RATE_SCALE);

    Ok(gas_needed)
}

pub fn quote_gas_payment(
    deps: Deps,
    dest_domain: u32,
    fees: Uint256,
) -> Result<QuoteGasPaymentResponse, ContractError> {
    let gas_needed = quote_gas_price(deps.storage, &deps.querier, dest_domain, fees)?;

    Ok(QuoteGasPaymentResponse { gas_needed })
}

pub fn quote_dispatch(
    deps: Deps,
    req: QuoteDispatchMsg,
) -> Result<QuoteDispatchResponse, ContractError> {
    let igp_message: Message = req.message.into();

    let gas_limit = match req.metadata.len() < 32 {
        true => Uint256::from(crate::get_default_gas(
            deps.storage,
            igp_message.dest_domain,
        )?),
        false => {
            let igp_metadata: IGPMetadata = req.metadata.clone().into();
            igp_metadata.gas_limit
        }
    };

    let fees = quote_gas_payment(deps, igp_message.dest_domain, gas_limit)?.gas_needed;
    let fees = if !fees.is_zero() {
        coins(
            fees.to_string().parse::<u128>()?,
            GAS_TOKEN.load(deps.storage)?,
        )
    } else {
        vec![]
    };

    Ok(QuoteDispatchResponse { fees })
}

pub fn get_exchange_rate_and_gas_price(
    deps: Deps,
    dest_domain: u32,
) -> Result<GetExchangeRateAndGasPriceResponse, ContractError> {
    let gas_oracle_set = hpl_router::get_route::<Addr>(deps.storage, dest_domain)?;
    let gas_oracle = gas_oracle_set
        .route
        .ok_or(ContractError::GasOracleNotFound(dest_domain))?;

    Ok(deps.querier.query_wasm_smart(
        gas_oracle,
        &IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { dest_domain }.wrap(),
    )?)
}
