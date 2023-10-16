pub mod multisig;
pub mod routing;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CustomQuery, HexBinary, QuerierWrapper, StdResult};

#[cw_serde]
#[repr(u32)]
pub enum IsmType {
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
pub enum IsmQueryMsg {
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

impl IsmQueryMsg {
    pub fn wrap(self) -> ExpectedIsmQueryMsg {
        ExpectedIsmQueryMsg::Ism(self)
    }
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum ExpectedIsmQueryMsg {
    Ism(IsmQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum IsmSpecifierQueryMsg {
    #[returns(InterchainSecurityModuleResponse)]
    InterchainSecurityModule(),
}

#[cw_serde]
pub struct ModuleTypeResponse {
    #[serde(rename = "type")]
    pub typ: IsmType,
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

pub fn recipient<C: CustomQuery>(
    querier: &QuerierWrapper<C>,
    recipient: impl Into<String>,
) -> StdResult<Option<Addr>> {
    let res = querier.query_wasm_smart::<InterchainSecurityModuleResponse>(
        recipient,
        &IsmSpecifierQueryMsg::InterchainSecurityModule(),
    )?;

    Ok(res.ism)
}

pub fn verify<C: CustomQuery>(
    querier: &QuerierWrapper<C>,
    ism: impl Into<String>,
    metadata: HexBinary,
    message: HexBinary,
) -> StdResult<bool> {
    let verify_resp = querier.query_wasm_smart::<VerifyResponse>(
        ism,
        &IsmQueryMsg::Verify { metadata, message }.wrap(),
    )?;

    Ok(verify_resp.verified)
}
