use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use hpl_interface::igp_gas_oracle::RemoteGasDataConfig;

#[cw_serde]
pub struct RemoteGasData {
    pub token_exchange_rate: Uint128,
    pub gas_price: Uint128,
}

pub const OWNER_KEY: &str = "owner";
pub const OWNER: Item<Addr> = Item::new(OWNER_KEY);

pub const PENDING_OWNER_KEY: &str = "pending_owner";
pub const PENDING_OWNER: Item<Addr> = Item::new(PENDING_OWNER_KEY);

pub const REMOTE_GAS_DATA_PREFIX: &str = "remote_gas_data";
pub const REMOTE_GAS_DATA: Map<u32, RemoteGasData> = Map::new(REMOTE_GAS_DATA_PREFIX);

pub fn insert_gas_data(storage: &mut dyn Storage, config: RemoteGasDataConfig) -> StdResult<()> {
    REMOTE_GAS_DATA.save(
        storage,
        config.remote_domain,
        &RemoteGasData {
            token_exchange_rate: config.token_exchange_rate,
            gas_price: config.gas_price,
        },
    )
}
