use std::error::Error;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, QueryResponse, StdError, StdResult};
use cw_storage_plus::Bound;

pub mod connection;
pub mod core;
pub mod hook;
pub mod igp;
pub mod ism;
mod macros;
pub mod ownable;
pub mod pausable;
pub mod router;
pub mod types;
pub mod warp;

pub use macros::*;

#[cw_serde]
pub enum Order {
    Asc,
    Desc,
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

impl From<Order> for cosmwasm_std::Order {
    fn from(v: Order) -> Self {
        match v {
            Order::Asc => cosmwasm_std::Order::Ascending,
            Order::Desc => cosmwasm_std::Order::Descending,
        }
    }
}

// Settings for pagination
pub const MAX_LIMIT: u32 = 30;
pub const DEFAULT_LIMIT: u32 = 10;

pub fn get_and_check_limit(limit: Option<u32>, max: u32, default: u32) -> StdResult<u32> {
    match limit {
        Some(l) => {
            if l <= max {
                Ok(l)
            } else {
                Err(StdError::generic_err(format!(
                    "oversized request. size: {:?}, max: {:?}",
                    l as u64, max as u64,
                )))
            }
        }
        None => Ok(default),
    }
}

type RangeOptionRespBound<'a, T> = Option<Bound<'a, T>>;
type RangeOptionResp<'a, T> = (
    (RangeOptionRespBound<'a, T>, RangeOptionRespBound<'a, T>),
    usize,
    Order,
);

pub fn range_option<'a, T: cw_storage_plus::PrimaryKey<'a>>(
    start: Option<T>,
    limit: Option<u32>,
    order: Option<Order>,
) -> StdResult<RangeOptionResp<'a, T>> {
    let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
    let order = order.unwrap_or(Order::Asc);
    let (min, max) = match order {
        Order::Asc => (start.map(Bound::exclusive), None),
        Order::Desc => (None, start.map(Bound::exclusive)),
    };

    Ok(((min, max), limit, order))
}

pub fn to_binary<T: serde::Serialize, E: Error, F: From<E> + From<StdError>>(
    data: Result<T, E>,
) -> Result<QueryResponse, F> {
    data.map(|v| to_json_binary(&v))?.map_err(|err| err.into())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{from_json, to_json_binary};
    use serde::{de::DeserializeOwned, Serialize};

    pub fn msg_checker<Input: Serialize, Output: DeserializeOwned>(input: Input) -> Output {
        from_json::<Output>(to_json_binary(&input).unwrap()).unwrap()
    }
}

#[cfg(test)]
pub use test::msg_checker;
