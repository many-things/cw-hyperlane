use std::sync::Arc;

use ethers::{prelude::SignerMiddleware, providers::Middleware, signers::Signer};

use super::types::{Deployments, Mailbox, TestHook, TestIsm, TestMerkleTreeHook, TestRecipient};

pub async fn deploy<'a, M: Middleware + 'static, S: Signer + 'static>(
    signer: Arc<SignerMiddleware<M, S>>,
    evm_domain: u32,
) -> eyre::Result<Deployments<M, S>> {
    println!("{}", signer.get_block_number().await?);

    let ism = TestIsm::deploy(signer.clone(), ())?.send().await?;

    let recipient = TestRecipient::deploy(signer.clone(), ())?.send().await?;

    let mailbox = Mailbox::deploy(signer.clone(), evm_domain)?.send().await?;

    let default_hook = TestHook::deploy(signer.clone(), ())?.send().await?;

    let _ = mailbox
        .initialize(
            signer.address(),
            ism.address(),
            default_hook.address(),
            default_hook.address(),
        )
        .send()
        .await?
        .await?;

    let required_hook = TestMerkleTreeHook::deploy(signer.clone(), mailbox.address())?
        .send()
        .await?;

    let _ = mailbox
        .set_required_hook(required_hook.address())
        .send()
        .await?;

    let deployments = Deployments {
        mailbox,
        ism,
        default_hook,
        required_hook,
        recipient,
    };

    Ok(deployments)
}
