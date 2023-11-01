use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{ensure, HexBinary, StdError, StdResult};

use crate::{
    ownable::{OwnableMsg, OwnableQueryMsg},
    Order,
};

use super::IsmQueryMsg;
#[allow(unused_imports)]
use super::{ModuleTypeResponse, VerifyInfoResponse, VerifyResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub struct ValidatorSet {
    pub domain: u32,
    pub validators: Vec<HexBinary>,
    pub threshold: u8,
}

impl ValidatorSet {
    fn validation_err(&self, reason: &str) -> StdError {
        StdError::generic_err(format!(
            "invalid validator set. domain: {}. {reason}",
            self.domain
        ))
    }

    fn is_duplicated(slice: Vec<HexBinary>) -> bool {
        (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
    }

    pub fn validate(&self) -> StdResult<()> {
        ensure!(
            !Self::is_duplicated(self.validators.clone()),
            self.validation_err("validator duplicated")
        );

        for v in self.validators.iter() {
            ensure!(
                v.len() == 20,
                self.validation_err("validator address length should be 20")
            )
        }

        ensure!(
            self.validators.len() <= 255,
            self.validation_err("validator count should be less than 255")
        );

        ensure!(
            self.validators.len() >= self.threshold as usize,
            self.validation_err("threshold should be less than validator count")
        );

        ensure!(
            self.threshold != 0,
            self.validation_err("threshold should be greater than 0")
        );

        Ok(())
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),

    UpdateValidatorSet { set: ValidatorSet },
    UnsetDomain { domain: u32 },
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

    #[returns(ListEnrolledValidatorsResponse)]
    ListEnrolledValidators {
        offset: Option<u32>,
        limit: Option<u32>,
        order: Option<Order>,
    },
}

#[cw_serde]
pub struct EnrolledValidatorsResponse {
    pub validators: Vec<HexBinary>,
    pub threshold: u8,
}

#[cw_serde]
pub struct ListEnrolledValidatorsResponse {
    pub validators: Vec<(u32, EnrolledValidatorsResponse)>,
}

#[cfg(test)]
mod test {
    use ibcx_test_utils::{gen_bz, hex};
    use rstest::rstest;

    use super::*;
    use crate::msg_checker;

    fn make_case(validators: &[HexBinary], threshold: u8) -> ValidatorSet {
        ValidatorSet {
            domain: 1,
            validators: validators.to_vec(),
            threshold,
        }
    }

    #[rstest]
    #[case(make_case(&[gen_bz(20), gen_bz(20)], 2))]
    #[should_panic(
        expected = "Generic error: invalid validator set. domain: 1. validator duplicated"
    )]
    #[case(make_case(&[hex(&"deadbeef".repeat(5)), hex(&"deadbeef".repeat(5))], 2))]
    #[should_panic(
        expected = "Generic error: invalid validator set. domain: 1. validator address length should be 20"
    )]
    #[case(make_case(&[gen_bz(21)], 1))]
    #[should_panic(
        expected = "Generic error: invalid validator set. domain: 1. threshold should be less than validator count"
    )]
    #[case(make_case(&[], 1))]
    #[should_panic(
        expected = "Generic error: invalid validator set. domain: 1. threshold should be less than validator count"
    )]
    #[case(make_case(&[gen_bz(20), gen_bz(20)], 3))]
    fn test_validation(#[case] validator: ValidatorSet) {
        validator.validate().map_err(|v| v.to_string()).unwrap()
    }

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
