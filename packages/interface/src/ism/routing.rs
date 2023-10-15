use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::ISMQueryMsg;
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
    Ownable(OwnableMsg),

    Set { ism: ISMSet },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),
    ISM(ISMQueryMsg),
    RoutingIsm(RoutingIsmQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum RoutingIsmQueryMsg {
    #[returns(RouteResponse)]
    Route { message: HexBinary },
}

#[cw_serde]
pub struct RouteResponse {
    pub ism: String,
}

#[cfg(test)]
mod test {
    use cosmwasm_std::HexBinary;

    use super::*;
    use crate::{ism::ISMQueryMsg, msg_checker};

    #[test]
    fn test_ism_interface() {
        let _checked: QueryMsg = msg_checker(ISMQueryMsg::ModuleType {});
        let _checked: QueryMsg = msg_checker(ISMQueryMsg::Verify {
            metadata: HexBinary::default(),
            message: HexBinary::default(),
        });
        let _checked: QueryMsg = msg_checker(ISMQueryMsg::VerifyInfo {
            message: HexBinary::default(),
        });
    }
}
