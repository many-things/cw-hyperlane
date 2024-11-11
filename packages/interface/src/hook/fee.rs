use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::{HookQueryMsg, PostDispatchMsg};

pub const TREE_DEPTH: usize = 32;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub fee: Coin,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    PostDispatch(PostDispatchMsg),
    FeeHook(FeeHookMsg),
}

#[cw_serde]
pub enum FeeHookMsg {
    SetFee { fee: Coin },
    Claim { recipient: Option<Addr> },
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),
    Hook(HookQueryMsg),
    FeeHook(FeeHookQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum FeeHookQueryMsg {
    #[returns(FeeResponse)]
    Fee {},
}

#[cw_serde]
pub struct FeeResponse {
    pub fee: Coin,
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
