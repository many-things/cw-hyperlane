use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::HexBinary;

use crate::ownable::{OwnableMsg, OwnableQueryMsg};

use super::{HookQueryMsg, PostDispatchMsg};

pub const TREE_DEPTH: usize = 32;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
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
    Ownable(OwnableQueryMsg),
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
