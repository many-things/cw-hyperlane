mod deploy_cw;
mod deploy_evm;

use std::{sync::Arc, time::Duration};

use cosmwasm_std::coin;
use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    utils::{Anvil, AnvilInstance},
};
use osmosis_test_tube::{OsmosisTestApp, SigningAccount};

use deploy_cw::deploy_cw_hyperlane;
use deploy_evm::deploy_evm_hyperlane;

pub use self::deploy_cw::HplCwIsmType;

const BASE: u128 = 1_000_000;

pub struct HplTestEnv<M: Middleware, S: Signer> {
    pub osmo_app: OsmosisTestApp,
    pub osmo_owner: SigningAccount,
    pub eth_app: AnvilInstance,
    pub eth_owner: S,
    pub cw_deployments: deploy_cw::HplCwDeploymentAddrs,
    pub evm_deployments: deploy_evm::HplEvmDeployment<M, S>,
}

pub async fn setup_env(
    evm_domain: u32,
    cw_domain: u32,
    cw_ism_type: HplCwIsmType<'_>,
) -> eyre::Result<HplTestEnv<Provider<Http>, Wallet<SigningKey>>> {
    let osmo_app = OsmosisTestApp::new();
    let eth_app = Anvil::new().spawn();
    let eth_wallet: LocalWallet = eth_app.keys()[0].clone().into();
    let eth_wallet = eth_wallet.with_chain_id(eth_app.chain_id());
    let eth_provider =
        Provider::<Http>::try_from(&eth_app.endpoint())?.interval(Duration::from_millis(10u64));
    let eth_signer = Arc::new(SignerMiddleware::new(eth_provider, eth_wallet.clone()));

    let osmo_owner = osmo_app.init_account(&[coin(1000000u128 * BASE, "uosmo")])?;
    let cw_deployments =
        deploy_cw_hyperlane(&osmo_app, &osmo_owner, cw_domain, cw_ism_type).await?;
    let evm_deployments = deploy_evm_hyperlane(eth_signer, evm_domain).await?;

    let env = HplTestEnv {
        osmo_app,
        osmo_owner,
        eth_app,
        eth_owner: eth_wallet,
        cw_deployments: cw_deployments.addrs,
        evm_deployments,
    };

    Ok(env)
}
