use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct HookConfig {
    pub destination: u32,
    pub hook: Addr,
}

#[cw_serde]
pub struct PauseInfoResponse {
    pub paused: bool,
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: String,
}
