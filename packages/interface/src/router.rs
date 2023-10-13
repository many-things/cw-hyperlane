use std::marker::PhantomData;

use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::Order;

#[cw_serde]
pub struct DomainRouteSet<T> {
    pub domain: u32,
    pub route: Option<T>,
}

#[cw_serde]
pub enum RouterMsg<T> {
    SetRoute { set: DomainRouteSet<T> },
    SetRoutes { set: Vec<DomainRouteSet<T>> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum RouterQuery<T> {
    #[returns(DomainsResponse)]
    Domains {},

    #[returns(RouteResponse<T>)]
    GetRoute { domain: u32 },

    #[returns(RoutesResponse<T>)]
    ListRoutes {
        offset: Option<u32>,
        limit: Option<u32>,
        order: Option<Order>,
    },

    #[serde(skip)]
    #[returns(cosmwasm_std::Empty)]
    Placeholder(PhantomData<T>),
}

#[cw_serde]
pub struct DomainsResponse {
    pub domains: Vec<u32>,
}

#[cw_serde]
pub struct RouteResponse<T> {
    pub route: DomainRouteSet<T>,
}

#[cw_serde]
pub struct RoutesResponse<T> {
    pub routes: Vec<DomainRouteSet<T>>,
}
