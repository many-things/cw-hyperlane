use cosmwasm_std::Empty;
use hpl_interface::ism::multisig::ThresholdSet;
use osmosis_test_tube::Wasm;
use test_tube::{Account, Runner, SigningAccount};

use crate::validator::{self, TestValidators};

use super::types::Codes;

#[derive(Clone)]
pub enum Ism {
    Routing(Vec<(u32, Self)>),

    Multisig {
        hrp: String,
        validators: validator::TestValidators,
    },

    #[allow(dead_code)]
    Mock,
}

impl Ism {
    pub fn routing(isms: Vec<(u32, Self)>) -> Self {
        Self::Routing(isms)
    }

    pub fn multisig(hrp: &str, validators: validator::TestValidators) -> Self {
        Self::Multisig {
            hrp: hrp.to_string(),
            validators,
        }
    }
}

impl Ism {
    fn deploy_mock<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        Ok(wasm
            .instantiate(codes.test_mock_ism, &Empty {}, None, None, &[], deployer)?
            .data
            .address)
    }

    fn deploy_multisig<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        hrp: String,
        set: validator::TestValidators,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let multisig_ism = wasm
            .instantiate(
                codes.ism_multisig,
                &hpl_interface::ism::multisig::InstantiateMsg {
                    owner: deployer.address(),
                    addr_prefix: hrp.to_string(),
                },
                None,
                None,
                &[],
                deployer,
            )?
            .data
            .address;

        wasm.execute(
            &multisig_ism,
            &hpl_interface::ism::multisig::ExecuteMsg::EnrollValidators {
                set: set.to_set(&hrp),
            },
            &[],
            deployer,
        )?;

        wasm.execute(
            &multisig_ism,
            &hpl_interface::ism::multisig::ExecuteMsg::SetThreshold {
                set: ThresholdSet {
                    domain: set.domain,
                    threshold: set.threshold,
                },
            },
            &[],
            deployer,
        )?;

        Ok(multisig_ism)
    }

    fn deploy_routing<'a, R: Runner<'a>>(
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        isms: Vec<(u32, Self)>,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        let routing_ism = wasm
            .instantiate(
                codes.ism_routing,
                &hpl_interface::ism::routing::InstantiateMsg {
                    owner: deployer.address(),
                    isms: isms
                        .into_iter()
                        .map(|(domain, ism)| {
                            Ok(hpl_interface::ism::routing::ISMSet {
                                domain,
                                address: ism.deploy(wasm, codes, deployer)?,
                            })
                        })
                        .collect::<eyre::Result<Vec<_>>>()?,
                },
                None,
                None,
                &[],
                deployer,
            )?
            .data
            .address;

        Ok(routing_ism)
    }

    pub fn deploy<'a, R: Runner<'a>>(
        self,
        wasm: &Wasm<'a, R>,
        codes: &Codes,
        deployer: &SigningAccount,
    ) -> eyre::Result<String> {
        match self {
            Self::Mock => Self::deploy_mock(wasm, codes, deployer),
            Self::Multisig {
                hrp,
                validators: set,
            } => Self::deploy_multisig(wasm, codes, hrp, set, deployer),
            Self::Routing(isms) => Self::deploy_routing(wasm, codes, isms, deployer),
        }
    }
}

pub fn prepare_routing_ism(info: Vec<(u32, &str, TestValidators)>) -> Ism {
    let mut isms = vec![];

    for (domain, hrp, set) in info {
        isms.push((domain, Ism::multisig(hrp, set)));
    }

    Ism::routing(isms)
}
