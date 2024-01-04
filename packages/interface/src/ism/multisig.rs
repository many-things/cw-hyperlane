use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::IsmQueryMsg;
#[allow(unused_imports)]
use super::{ModuleTypeResponse, ModulesAndThresholdResponse, VerifyResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub struct ValidatorSet {
    pub domain: u32,
    pub validator: HexBinary,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),

    SimulateVerify {
        metadata: HexBinary,
        message: HexBinary,
    },

    SetValidators {
        domain: u32,
        threshold: u8,
        validators: Vec<HexBinary>, // should be 20 lenghted
    },
    UnsetDomain {
        domain: u32,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),
    Ism(IsmQueryMsg),
    MultisigIsm(MultisigIsmQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum MultisigIsmQueryMsg {
    #[returns(EnrolledValidatorsResponse)]
    EnrolledValidators { domain: u32 },
}

#[cw_serde]
pub struct EnrolledValidatorsResponse {
    pub validators: Vec<HexBinary>,
    pub threshold: u8,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ism::ExpectedIsmMsg, msg_checker};

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
            IsmQueryMsg::ModulesAndThreshold {
                message: HexBinary::default(),
            }
            .wrap(),
        );
    }
}
