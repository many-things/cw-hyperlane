use crate::event::{
    emit_claim, emit_pay_for_gas, emit_post_dispatch, emit_set_beneficiary, emit_set_default_gas,
    emit_set_gas_for_domain, emit_unset_gas_for_domain,
};
use crate::query::quote_gas_price;
use crate::{
    get_default_gas, ContractError, BENEFICIARY, DEFAULT_GAS_USAGE, GAS_FOR_DOMAIN, GAS_TOKEN, HRP,
};

use cosmwasm_std::{
    coins, ensure, ensure_eq, BankMsg, DepsMut, Env, HexBinary, MessageInfo, Response, Uint128,
    Uint256,
};
use hpl_interface::{
    hook::PostDispatchMsg,
    types::{IGPMetadata, Message},
};
use hpl_ownable::get_owner;

use std::str::FromStr;

pub fn set_default_gas(
    deps: DepsMut,
    info: MessageInfo,
    default_gas: u128,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
        ContractError::Unauthorized {}
    );

    DEFAULT_GAS_USAGE.save(deps.storage, &default_gas)?;

    Ok(Response::new().add_event(emit_set_default_gas(info.sender, default_gas)))
}

pub fn set_gas_for_domain(
    deps: DepsMut,
    info: MessageInfo,
    config: Vec<(u32, u128)>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
        ContractError::Unauthorized {}
    );

    for (domain, custom_gas) in config.clone() {
        GAS_FOR_DOMAIN.save(deps.storage, domain, &custom_gas)?;
    }

    Ok(Response::new().add_event(emit_set_gas_for_domain(info.sender, config)))
}

pub fn unset_gas_for_domain(
    deps: DepsMut,
    info: MessageInfo,
    domains: Vec<u32>,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
        ContractError::Unauthorized {}
    );

    for domain in domains.clone() {
        GAS_FOR_DOMAIN.remove(deps.storage, domain);
    }

    Ok(Response::new().add_event(emit_unset_gas_for_domain(info.sender, domains)))
}

pub fn set_beneficiary(
    deps: DepsMut,
    info: MessageInfo,
    beneficiary: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
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
    req: PostDispatchMsg,
) -> Result<Response, ContractError> {
    let message: Message = req.message.clone().into();
    let hrp = HRP.load(deps.storage)?;

    let (gas_limit, refund_address) = match req.metadata.to_vec().len() < 32 {
        true => (
            Uint256::from(get_default_gas(deps.storage, message.dest_domain)?),
            message.sender_addr(&hrp)?,
        ),
        false => {
            let igp_metadata: IGPMetadata = req.metadata.clone().into();
            (
                igp_metadata.gas_limit,
                igp_metadata.get_refund_address(&hrp, message.sender_addr(&hrp)?),
            )
        }
    };

    Ok(pay_for_gas(
        &deps,
        info,
        message.id(),
        message.dest_domain,
        gas_limit,
        refund_address.to_string(),
    )?
    .add_event(emit_post_dispatch(req.metadata, req.message)))
}

pub fn pay_for_gas(
    deps: &DepsMut,
    info: MessageInfo,
    message_id: HexBinary,
    dest_domain: u32,
    gas_amount: Uint256,
    refund_address: String,
) -> Result<Response, ContractError> {
    let gas_token = GAS_TOKEN.load(deps.storage)?;
    let received = Uint256::from(cw_utils::must_pay(&info, &gas_token)?);
    let gas_needed = quote_gas_price(deps.storage, &deps.querier, dest_domain, gas_amount)?;
    ensure!(
        received >= gas_needed,
        ContractError::InsufficientFunds {
            received,
            gas_needed,
        }
    );

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
        dest_domain,
        message_id,
        gas_amount,
        payment_gap,
        gas_needed,
        received,
    )))
}
