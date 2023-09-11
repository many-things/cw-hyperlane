use crate::constant::{DEFAULT_GAS_USAGE, TOKEN_EXCHANGE_RATE_SCALE};
use crate::error::ContractError;
use crate::event::{emit_claim, emit_pay_for_gas, emit_set_beneficiary, emit_set_gas_oracles};
use crate::state::{BENEFICIARY, CONFIG, GAS_ORACLE, GAS_TOKEN};
use cosmwasm_std::{
    coins, ensure, ensure_eq, BankMsg, Binary, DepsMut, Env, MessageInfo, QuerierWrapper, Response,
    Storage, Uint128, Uint256,
};
use cw_utils::PaymentError;
use hpl_interface::igp_core::GasOracleConfig;
use hpl_interface::igp_gas_oracle;
use hpl_interface::types::message::Message;
use hpl_interface::types::metadata::IGPMetadata;
use std::str::FromStr;

pub fn quote_gas_price(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    dest_domain: u32,
    gas_amount: Uint256,
) -> Result<Uint256, ContractError> {
    let gas_oracle = GAS_ORACLE
        .may_load(storage, dest_domain)?
        .ok_or(ContractError::GasOracleNotFound {})?;

    let gas_price_resp: igp_gas_oracle::GetExchangeRateAndGasPriceResponse = querier
        .query_wasm_smart(
            gas_oracle,
            &igp_gas_oracle::QueryMsg::GetExchangeRateAndGasPrice { dest_domain },
        )?;

    let dest_gas_cost = gas_amount * Uint256::from(gas_price_resp.gas_price);
    let gas_needed = (dest_gas_cost * Uint256::from(gas_price_resp.exchange_rate))
        / Uint256::from(TOKEN_EXCHANGE_RATE_SCALE);

    Ok(gas_needed)
}

pub fn set_gas_oracle(
    deps: DepsMut,
    info: MessageInfo,
    configs: Vec<GasOracleConfig>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::OWNER.load(deps.storage)?,
        ContractError::Unauthorized {}
    );

    let mut domains = vec![];
    for c in configs {
        domains.push(c.remote_domain.to_string());

        GAS_ORACLE.save(
            deps.storage,
            c.remote_domain,
            &deps.api.addr_validate(&c.gas_oracle)?,
        )?;
    }

    Ok(Response::new().add_event(emit_set_gas_oracles(info.sender, domains)))
}

pub fn set_beneficiary(
    deps: DepsMut,
    info: MessageInfo,
    beneficiary: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::OWNER.load(deps.storage)?,
        ContractError::Unauthorized {}
    );

    BENEFICIARY.save(deps.storage, &deps.api.addr_validate(&beneficiary)?)?;

    Ok(Response::new().add_event(emit_set_beneficiary(info.sender, beneficiary)))
}

pub fn claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let beneficiary = BENEFICIARY.load(deps.storage)?;
    ensure_eq!(info.sender, beneficiary, ContractError::Unauthorized {});

    let gas_token = GAS_TOKEN.load(deps.storage)?;

    let balance = deps
        .querier
        .query_balance(env.contract.address, gas_token)?;

    let send_msg = BankMsg::Send {
        to_address: beneficiary.to_string(),
        amount: vec![balance.clone()],
    };

    Ok(Response::new()
        .add_message(send_msg)
        .add_event(emit_claim(beneficiary, balance)))
}

pub fn post_dispatch(
    deps: DepsMut,
    info: MessageInfo,
    metadata: Binary,
    raw_message: Binary,
) -> Result<Response, ContractError> {
    let igp_metadata: IGPMetadata = metadata.clone().into();
    let message: Message = raw_message.into();
    let prefix = CONFIG.load(deps.storage)?.prefix;

    let gas_limit = match metadata.0.len() < 32 {
        true => Uint256::from(DEFAULT_GAS_USAGE),
        false => igp_metadata.gas_limit,
    };
    let refund_address =
        igp_metadata.get_refund_address(prefix.clone(), message.sender_addr(prefix.as_str())?);
    pay_for_gas(
        deps,
        info,
        message.id(),
        message.dest_domain,
        gas_limit,
        refund_address.to_string(),
    )
}

pub fn pay_for_gas(
    deps: DepsMut,
    info: MessageInfo,
    message_id: Binary,
    dest_domain: u32,
    gas_amount: Uint256,
    refund_address: String,
) -> Result<Response, ContractError> {
    let gas_token = GAS_TOKEN.load(deps.storage)?;
    let received = Uint256::from(cw_utils::must_pay(&info, &gas_token)?);
    let gas_needed = quote_gas_price(deps.storage, &deps.querier, dest_domain, gas_amount)?;
    ensure!(received >= gas_needed, PaymentError::NonPayable {});

    let payment_gap = Uint128::from_str(&(received - gas_needed).to_string())?;

    let mut resp = Response::new();

    if !payment_gap.is_zero() {
        let refund_msg = BankMsg::Send {
            to_address: refund_address,
            amount: coins(payment_gap.u128(), &gas_token),
        };
        resp = resp.add_message(refund_msg);
    }

    Ok(resp.add_event(emit_pay_for_gas(
        info.sender,
        message_id,
        gas_amount,
        payment_gap,
        gas_needed,
        received,
    )))
}
