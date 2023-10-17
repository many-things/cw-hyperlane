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

use super::{igp::Igp, types::Codes};

pub enum Hook {
    Mock {
        gas: Uint256,
    },

    Igp(Igp),

    Merkle {
        owner: String,
    },

    Pausable {
        owner: String,
    },

    Routing {
        owner: String,
        routes: Vec<(u32, Self)>,
    },

    RoutingCustom {
        owner: String,
        routes: Vec<(u32, Self)>,
        custom_hooks: BTreeMap<(u32, Addr), Self>,
    },

    RoutingFallback {
        owner: String,
        routes: Vec<(u32, Self)>,
        fallback_hook: Box<Self>,
    },

    Aggregate {
        hooks: Vec<Self>,
    },
}

impl Hook {
    pub fn mock(gas: Uint256) -> Self {
        Self::Mock { gas }
    }

    pub fn routing(owner: String, routes: Vec<(u32, Self)>) -> Self {
        Self::Routing { owner, routes }
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
        owner: String,
        mailbox: String,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let hook = wasm
            .instantiate(
                codes.hook_merkle,
                &hook::merkle::InstantiateMsg { owner, mailbox },
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
        owner: String,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let hook = wasm
            .instantiate(
                codes.hook_pausable,
                &hook::pausable::InstantiateMsg {
                    owner,
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
        owner: String,
        mailbox: String,
        routes: Vec<(u32, Self)>,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        use hpl_interface::hook::routing::*;

        let hook = wasm
            .instantiate(
                code,
                &InstantiateMsg { owner },
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
                let hook_addr = hook.deploy(wasm, codes, mailbox.clone(), deployer)?;
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
            deployer,
        )?;

        Ok(hook)
    }

    pub fn deploy<'a, R: Runner<'a>>(
        self,
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        mailbox: String,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        match self {
            Hook::Mock { gas } => Self::deploy_mock(wasm, codes, gas, deployer),
            Hook::Igp(igp) => Ok(igp.deploy(wasm, codes, mailbox, deployer)?.core),
            Hook::Merkle { owner } => Self::deploy_merkle(wasm, codes, owner, mailbox, deployer),
            Hook::Pausable { owner } => Self::deploy_pausable(wasm, codes, owner, deployer),
            Hook::Routing { owner, routes } => Self::deploy_routing(
                wasm,
                codes.hook_routing,
                codes,
                owner,
                mailbox,
                routes,
                deployer,
            ),
            Hook::RoutingCustom {
                owner,
                routes,
                custom_hooks,
            } => {
                let hook_addr = Self::deploy_routing(
                    wasm,
                    codes.hook_routing_custom,
                    codes,
                    owner,
                    mailbox.clone(),
                    routes,
                    deployer,
                )?;

                let custom_hooks = custom_hooks
                    .into_iter()
                    .map(|(k, v)| {
                        Ok(RegisterCustomHookMsg {
                            dest_domain: k.0,
                            recipient: k.1.to_string(),
                            hook: v.deploy(wasm, codes, mailbox.clone(), deployer)?,
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
                owner,
                routes,
                fallback_hook,
            } => {
                let hook_addr = Self::deploy_routing(
                    wasm,
                    codes.hook_routing_fallback,
                    codes,
                    owner,
                    mailbox.clone(),
                    routes,
                    deployer,
                )?;

                let fallback_hook = fallback_hook.deploy(wasm, codes, mailbox, deployer)?;

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
            Hook::Aggregate { hooks } => todo!(),
        }
    }
}

pub fn prepare_routing_hook(owner: String, routes: Vec<(u32, u128)>) -> Hook {
    let routes = routes
        .into_iter()
        .map(|(domain, gas)| (domain, Hook::mock(gas.into())))
        .collect();

    Hook::routing(owner, routes)
}
