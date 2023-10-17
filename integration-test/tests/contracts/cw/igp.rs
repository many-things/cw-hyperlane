use hpl_interface::{
    igp::{self, oracle::RemoteGasDataConfig},
    router::{DomainRouteSet, RouterMsg},
};
use ibcx_test_utils::addr;
use test_tube::{Account, Runner, SigningAccount, Wasm};

use super::types::Codes;

#[derive(Clone)]
pub struct Igp {
    pub hrp: String,
    pub owner: String,
    pub gas_token: String,
    pub beneficiary: String,
    pub oracle_configs: Vec<RemoteGasDataConfig>,
}

pub struct IgpDeployment {
    pub core: String,
    pub oracle: String,
}

impl Igp {
    pub fn deploy<'a, R: Runner<'a>>(
        self,
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        mailbox: String,
        deployer: &SigningAccount,
    ) -> eyre::Result<IgpDeployment> {
        let igp = wasm
            .instantiate(
                codes.igp,
                &igp::core::InstantiateMsg {
                    hrp: self.hrp,
                    owner: self.owner,
                    mailbox,
                    gas_token: self.gas_token,
                    beneficiary: self.beneficiary,
                },
                Some(deployer.address().as_str()),
                Some("cw-hpl-igp"),
                &[],
                deployer,
            )?
            .data
            .address;

        let igp_oracle = wasm
            .instantiate(
                codes.igp_oracle,
                &igp::oracle::InstantiateMsg {},
                Some(deployer.address().as_str()),
                Some("cw-hpl-igp-oracle"),
                &[],
                deployer,
            )?
            .data
            .address;

        wasm.execute(
            &igp,
            &igp::core::ExecuteMsg::Router(RouterMsg::SetRoutes {
                set: self
                    .oracle_configs
                    .iter()
                    .map(|v| DomainRouteSet {
                        domain: v.remote_domain,
                        route: Some(addr(&igp_oracle)),
                    })
                    .collect(),
            }),
            &[],
            deployer,
        )?;

        wasm.execute(
            &igp_oracle,
            &igp::oracle::ExecuteMsg::SetRemoteGasDataConfigs {
                configs: self.oracle_configs,
            },
            &[],
            deployer,
        )?;

        Ok(IgpDeployment {
            core: igp,
            oracle: igp_oracle,
        })
    }
}
