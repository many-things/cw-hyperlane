#[cfg(test)]
mod test;

use cosmwasm_std::{
    ensure_eq, to_binary, Addr, CustomQuery, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse,
    Response, StdError, StdResult, Storage,
};
use cw_storage_plus::Map;
use hpl_interface::{
    range_option,
    router::{
        DomainRouteSet, DomainsResponse, RouteResponse, RouterMsg, RouterQuery, RoutesResponse,
    },
    Order,
};
use serde::{de::DeserializeOwned, Serialize};

const ROUTES_PREFIX: &str = "routes";

fn event_to_resp(event: Event) -> Response {
    Response::new().add_event(event)
}

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_router::{}", name))
}

fn get_route_map<T>() -> Map<'static, u32, T>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    Map::new(ROUTES_PREFIX)
}

pub fn handle<C: CustomQuery, T>(
    deps: DepsMut<'_, C>,
    _env: Env,
    info: MessageInfo,
    msg: RouterMsg<T>,
) -> StdResult<Response>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    use RouterMsg::*;

    ensure_eq!(
        hpl_ownable::get_owner(deps.storage)?,
        info.sender,
        StdError::generic_err("unauthorized")
    );

    match msg {
        SetRoute { set } => {
            let event = set_route(deps.storage, &info.sender, set)?;

            Ok(event_to_resp(event))
        }
        SetRoutes { set } => {
            let event = set_routes(deps.storage, &info.sender, set)?;

            Ok(event_to_resp(event))
        }
    }
}

pub fn set_route<T>(
    storage: &mut dyn Storage,
    sender: &Addr,
    set: DomainRouteSet<T>,
) -> StdResult<Event>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    get_route_map().save(storage, set.domain, &set.route)?;

    Ok(new_event("set_route")
        .add_attribute("sender", sender)
        .add_attribute(
            "set",
            serde_json_wasm::to_string(&set)
                .map_err(|_| StdError::generic_err("encoding failed"))?,
        ))
}

pub fn set_routes<T>(
    storage: &mut dyn Storage,
    sender: &Addr,
    set: Vec<DomainRouteSet<T>>,
) -> StdResult<Event>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    for DomainRouteSet {
        domain,
        route: router,
    } in set.clone()
    {
        get_route_map().save(storage, domain, &router)?;
    }

    Ok(new_event("set_routes")
        .add_attribute("sender", sender)
        .add_attribute(
            "set",
            serde_json_wasm::to_string(&set)
                .map_err(|_| StdError::generic_err("encoding failed"))?,
        ))
}

pub fn handle_query<C: CustomQuery, T>(
    deps: Deps<'_, C>,
    _env: Env,
    msg: RouterQuery<T>,
) -> StdResult<QueryResponse>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    match msg {
        RouterQuery::Domains {} => to_binary(&DomainsResponse {
            domains: get_domains::<T>(deps.storage)?,
        }),
        RouterQuery::GetRoute { domain } => to_binary(&RouteResponse::<T> {
            route: get_route(deps.storage, domain)?,
        }),
        RouterQuery::ListRoutes {
            offset,
            limit,
            order,
        } => to_binary(&RoutesResponse::<T> {
            routes: get_routes(deps.storage, offset, limit, order)?,
        }),
        RouterQuery::Placeholder(_) => unreachable!(),
    }
}

pub fn is_router<T>(storage: &dyn Storage, domain: u32, router: T) -> StdResult<bool>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    Ok(router == get_route_map().load(storage, domain)?)
}

pub fn get_domains<T>(storage: &dyn Storage) -> StdResult<Vec<u32>>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    get_route_map::<T>()
        .keys(storage, None, None, Order::Asc.into())
        .collect()
}

pub fn get_route<T>(storage: &dyn Storage, domain: u32) -> StdResult<DomainRouteSet<T>>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    Ok(DomainRouteSet {
        domain,
        route: get_route_map().load(storage, domain).ok(),
    })
}

pub fn get_routes<T>(
    storage: &dyn Storage,
    offset: Option<u32>,
    limit: Option<u32>,
    order: Option<Order>,
) -> StdResult<Vec<DomainRouteSet<T>>>
where
    T: Serialize + DeserializeOwned + Clone + Eq,
{
    let ((min, max), limit, order) = range_option(offset, limit, order)?;

    get_route_map()
        .range(storage, min, max, order.into())
        .take(limit)
        .map(|item| {
            let (domain, router) = item?;
            Ok(DomainRouteSet {
                domain,
                route: router,
            })
        })
        .collect()
}
