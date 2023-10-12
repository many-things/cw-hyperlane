use crate::event::{emit_claim, emit_pay_for_gas, emit_post_dispatch, emit_set_beneficiary};
use crate::query::quote_gas_price;
use crate::state::{BENEFICIARY, CONFIG, GAS_TOKEN};
use crate::ContractError;
use crate::DEFAULT_GAS_USAGE;

use cosmwasm_std::{
    coins, ensure, ensure_eq, BankMsg, DepsMut, Env, HexBinary, MessageInfo, Response, Uint128,
    Uint256,
};
use cw_utils::PaymentError;
use hpl_interface::{
    hook::PostDispatchMsg,
    types::{IGPMetadata, Message},
};

use std::str::FromStr;

pub fn set_beneficiary(
    deps: DepsMut,
    info: MessageInfo,
    beneficiary: String,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        hpl_ownable::get_owner(deps.storage)?,
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
    let igp_metadata: IGPMetadata = req.metadata.clone().into();
    let message: Message = req.message.clone().into();
    let prefix = CONFIG.load(deps.storage)?.prefix;

    let gas_limit = match req.metadata.to_vec().len() < 32 {
        true => Uint256::from(DEFAULT_GAS_USAGE),
        false => igp_metadata.gas_limit,
    };
    let refund_address =
        igp_metadata.get_refund_address(prefix.clone(), message.sender_addr(prefix.as_str())?);

    Ok(pay_for_gas(
        deps,
        info,
        message.id(),
        message.dest_domain,
        gas_limit,
        refund_address.to_string(),
    )?
    .add_event(emit_post_dispatch(req.metadata, req.message)))
}

pub fn pay_for_gas(
    deps: DepsMut,
    info: MessageInfo,
    message_id: HexBinary,
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
