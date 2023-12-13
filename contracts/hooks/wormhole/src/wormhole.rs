use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub enum WormholeExecuteMsg {
    PostMessage { message: Binary, nonce: u32 },
}