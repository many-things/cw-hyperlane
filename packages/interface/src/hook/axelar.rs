use cosmwasm_schema::{cw_serde, QueryResponses};
use serde::{Deserialize, Serialize};
use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::{HookQueryMsg, PostDispatchMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub mailbox: String,
    pub destination_chain: String,
    pub destination_contract: String,
    pub destination_ism: String,
    pub axelar_gateway_channel: String,
}

#[cw_serde]
pub struct RegisterDestinationContractMsg {
    pub destination_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    PostDispatch(PostDispatchMsg),
    RegisterDestinationContract(RegisterDestinationContractMsg)
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Axelar(AxelarQueryMsg),
    Ownable(OwnableQueryMsg),
    Hook(HookQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum AxelarQueryMsg {
    #[returns(AxelarInfoResponse)]
    PauseInfo {},
}

#[cw_serde]
pub struct AxelarInfoResponse {
    pub destination_chain: String,
    pub destination_contract: String,
    pub destination_ism: String,
    pub axelar_gateway_channel: String,
}


#[derive(
Clone,
Debug,
PartialEq,
Eq,
serde::Serialize,
serde::Deserialize,
schemars::JsonSchema,
)]
pub struct AxelarFee {
    amount: String,
    recipient: String
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AxelarGeneralMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    #[serde(rename = "type")]
    pub type_: i64,
    pub fee: Option<AxelarFee>
}





#[cfg(test)]
mod test {
    use cosmwasm_std::HexBinary;

    use super::*;
    use crate::{
        hook::{ExpectedHookQueryMsg, PostDispatchMsg, QuoteDispatchMsg},
        msg_checker,
    };

    #[test]
    fn test_hook_interface() {
        let _checked: ExecuteMsg = msg_checker(
            PostDispatchMsg {
                metadata: HexBinary::default(),
                message: HexBinary::default(),
            }
            .wrap(),
        );

        let _checked: QueryMsg = msg_checker(ExpectedHookQueryMsg::Hook(HookQueryMsg::Mailbox {}));
        let _checked: QueryMsg = msg_checker(
            QuoteDispatchMsg {
                metadata: HexBinary::default(),
                message: HexBinary::default(),
            }
            .request(),
        );
    }
}
