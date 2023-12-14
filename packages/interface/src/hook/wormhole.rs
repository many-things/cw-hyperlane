use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::{HookQueryMsg, PostDispatchMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub wormhole_core: String,
    pub mailbox: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    PostDispatch(PostDispatchMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Wormhole(WormholeQueryMsg),
    Ownable(OwnableQueryMsg),
    Hook(HookQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum WormholeQueryMsg {
    #[returns(WormholeInfoResponse)]
    PauseInfo {},
}

#[cw_serde]
pub struct WormholeInfoResponse {
    pub wormhole_core: String,
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
