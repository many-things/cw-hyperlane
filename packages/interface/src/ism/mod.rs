pub mod multisig;
pub mod routing;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, HexBinary};

#[cw_serde]
#[repr(u32)]
pub enum ISMType {
    Unused = 0,
    Routing = 1,
    Aggregation = 2,
    LegacyMultisig = 3,
    MerkleRootMultisig = 4,
    MessageIdMultisig = 5,
    Null = 6, // used with relayer carrying no metadata
    CcipRead = 7,
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

    #[returns(VerifyInfoResponse)]
    VerifyInfo { message: HexBinary },
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
pub struct VerifyInfoResponse {
    pub threshold: u8,
    pub validators: Vec<String>,
}

#[cw_serde]
pub struct InterchainSecurityModuleResponse {
    pub ism: Option<Addr>,
}
