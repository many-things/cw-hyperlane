use std::path::PathBuf;

use osmosis_test_tube::Wasm;
use test_tube::{Runner, SigningAccount};

use super::types::{Codes, CodesMap};

const DEFAULT_ARTIFACTS_PATH: &str = "../target/wasm32-unknown-unknown/release/";

const CONTRACTS: [&str; 15] = [
    "default_hook",
    "domain_routing_hook",
    "hub",
    "igp_core",
    "igp_gas_oracle",
    "ism_multisig",
    "ism_routing",
    "mailbox",
    "test_mock_hook",
    "test_mock_ism",
    "test_mock_msg_receiver",
    "multicall",
    "token_cw20",
    "token_native",
    "validator_announce",
];

pub fn store_code<'a, R: Runner<'a>>(
    wasm: &Wasm<'a, R>,
    deployer: &SigningAccount,
    artifacts: Option<impl Into<PathBuf>>,
) -> eyre::Result<Codes> {
    let base_path: PathBuf = artifacts
        .map(|v| v.into())
        .unwrap_or(DEFAULT_ARTIFACTS_PATH.into());

    let artifacts = CONTRACTS
        .into_iter()
        .map(|name| {
            let filename = format!("hpl_{name}.wasm");
            let path = base_path.join(filename);
            let code = std::fs::read(path)?;
            let store_resp = wasm.store_code(&code, None, deployer)?;
            let code_id = store_resp.data.code_id;

            Ok((name.to_string(), code_id))
        })
        .collect::<eyre::Result<CodesMap>>()?;

    artifacts.try_into()
}
