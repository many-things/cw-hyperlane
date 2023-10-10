use ethers::{prelude::SignerMiddleware, providers::Middleware, signers::Signer};

use super::{
    mailbox::Mailbox, test_mock_ism::TestMultisigIsm, test_mock_msg_receiver::TestRecipient,
};

pub struct Deployments<M: Middleware, S: Signer> {
    pub mailbox: Mailbox<SignerMiddleware<M, S>>,

    pub ism: TestMultisigIsm<SignerMiddleware<M, S>>,
    pub msg_receiver: TestRecipient<SignerMiddleware<M, S>>,
}
