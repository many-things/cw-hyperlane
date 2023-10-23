use cosmwasm_schema::{cw_serde, QueryResponses};

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

    SetIsms { isms: Vec<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),

    Ism(IsmQueryMsg),
}

#[cfg(test)]
mod test {
    use cosmwasm_std::HexBinary;

    use super::*;
    use crate::{ism::IsmQueryMsg, msg_checker};

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
