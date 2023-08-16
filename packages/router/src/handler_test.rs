use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Binary, Empty, Env, MessageInfo, OwnedDeps, Response, StdResult,
};
use hpl_interface::router::{DomainsResponse, RouterMsg, RouterQuery, RouterResponse, RouterSet};
use serde::de::DeserializeOwned;

use crate::{handle, handle_query, is_router};

pub struct Router {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl Router {
    fn handle(&mut self, info: MessageInfo, msg: RouterMsg) -> StdResult<Response> {
        handle(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    pub fn enroll_one(
        &mut self,
        sender: &Addr,
        domain: u32,
        router: Binary,
    ) -> StdResult<Response> {
        self.handle(
            mock_info(sender.as_str(), &[]),
            RouterMsg::EnrollRemoteRouter {
                set: RouterSet { domain, router },
            },
        )
    }

    pub fn enroll_many(&mut self, sender: &Addr, set: &[(u32, Binary)]) -> StdResult<Response> {
        self.handle(
            mock_info(sender.as_str(), &[]),
            RouterMsg::EnrollRemoteRouters {
                set: set
                    .iter()
                    .map(|v| RouterSet {
                        domain: v.0,
                        router: v.1.clone(),
                    })
                    .collect(),
            },
        )
    }

    fn handle_query<T: DeserializeOwned>(&self, msg: RouterQuery) -> StdResult<T> {
        from_binary(&handle_query(self.deps.as_ref(), self.env.clone(), msg)?)
    }

    pub fn query_domains(&self) -> StdResult<DomainsResponse> {
        self.handle_query(RouterQuery::Domains {})
    }

    pub fn query_router(&self, domain: u32) -> StdResult<RouterResponse> {
        self.handle_query(RouterQuery::Router { domain })
    }
}

impl Default for Router {
    fn default() -> Self {
        Self {
            deps: mock_dependencies(),
            env: mock_env(),
        }
    }
}

#[test]
fn test_handle() -> anyhow::Result<()> {
    let owner = Addr::unchecked("owner");

    let set_a = RouterSet {
        domain: 1,
        router: Binary(b"router_a".to_vec()),
    };

    let set_b = RouterSet {
        domain: 2,
        router: Binary(b"router_b".to_vec()),
    };

    let domain_no = 99999;

    let mut router = Router::default();

    router.enroll_one(&owner, set_a.domain, set_a.router.clone())?;
    router.enroll_many(&owner, &[(set_b.domain, set_b.router.clone())])?;

    let DomainsResponse { domains } = router.query_domains()?;
    assert_eq!(domains, vec![1, 2]);

    let RouterResponse { router: router_a } = router.query_router(set_a.domain)?;
    assert_eq!(router_a, set_a.router);

    let RouterResponse { router: router_b } = router.query_router(set_b.domain)?;
    assert_eq!(router_b, set_b.router);

    let RouterResponse { router: router_no } = router.query_router(domain_no)?;
    assert_eq!(router_no, Binary::default());

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
