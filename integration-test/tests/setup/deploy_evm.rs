use std::sync::Arc;

use ethers::{prelude::SignerMiddleware, providers::Middleware, signers::Signer};
use hpl_tests::{ism_multisig, mailbox};

#[derive(Debug)]
pub struct HplEvmDeployment<M: Middleware, S: Signer> {
    pub ism: ism_multisig::TestMultisigIsm<SignerMiddleware<M, S>>,
    pub mailbox: mailbox::Mailbox<SignerMiddleware<M, S>>,
}

pub async fn deploy_evm_hyperlane<'a, M: Middleware + 'static, S: Signer + 'static>(
    signer: Arc<SignerMiddleware<M, S>>,
) -> eyre::Result<HplEvmDeployment<M, S>> {
    let ism_multisig_contract = ism_multisig::TestMultisigIsm::deploy(signer.clone(), ())?
        .send()
        .await?;

    let mailbox_contract = mailbox::Mailbox::deploy(signer.clone(), 1u64)?
        .send()
        .await?;

    let _ = mailbox_contract
        .initialize(signer.address(), ism_multisig_contract.address())
        .send()
        .await?
        .await?;

    let deployments = HplEvmDeployment {
        ism: ism_multisig_contract,
        mailbox: mailbox_contract,
    };

    Ok(deployments)
}
