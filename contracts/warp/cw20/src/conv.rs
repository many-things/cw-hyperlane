use std::str::FromStr;

use cosmwasm_std::{wasm_execute, StdResult, Uint128, Uint256, WasmMsg};
use cw20::Cw20ExecuteMsg;

use crate::error::ContractError;

pub fn to_mint_msg(
    contract: impl Into<String>,
    recipient: impl Into<String>,
    amount: Uint256,
) -> Result<WasmMsg, ContractError> {
    Ok(wasm_execute(
        contract,
        &Cw20ExecuteMsg::Mint {
            recipient: recipient.into(),
            amount: to_uint128(amount)?,
        },
        vec![],
    )?)
}

pub fn to_burn_msg(contract: impl Into<String>, amount: Uint128) -> StdResult<WasmMsg> {
    wasm_execute(contract, &Cw20ExecuteMsg::Burn { amount }, vec![])
}

pub fn to_send_msg(
    contract: impl Into<String>,
    recipient: impl Into<String>,
    amount: Uint256,
) -> Result<WasmMsg, ContractError> {
    Ok(wasm_execute(
        contract,
        &Cw20ExecuteMsg::Transfer {
            recipient: recipient.into(),
            amount: to_uint128(amount)?,
        },
        vec![],
    )?)
}

pub fn to_uint128(v: Uint256) -> Result<Uint128, ContractError> {
    Ok(Uint128::from_str(&v.to_string())?)
}
