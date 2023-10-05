use std::sync::Arc;

use ethers::{prelude::SignerMiddleware, providers::Middleware, signers::Signer};

use crate::contracts::{mailbox, test_mock_ism, test_mock_msg_receiver};

#[derive(Debug)]
pub struct HplEvmDeployment<M: Middleware, S: Signer> {
    pub mailbox: mailbox::Mailbox<SignerMiddleware<M, S>>,

    pub ism: test_mock_ism::TestMultisigIsm<SignerMiddleware<M, S>>,
    pub msg_receiver: test_mock_msg_receiver::TestRecipient<SignerMiddleware<M, S>>,
}

pub async fn deploy_evm_hyperlane<'a, M: Middleware + 'static, S: Signer + 'static>(
    signer: Arc<SignerMiddleware<M, S>>,
    evm_domain: u32,
) -> eyre::Result<HplEvmDeployment<M, S>> {
    let ism_multisig_contract = test_mock_ism::TestMultisigIsm::deploy(signer.clone(), ())?
        .send()
        .await?;

    let msg_receiver_contract = test_mock_msg_receiver::TestRecipient::deploy(signer.clone(), ())?
        .send()
        .await?;

    let mailbox_contract = mailbox::Mailbox::deploy(signer.clone(), evm_domain)?
        .send()
        .await?;

    let _ = mailbox_contract
        .initialize(signer.address(), ism_multisig_contract.address())
        .send()
        .await?
        .await?;

    let deployments = HplEvmDeployment {
        mailbox: mailbox_contract,
        ism: ism_multisig_contract,
        msg_receiver: msg_receiver_contract,
    };

    Ok(deployments)
}
