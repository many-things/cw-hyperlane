use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::{
    ownable::{OwnableMsg, OwnableQueryMsg},
    pausable::{PausableMsg, PausableQueryMsg},
};

use super::{HookQueryMsg, PostDispatchMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub paused: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    Pausable(PausableMsg),
    PostDispatch(PostDispatchMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Pausable(PausableQueryMsg),
    Ownable(OwnableQueryMsg),
    Hook(HookQueryMsg),
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
