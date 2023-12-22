use ethers::{prelude::SignerMiddleware, providers::Middleware, signers::Signer};

pub use super::{
    mailbox::Mailbox, test_mock_hook::TestHook, test_mock_ism::TestIsm,
    test_mock_merkle_tree_hook::TestMerkleTreeHook, test_mock_recipient::TestRecipient,
};

pub struct Deployments<M: Middleware, S: Signer> {
    pub mailbox: Mailbox<SignerMiddleware<M, S>>,

    pub default_hook: TestHook<SignerMiddleware<M, S>>,
    pub required_hook: TestMerkleTreeHook<SignerMiddleware<M, S>>,
    pub ism: TestIsm<SignerMiddleware<M, S>>,
    pub recipient: TestRecipient<SignerMiddleware<M, S>>,
}
