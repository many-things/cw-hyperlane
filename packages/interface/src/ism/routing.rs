use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::ownable::OwnableMsg;

#[allow(unused_imports)]
use super::{ModuleTypeResponse, VerifyResponse};

#[cw_serde]
pub struct ISMSet {
    pub domain: u32,
    pub address: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub isms: Vec<ISMSet>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownership(OwnableMsg),

    Set { ism: ISMSet },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ModuleTypeResponse)]
    ModuleType {},

    #[returns(VerifyResponse)]
    Verify {
        metadata: HexBinary,
        message: HexBinary,
    },

    #[returns(RouteResponse)]
    Route { message: HexBinary },
}

#[cw_serde]
pub struct RouteResponse {
    pub ism: String,
}

#[cw_serde]
pub struct MigrateMsg {}
