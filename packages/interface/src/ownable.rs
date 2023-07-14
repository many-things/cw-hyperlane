use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum OwnableMsg {
    InitOwnershipTransfer { next_owner: String },
    RevokeOwnershipTransfer {},
    ClaimOwnership {},
}
