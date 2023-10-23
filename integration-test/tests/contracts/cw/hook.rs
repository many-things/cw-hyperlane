use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty, Uint256};
use hpl_interface::{
    hook::{self, routing_custom::RegisterCustomHookMsg},
    router::{DomainRouteSet, RouterMsg},
};
use ibcx_test_utils::addr;
use osmosis_test_tube::Wasm;
use test_tube::{Account, Runner, SigningAccount};

use super::{igp::Igp, instantiate, types::Codes};

#[allow(dead_code)]
pub enum Hook {
    Mock {
        gas: Uint256,
    },

    Igp(Igp),

    Merkle {},

    Pausable {},

    Routing {
        routes: Vec<(u32, Self)>,
    },

    RoutingCustom {
        routes: Vec<(u32, Self)>,
        custom_hooks: BTreeMap<(u32, Addr), Self>,
    },

    RoutingFallback {
        routes: Vec<(u32, Self)>,
        fallback_hook: Box<Self>,
    },

    Aggregate {
        hooks: Vec<Self>,
    },
}

#[allow(dead_code)]
impl Hook {
    pub fn mock(gas: Uint256) -> Self {
        Self::Mock { gas }
    }

    pub fn routing(routes: Vec<(u32, Self)>) -> Self {
        Self::Routing { routes }
    }
}

impl Hook {
    fn deploy_mock<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        gas: Uint256,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let hook = wasm
            .instantiate(
                codes.test_mock_hook,
                &Empty {},
                Some(deployer.address().as_str()),
                Some("cw-hpl-test-mock-hook"),
                &[],
                deployer,
            )?
            .data
            .address;

        #[cw_serde]
        pub enum TestHookExecuteMsg {
            SetGasAmount { gas: Uint256 },
        }

        wasm.execute(
            &hook,
            &TestHookExecuteMsg::SetGasAmount { gas },
            &[],
            deployer,
        )?;

        Ok(hook)
    }

    fn deploy_merkle<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        mailbox: String,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let hook = wasm
            .instantiate(
                codes.hook_merkle,
                &hook::merkle::InstantiateMsg {
                    owner: owner.address(),
                    mailbox,
                },
                Some(deployer.address().as_str()),
                Some("cw-hpl-hook-merkle"),
                &[],
                deployer,
            )?
            .data
            .address;

        Ok(hook)
    }

    fn deploy_pausable<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let hook = wasm
            .instantiate(
                codes.hook_pausable,
                &hook::pausable::InstantiateMsg {
                    owner: owner.address(),
                    paused: false,
                },
                Some(deployer.address().as_str()),
                Some("cw-hpl-hook-pausable"),
                &[],
                deployer,
            )?
            .data
            .address;

        Ok(hook)
    }

    fn deploy_routing<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        code: u64,
        codes: &Codes,
        mailbox: String,
        routes: Vec<(u32, Self)>,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        use hpl_interface::hook::routing::*;

        let hook = wasm
            .instantiate(
                code,
                &InstantiateMsg {
                    owner: owner.address(),
                },
                Some(deployer.address().as_str()),
                Some("cw-hpl-domain-routing-hook"),
                &[],
                deployer,
            )?
            .data
            .address;

        let routes = routes
            .into_iter()
            .map(|(domain, hook)| {
                let hook_addr = hook.deploy(wasm, codes, mailbox.clone(), owner, deployer)?;
                Ok(DomainRouteSet {
                    domain,
                    route: Some(addr(&hook_addr)),
                })
            })
            .collect::<eyre::Result<_>>()?;

        wasm.execute(
            &hook,
            &ExecuteMsg::Router(RouterMsg::SetRoutes { set: routes }),
            &[],
            owner,
        )?;

        Ok(hook)
    }

    fn deploy_aggregate<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        code: u64,
        codes: &Codes,
        mailbox: String,
        hooks: Vec<Self>,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        use hpl_interface::hook::aggregate::*;

        let hook_addrs = hooks
            .into_iter()
            .map(|hook| hook.deploy(wasm, codes, mailbox.clone(), owner, deployer))
            .collect::<eyre::Result<Vec<_>>>()?;

        let hook = instantiate(
            wasm,
            code,
            deployer,
            "cw-hpl-hook-aggregate",
            &InstantiateMsg {
                owner: owner.address(),
                hooks: hook_addrs,
            },
        );

        Ok(hook.data.address)
    }

    pub fn deploy<'a, R: Runner<'a>>(
        self,
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        mailbox: String,
        owner: &SigningAccount,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        match self {
            Hook::Mock { gas } => Self::deploy_mock(wasm, codes, gas, deployer),
            Hook::Igp(igp) => Ok(igp.deploy(wasm, codes, owner, deployer)?.core),
            Hook::Merkle {} => Self::deploy_merkle(wasm, codes, mailbox, owner, deployer),
            Hook::Pausable {} => Self::deploy_pausable(wasm, codes, owner, deployer),
            Hook::Routing { routes } => Self::deploy_routing(
                wasm,
                codes.hook_routing,
                codes,
                mailbox,
                routes,
                owner,
                deployer,
            ),
            Hook::RoutingCustom {
                routes,
                custom_hooks,
            } => {
                let hook_addr = Self::deploy_routing(
                    wasm,
                    codes.hook_routing_custom,
                    codes,
                    mailbox.clone(),
                    routes,
                    owner,
                    deployer,
                )?;

                let custom_hooks = custom_hooks
                    .into_iter()
                    .map(|(k, v)| {
                        Ok(RegisterCustomHookMsg {
                            dest_domain: k.0,
                            recipient: k.1.to_string(),
                            hook: v.deploy(wasm, codes, mailbox.clone(), owner, deployer)?,
                        })
                    })
                    .collect::<eyre::Result<_>>()?;

                wasm.execute(
                    &hook_addr,
                    &hook::routing_custom::ExecuteMsg::RegisterCustomHooks(custom_hooks),
                    &[],
                    deployer,
                )?;

                Ok(hook_addr)
            }
            Hook::RoutingFallback {
                routes,
                fallback_hook,
            } => {
                let hook_addr = Self::deploy_routing(
                    wasm,
                    codes.hook_routing_fallback,
                    codes,
                    mailbox.clone(),
                    routes,
                    owner,
                    deployer,
                )?;

                let fallback_hook = fallback_hook.deploy(wasm, codes, mailbox, owner, deployer)?;

                wasm.execute(
                    &hook_addr,
                    &hook::routing_fallback::ExecuteMsg::SetFallbackHook {
                        hook: fallback_hook,
                    },
                    &[],
                    deployer,
                )?;

                Ok(hook_addr)
            }
            Hook::Aggregate { hooks } => Self::deploy_aggregate(
                wasm,
                codes.hook_aggregate,
                codes,
                mailbox,
                hooks,
                owner,
                deployer,
            ),
        }
    }
}
