use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty, Uint256};
use osmosis_test_tube::Wasm;
use test_tube::{Account, Runner, SigningAccount};

use super::types::Codes;

pub enum Hook {
    Mock {
        gas: Uint256,
    },

    Routing {
        owner: String,
        routes: Vec<(u32, Self)>,
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

    fn deploy_routing<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        owner: String,
        mailbox: String,
        routes: Vec<(u32, Self)>,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        use hpl_interface::domain_routing_hook::*;

        let hook = wasm
            .instantiate(
                codes.domain_routing_hook,
                &InstantiateMsg {
                    owner,
                    mailbox: mailbox.clone(),
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
                Ok(HookConfig {
                    destination: domain,
                    hook: Addr::unchecked(hook.deploy(wasm, codes, mailbox.clone(), deployer)?),
                })
            })
            .collect::<eyre::Result<_>>()?;

        wasm.execute(
            &hook,
            &ExecuteMsg::SetHooks { hooks: routes },
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
            Hook::Routing { owner, routes } => {
                Self::deploy_routing(wasm, codes, owner, mailbox, routes, deployer)
            }
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
