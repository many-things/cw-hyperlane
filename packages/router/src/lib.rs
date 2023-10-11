use cosmwasm_std::{
    to_binary, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response, StdError,
    StdResult, Storage,
};
use cw_storage_plus::Map;
use hpl_interface::{
    range_option,
    router::{
        DomainRouterSet, DomainsResponse, RouteResponse, RouterMsg, RouterQuery, RoutesResponse,
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

pub fn handle<T>(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RouterMsg<T>,
) -> StdResult<Response>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    use RouterMsg::*;

    match msg {
        EnrollRemoteRouter { set } => {
            enroll_remote_router(deps.storage, set.clone())?;

            let resp = Response::new().add_event(
                Event::new("hpl_router::enroll_remote_router")
                    .add_attribute("sender", info.sender)
                    .add_attribute(
                        "set",
                        serde_json_wasm::to_string(&set)
                            .map_err(|_| StdError::generic_err("encoding failed"))?,
                    ),
            );

            Ok(resp)
        }
        EnrollRemoteRouters { set } => {
            enroll_remote_routers(deps.storage, set.clone())?;

            let resp = Response::new().add_event(
                Event::new("hpl_router::enroll_remote_routers")
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

pub fn enroll_remote_router<T>(storage: &mut dyn Storage, set: DomainRouterSet<T>) -> StdResult<()>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    get_route_map().save(storage, set.domain, &set.router)?;

    Ok(())
}

pub fn enroll_remote_routers<T>(
    storage: &mut dyn Storage,
    set: Vec<DomainRouterSet<T>>,
) -> StdResult<()>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    for DomainRouterSet { domain, router } in set {
        get_route_map().save(storage, domain, &router)?;
    }

    Ok(())
}

pub fn handle_query<T>(deps: Deps, _env: Env, msg: RouterQuery<T>) -> StdResult<QueryResponse>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    match msg {
        RouterQuery::Domains {} => to_binary(&DomainsResponse {
            domains: get_domains::<T>(deps.storage)?,
        }),
        RouterQuery::Route { domain } => to_binary(&RouteResponse::<T> {
            route: get_route(deps.storage, domain)?,
        }),
        RouterQuery::Routes {
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

pub fn get_route<T>(storage: &dyn Storage, domain: u32) -> StdResult<DomainRouterSet<T>>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    Ok(DomainRouterSet {
        domain,
        router: get_route_map().load(storage, domain).unwrap_or_default(),
    })
}

pub fn get_routes<T>(
    storage: &dyn Storage,
    offset: Option<u32>,
    limit: Option<u32>,
    order: Option<Order>,
) -> StdResult<Vec<DomainRouterSet<T>>>
where
    T: Serialize + DeserializeOwned + Clone + Eq + Default,
{
    let ((min, max), limit, order) = range_option(offset, limit, order)?;

    get_route_map()
        .range(storage, min, max, order.into())
        .take(limit)
        .map(|item| {
            let (domain, router) = item?;
            Ok(DomainRouterSet { domain, router })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, Binary, Empty, Env, MessageInfo, OwnedDeps, Response, StdResult,
    };
    use hpl_interface::{
        router::{
            DomainRouterSet, DomainsResponse, RouteResponse, RouterMsg, RouterQuery, RoutesResponse,
        },
        Order,
    };
    use serde::{de::DeserializeOwned, Serialize};

    use crate::{handle, handle_query, is_router};

    pub struct Router<T>
    where
        T: Serialize + DeserializeOwned + Clone + Eq + Default,
    {
        pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        pub env: Env,

        _marker: PhantomData<T>,
    }

    impl<T> Router<T>
    where
        T: Serialize + DeserializeOwned + Clone + Eq + Default,
    {
        fn handle(&mut self, info: MessageInfo, msg: RouterMsg<T>) -> StdResult<Response> {
            handle(self.deps.as_mut(), self.env.clone(), info, msg)
        }

        pub fn enroll_one(&mut self, sender: &Addr, domain: u32, router: T) -> StdResult<Response> {
            self.handle(
                mock_info(sender.as_str(), &[]),
                RouterMsg::EnrollRemoteRouter {
                    set: DomainRouterSet { domain, router },
                },
            )
        }

        pub fn enroll_many(&mut self, sender: &Addr, set: &[(u32, T)]) -> StdResult<Response> {
            self.handle(
                mock_info(sender.as_str(), &[]),
                RouterMsg::EnrollRemoteRouters {
                    set: set
                        .iter()
                        .map(|v| DomainRouterSet {
                            domain: v.0,
                            router: v.1.clone(),
                        })
                        .collect(),
                },
            )
        }

        fn handle_query<R: DeserializeOwned>(&self, msg: RouterQuery<T>) -> StdResult<R> {
            from_binary(&handle_query(self.deps.as_ref(), self.env.clone(), msg)?)
        }

        pub fn query_domains(&self) -> StdResult<DomainsResponse> {
            self.handle_query(RouterQuery::Domains {})
        }

        pub fn query_route(&self, domain: u32) -> StdResult<RouteResponse<T>> {
            self.handle_query(RouterQuery::Route { domain })
        }

        pub fn query_routes(
            &self,
            offset: Option<u32>,
            limit: Option<u32>,
            order: Option<Order>,
        ) -> StdResult<RoutesResponse<T>> {
            self.handle_query(RouterQuery::Routes {
                offset,
                limit,
                order,
            })
        }
    }

    impl<T> Default for Router<T>
    where
        T: Serialize + DeserializeOwned + Clone + Eq + Default,
    {
        fn default() -> Self {
            Self {
                deps: mock_dependencies(),
                env: mock_env(),
                _marker: PhantomData::<T>::default(),
            }
        }
    }

    #[test]
    fn test_handle() -> anyhow::Result<()> {
        let owner = Addr::unchecked("owner");

        let set_a = DomainRouterSet {
            domain: 1,
            router: Binary(b"router_a".to_vec()),
        };

        let set_b = DomainRouterSet {
            domain: 2,
            router: Binary(b"router_b".to_vec()),
        };

        let domain_no = 99999;

        let mut router = Router::default();

        router.enroll_one(&owner, set_a.domain, set_a.router.clone())?;
        router.enroll_many(&owner, &[(set_b.domain, set_b.router.clone())])?;

        let DomainsResponse { domains } = router.query_domains()?;
        assert_eq!(domains, vec![1, 2]);

        let RouteResponse { route: route_a } = router.query_route(set_a.domain)?;
        assert_eq!(route_a, set_a.router);

        let RouteResponse { route: route_b } = router.query_route(set_b.domain)?;
        assert_eq!(route_b, set_b.router);

        let RouteResponse { route: route_no } = router.query_route(domain_no)?;
        assert_eq!(route_no, Binary::default());

        Ok(())
    }

    #[test]
    fn test_check() -> anyhow::Result<()> {
        let owner = Addr::unchecked("owner");

        let mut router = Router::default();

        let domain_t = 1;
        let router_t = Binary(b"test".to_vec());
        let router_n = Binary(b"no".to_vec());

        router.enroll_one(&owner, domain_t, router_t.clone())?;

        assert!(is_router(&router.deps.storage, domain_t, router_t)?);
        assert!(!is_router(&router.deps.storage, domain_t, router_n)?);

        Ok(())
    }
}
