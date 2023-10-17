use std::str::FromStr;

use cosmwasm_std::{coin, Addr, BankMsg, Coin, Uint128, Uint256};
use hpl_interface::warp::native;

use crate::{error::ContractError, proto};

pub fn to_mint_msg(sender: &Addr, denom: &str, amount: impl ToString) -> proto::MsgMint {
    proto::MsgMint {
        sender: sender.to_string(),
        amount: Some(proto::Coin {
            denom: denom.to_string(),
            amount: amount.to_string(),
        }),
    }
}

pub fn to_burn_msg(sender: &Addr, denom: &str, amount: impl ToString) -> proto::MsgBurn {
    proto::MsgBurn {
        sender: sender.to_string(),
        amount: Some(proto::Coin {
            denom: denom.to_string(),
            amount: amount.to_string(),
        }),
    }
}

pub fn to_send_msg(recipient: &Addr, amount: Vec<Coin>) -> BankMsg {
    BankMsg::Send {
        to_address: recipient.to_string(),
        amount,
    }
}

pub fn to_coin(amount: impl Into<u128>, denom: impl Into<String>) -> Coin {
    coin(amount.into(), denom)
}

pub fn to_coin_u256(amount: Uint256, denom: impl Into<String>) -> Result<Coin, ContractError> {
    Ok(to_coin(to_uint128(amount)?, denom))
}

pub fn to_uint128(v: Uint256) -> Result<Uint128, ContractError> {
    Ok(Uint128::from_str(&v.to_string())?)
}

pub fn to_set_metadata_msg(sender: &Addr, data: native::Metadata) -> proto::MsgSetDenomMetadata {
    proto::MsgSetDenomMetadata {
        sender: sender.to_string(),
        metadata: Some(proto::Metadata {
            description: data.description,
            denom_units: data
                .denom_units
                .into_iter()
                .map(|v| proto::DenomUnit {
                    denom: v.denom,
                    exponent: v.exponent,
                    aliases: v.aliases,
                })
                .collect(),
            base: data.base,
            display: data.display,
            name: data.name,
            symbol: data.symbol,
        }),
    }
}
