use std::marker::PhantomData;

use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Binary, Empty, Env, MessageInfo, OwnedDeps, Response, StdResult,
};
use hpl_interface::{
    router::{
        DomainRouteSet, DomainsResponse, RouteResponse, RouterMsg, RouterQuery, RoutesResponse,
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

    pub fn set_route(&mut self, sender: &Addr, domain: u32, router: T) -> StdResult<Response> {
        self.handle(
            mock_info(sender.as_str(), &[]),
            RouterMsg::SetRoute {
                set: DomainRouteSet {
                    domain,
                    route: Some(router),
                },
            },
        )
    }

    pub fn set_routes(&mut self, sender: &Addr, set: &[(u32, T)]) -> StdResult<Response> {
        self.handle(
            mock_info(sender.as_str(), &[]),
            RouterMsg::SetRoutes {
                set: set
                    .iter()
                    .map(|v| DomainRouteSet {
                        domain: v.0,
                        route: Some(v.1.clone()),
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
        self.handle_query(RouterQuery::GetRoute { domain })
    }

    pub fn query_routes(
        &self,
        offset: Option<u32>,
        limit: Option<u32>,
        order: Option<Order>,
    ) -> StdResult<RoutesResponse<T>> {
        self.handle_query(RouterQuery::ListRoutes {
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

    let set_a = DomainRouteSet {
        domain: 1,
        route: Some(Binary(b"router_a".to_vec())),
    };

    let set_b = DomainRouteSet {
        domain: 2,
        route: Some(Binary(b"router_b".to_vec())),
    };

    let domain_no = 99999;

    let mut router = Router::default();

    hpl_ownable::initialize(router.deps.as_mut().storage, &owner)?;

    router.set_route(&owner, set_a.domain, set_a.route.clone().unwrap())?;
    router.set_routes(&owner, &[(set_b.domain, set_b.route.clone().unwrap())])?;

    let DomainsResponse { domains } = router.query_domains()?;
    assert_eq!(domains, vec![1, 2]);

    let RouteResponse { route: route_a } = router.query_route(set_a.domain)?;
    assert_eq!(route_a.route, set_a.route);

    let RouteResponse { route: route_b } = router.query_route(set_b.domain)?;
    assert_eq!(route_b.route, set_b.route);

    let RouteResponse { route: route_no } = router.query_route(domain_no)?;
    assert_eq!(route_no.route, None);

    let RoutesResponse { routes } = router.query_routes(None, None, None)?;
    assert_eq!(routes, vec![set_a, set_b]);

    Ok(())
}

#[test]
fn test_check() -> anyhow::Result<()> {
    let owner = Addr::unchecked("owner");

    let mut router = Router::default();

    hpl_ownable::initialize(router.deps.as_mut().storage, &owner)?;

    let domain_t = 1;
    let router_t = Binary(b"test".to_vec());
    let router_n = Binary(b"no".to_vec());

    router.set_route(&owner, domain_t, router_t.clone())?;

    assert!(is_router(&router.deps.storage, domain_t, router_t)?);
    assert!(!is_router(&router.deps.storage, domain_t, router_n)?);

    Ok(())
}
