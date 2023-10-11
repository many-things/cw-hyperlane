use std::marker::PhantomData;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Empty;

use crate::Order;

#[cw_serde]
pub struct DomainRouterSet<T> {
    pub domain: u32,
    pub router: T,
}

#[cw_serde]
pub enum RouterMsg<T> {
    EnrollRemoteRouter { set: DomainRouterSet<T> },
    EnrollRemoteRouters { set: Vec<DomainRouterSet<T>> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum RouterQuery<T> {
    #[returns(DomainsResponse)]
    Domains {},

    #[returns(RouteResponse<T>)]
    Route { domain: u32 },

    #[returns(RoutesResponse<T>)]
    Routes {
        offset: Option<u32>,
        limit: Option<u32>,
        order: Option<Order>,
    },

    #[serde(skip)]
    #[returns(Empty)]
    Placeholder(PhantomData<T>),
}

#[cw_serde]
pub struct DomainsResponse {
    pub domains: Vec<u32>,
}

#[cw_serde]
pub struct RouteResponse<T> {
    pub route: DomainRouterSet<T>,
}

#[cw_serde]
pub struct RoutesResponse<T> {
    pub routes: Vec<DomainRouterSet<T>>,
}
