use std::{collections::BTreeMap, fs, path::Path};

use cosmwasm_std::coin;
use ethers::{
    providers::{Middleware, Provider},
    utils::Anvil,
};
use osmosis_test_tube::{Account, Module, OsmosisTestApp, SigningAccount};

pub const BASE: u128 = 1000000;

async fn deploy_evm_hyperlane<'a>() -> eyre::Result<BTreeMap<&'a str, u64>> {
    // TODO: fill deploy logic
    Ok(BTreeMap::new())
}

async fn deploy_cw_hyperlane<'a>(
    app: &OsmosisTestApp,
    deployer: &SigningAccount,
) -> eyre::Result<BTreeMap<&'a str, u64>> {
    let wasm = osmosis_test_tube::Wasm::new(app);

    // store codes
    let base_path = Path::new("../target/wasm32-unknown-unknown/release/");
    let artifacts = [
        ("ism_multisig", "hpl_ism_multisig.wasm"),
        ("mailbox", "hpl_mailbox.wasm"),
        ("mailbox_factory", "hpl_mailbox_factory.wasm"),
        ("multicall", "hpl_multicall.wasm"),
    ]
    .into_iter()
    .map(|(name, filename)| {
        let path = base_path.join(filename);
        let code = fs::read(path).unwrap();
        let store_resp = wasm.store_code(&code, None, deployer).unwrap();
        let code_id = store_resp.data.code_id;

        (name, code_id)
    })
    .collect::<BTreeMap<_, _>>();

    Ok(artifacts)
}

async fn setup_node() -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let eth_app = Anvil::new().spawn();
    let eth_provider = Provider::try_from(&eth_app.endpoint())?;

    let owner = osmo_app.init_account(&[coin(1000000u128 * BASE, "uosmo")])?;

    let cw_deployments = deploy_cw_hyperlane(&osmo_app, &owner).await?;
    println!("cw_deployments: {:?}", cw_deployments);

    let evm_deployments = deploy_evm_hyperlane().await?;
    println!("evm_deployments: {:?}", evm_deployments);

    Ok(())
}

#[tokio::test]
async fn test_setup() {
    setup_node().await.unwrap();
}
