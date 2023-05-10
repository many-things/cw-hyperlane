use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, HexBinary};

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(VerifyResponse)]
    Verify {
        metadata: HexBinary,
        message: HexBinary,
    },

    #[returns(InterchainSecurityModuleResponse)]
    InterchainSecurityModule(),
}

#[cw_serde]
pub struct VerifyResponse(pub bool);

#[cw_serde]
pub struct InterchainSecurityModuleResponse(pub Option<Addr>);
