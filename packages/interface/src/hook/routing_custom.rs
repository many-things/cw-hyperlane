use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::{
    ownable::{OwnableMsg, OwnableQueryMsg},
    router::{RouterMsg, RouterQuery},
    Order,
};

use super::{HookQueryMsg, PostDispatchMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub struct RegisterCustomHookMsg {
    pub dest_domain: u32,
    pub recipient: String,
    pub hook: String,
}

#[cw_serde]
pub struct ClearCustomHookMsg {
    pub dest_domain: u32,
    pub recipient: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(OwnableMsg),
    PostDispatch(PostDispatchMsg),
    Router(RouterMsg<Addr>),

    RegisterCustomHook(RegisterCustomHookMsg),
    RegisterCustomHooks(Vec<RegisterCustomHookMsg>),

    ClearCustomHook(ClearCustomHookMsg),
    ClearCustomHooks(Vec<ClearCustomHookMsg>),
}

#[cw_serde]
#[derive(QueryResponses)]
#[query_responses(nested)]
pub enum QueryMsg {
    Ownable(OwnableQueryMsg),
    Router(RouterQuery<Addr>),
    Hook(HookQueryMsg),
    CustomRoutingHook(CustomRoutingHookQueryMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum CustomRoutingHookQueryMsg {
    #[returns(CustomHookResponse)]
    CustomHook { dest_domain: u32, recipient: String },

    #[returns(CustomHooksResponse)]
    CustomHooks {
        dest_domain: u32,
        offset: Option<String>,
        limit: Option<u32>,
        order: Option<Order>,
    },
}

#[cw_serde]
pub struct CustomHookResponse {
    pub dest_domain: u32,
    pub recipient: String,
    pub hook: String,
}

#[cw_serde]
pub struct CustomHooksResponse {
    pub custom_hooks: Vec<CustomHookResponse>,
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
