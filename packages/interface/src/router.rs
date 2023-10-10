use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

#[cw_serde]
pub struct RouterSet {
    pub domain: u32,
    pub router: Binary,
}

#[cw_serde]
pub enum RouterMsg {
    EnrollRemoteRouter { set: RouterSet },
    EnrollRemoteRouters { set: Vec<RouterSet> },
}

// router should also implement ism::ISMSpecifierQueryMsg::InterchainSecurityModule(),
#[cw_serde]
#[derive(QueryResponses)]
pub enum RouterQuery {
    #[returns(DomainsResponse)]
    Domains {},
    #[returns(RouterResponse)]
    Router { domain: u32 },
}

#[cw_serde]
pub struct DomainsResponse {
    pub domains: Vec<u32>,
}

#[cw_serde]
pub struct RouterResponse {
    pub router: Binary,
}
