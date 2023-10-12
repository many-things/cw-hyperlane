#[cfg(test)]
mod test;

use cosmwasm_std::{
    to_binary, CustomQuery, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response,
    StdError, StdResult, Storage,
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

fn get_route_map<T>() -> Map<'static, u32, T>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
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
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    use RouterMsg::*;

    match msg {
        SetRoute { set } => {
            set_route(deps.storage, set.clone())?;

            let resp = Response::new().add_event(
                Event::new("hpl_router::set_route")
                    .add_attribute("sender", info.sender)
                    .add_attribute(
                        "set",
                        serde_json_wasm::to_string(&set)
                            .map_err(|_| StdError::generic_err("encoding failed"))?,
                    ),
            );

            Ok(resp)
        }
        SetRoutes { set } => {
            set_routes(deps.storage, set.clone())?;

            let resp = Response::new().add_event(
                Event::new("hpl_router::set_routes")
                    .add_attribute("sender", info.sender)
                    .add_attribute(
                        "set",
                        serde_json_wasm::to_string(&set)
                            .map_err(|_| StdError::generic_err("encoding failed"))?,
                    ),
            );

            Ok(resp)
        }
    }
}

pub fn set_route<T>(storage: &mut dyn Storage, set: DomainRouteSet<T>) -> StdResult<()>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    get_route_map().save(storage, set.domain, &set.route)?;

    Ok(())
}

pub fn set_routes<T>(storage: &mut dyn Storage, set: Vec<DomainRouteSet<T>>) -> StdResult<()>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    for DomainRouteSet {
        domain,
        route: router,
    } in set
    {
        get_route_map().save(storage, domain, &router)?;
    }

    Ok(())
}

pub fn handle_query<C: CustomQuery, T>(
    deps: Deps<'_, C>,
    _env: Env,
    msg: RouterQuery<T>,
) -> StdResult<QueryResponse>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
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
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    Ok(router == get_route_map().load(storage, domain)?)
}

pub fn get_domains<T>(storage: &dyn Storage) -> StdResult<Vec<u32>>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    get_route_map::<T>()
        .keys(storage, None, None, Order::Asc.into())
        .collect()
}

pub fn get_route<T>(storage: &dyn Storage, domain: u32) -> StdResult<DomainRouteSet<T>>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    Ok(DomainRouteSet {
        domain,
        route: get_route_map().load(storage, domain).unwrap_or_default(),
    })
}

pub fn get_routes<T>(
    storage: &dyn Storage,
    offset: Option<u32>,
    limit: Option<u32>,
    order: Option<Order>,
) -> StdResult<Vec<DomainRouteSet<T>>>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
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
