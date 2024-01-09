use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::IsmQueryMsg;
#[allow(unused_imports)]
use super::{ModuleTypeResponse, VerifyResponse};

#[cw_serde]
pub struct IsmSet {
    pub domain: u32,
    pub address: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub isms: Vec<IsmSet>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),

    SimulateVerify {
        metadata: HexBinary,
        message: HexBinary,
    },

    Set {
        ism: IsmSet,
    },
    Unset {
        domains: Vec<u32>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),
    Ism(IsmQueryMsg),
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
    use crate::{
        ism::{ExpectedIsmMsg, IsmQueryMsg},
        msg_checker,
    };

    #[test]
    fn test_ism_interface() {
        let _checked: ExecuteMsg = msg_checker(ExpectedIsmMsg::SimulateVerify {
            metadata: HexBinary::default(),
            message: HexBinary::default(),
        });

        let _checked: QueryMsg = msg_checker(IsmQueryMsg::ModuleType {}.wrap());
        let _checked: QueryMsg = msg_checker(
            IsmQueryMsg::Verify {
                metadata: HexBinary::default(),
                message: HexBinary::default(),
            }
            .wrap(),
        );
        let _checked: QueryMsg = msg_checker(
            IsmQueryMsg::VerifyInfo {
                message: HexBinary::default(),
            }
            .wrap(),
        );
    }
}
