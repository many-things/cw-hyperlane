use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use super::{HookQueryMsg, PostDispatchMsg};

pub const TREE_DEPTH: usize = 32;

#[cw_serde]
pub struct InstantiateMsg {
    pub mailbox: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    PostDispatch(PostDispatchMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Hook(HookQueryMsg),
    MerkleHook(MerkleHookQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum MerkleHookQueryMsg {
    #[returns(CountResponse)]
    Count {},

    #[returns(RootResponse)]
    Root {},

    #[returns(BranchResponse)]
    Branch {},

    #[returns(TreeResponse)]
    Tree {},

    #[returns(CheckPointResponse)]
    CheckPoint {},
}

#[cw_serde]
pub struct CountResponse {
    pub count: u32,
}

#[cw_serde]
pub struct RootResponse {
    pub root: HexBinary,
}

#[cw_serde]
pub struct BranchResponse {
    pub branch: [HexBinary; TREE_DEPTH],
}

#[cw_serde]
pub struct TreeResponse {
    pub branch: [HexBinary; TREE_DEPTH],
    pub count: u32,
}

#[cw_serde]
pub struct CheckPointResponse {
    pub root: HexBinary,
    pub count: u32,
}

#[cfg(test)]
mod test {
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
