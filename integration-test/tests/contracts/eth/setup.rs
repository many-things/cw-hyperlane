use std::{sync::Arc, time::Duration};

use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::U64,
    utils::{Anvil, AnvilInstance},
};

use super::{deploy, types::Deployments};

pub struct Env<M: Middleware, S: Signer> {
    pub app: AnvilInstance,
    pub core: Deployments<M, S>,
    pub domain: u32,

    pub signer: Arc<SignerMiddleware<M, S>>,
    pub acc_owner: S,
}

pub async fn setup_env(domain: u32) -> eyre::Result<Env<Provider<Http>, Wallet<SigningKey>>> {
    let app = Anvil::new().block_time(1u64).spawn();

    let wallet = LocalWallet::from(app.keys()[0].clone());
    let wallet = wallet.with_chain_id(app.chain_id());

    let provider =
        Provider::<Http>::try_from(&app.endpoint())?.interval(Duration::from_millis(10u64));
    let signer = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

    loop {
        let block_number = signer.get_block_number().await?;

        if block_number > U64::from(0u64) {
            break;
        }
    }

    let core = deploy(signer.clone(), domain).await?;

    Ok(Env {
        app,
        core,
        domain,

        signer,
        acc_owner: wallet,
    })
}
