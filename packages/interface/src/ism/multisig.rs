use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::IsmQueryMsg;
#[allow(unused_imports)]
use super::{ModuleTypeResponse, VerifyInfoResponse, VerifyResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub hrp: String,
}

#[cw_serde]
pub struct ValidatorSet {
    pub domain: u32,
    pub validator: String,
    pub validator_pubkey: HexBinary,
}

#[cw_serde]
pub struct ThresholdSet {
    pub domain: u32,
    pub threshold: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),

    EnrollValidator { set: ValidatorSet },
    EnrollValidators { set: Vec<ValidatorSet> },
    UnenrollValidator { domain: u32, validator: String },

    SetThreshold { set: ThresholdSet },
    SetThresholds { set: Vec<ThresholdSet> },
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
    pub validators: Vec<String>,
    pub threshold: u8,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::msg_checker;

    #[test]
    fn test_ism_interface() {
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
