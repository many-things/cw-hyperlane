pub mod multisig;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, HexBinary};

#[cw_serde]
#[repr(u32)]
pub enum ISMType {
    Unused = 0,
    Routing = 1,
    Aggregation = 2,
    LegacyMultisig = 3,
    Multisig = 4,
    Owned = 5,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum ISMQueryMsg {
    #[returns(ModuleTypeResponse)]
    ModuleType {},

    #[returns(VerifyResponse)]
    Verify {
        metadata: HexBinary,
        message: HexBinary,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum ISMSpecifierQueryMsg {
    #[returns(InterchainSecurityModuleResponse)]
    InterchainSecurityModule(),
}

#[cw_serde]
pub struct ModuleTypeResponse {
    #[serde(rename = "type")]
    pub typ: ISMType,
}

#[cw_serde]
pub struct VerifyResponse {
    pub verified: bool,
}

#[cw_serde]
pub struct InterchainSecurityModuleResponse {
    pub ism: Option<Addr>,
}
