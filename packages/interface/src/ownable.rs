use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub enum OwnableMsg {
    InitOwnershipTransfer { next_owner: String },
    RevokeOwnershipTransfer {},
    ClaimOwnership {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum OwnableQueryMsg {
    #[returns(OwnerResponse)]
    GetOwner {},

    #[returns(PendingOwnerResponse)]
    GetPendingOwner {},
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct PendingOwnerResponse {
    pub pending_owner: Option<Addr>,
}
