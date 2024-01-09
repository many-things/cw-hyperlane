use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::IsmQueryMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub isms: Vec<String>,
    pub threshold: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),

    SimulateVerify {
        metadata: HexBinary,
        message: HexBinary,
    },

    SetIsms {
        isms: Vec<String>,
        threshold: u8,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),

    Ism(IsmQueryMsg),

    AggregateIsm(AggregateIsmQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum AggregateIsmQueryMsg {
    #[returns(IsmsResponse)]
    Isms {},
}

#[cw_serde]
pub struct IsmsResponse {
    pub isms: Vec<String>,
    pub threshold: u8,
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
