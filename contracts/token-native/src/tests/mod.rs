use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Binary, Coin, Empty, Env, MessageInfo, OwnedDeps, Response,
};
use hpl_interface::{
    mailbox, router,
    token::TokenMode,
    token_native::{ExecuteMsg, QueryMsg},
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
    msg::{InstantiateMsg, Metadata},
    state::{MAILBOX, MODE, OWNER, TOKEN},
};

mod contracts;

pub struct TokenNative {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl Default for TokenNative {
    fn default() -> Self {
        Self {
            deps: mock_dependencies(),
            env: mock_env(),
        }
    }
}

impl TokenNative {
    pub fn init(
        &mut self,
        sender: &Addr,
        owner: &Addr,
        mailbox: &Addr,
        denom: &str,
        metadata: Option<Metadata>,
        mode: TokenMode,
    ) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                denom: denom.to_string(),
                metadata,
                mode,
                owner: owner.to_string(),
                mailbox: mailbox.to_string(),
            },
        )
    }

    pub fn init_hack(
        &mut self,
        _sender: &Addr,
        owner: &Addr,
        mailbox: &Addr,
        denom: &str,
        mode: TokenMode,
    ) -> anyhow::Result<()> {
        MODE.save(&mut self.deps.storage, &mode)?;
        TOKEN.save(&mut self.deps.storage, &denom.to_string())?;
        OWNER.save(&mut self.deps.storage, owner)?;
        MAILBOX.save(&mut self.deps.storage, mailbox)?;

        Ok(())
    }

    fn execute(&mut self, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    fn query<T: DeserializeOwned>(&self, msg: QueryMsg) -> Result<T, ContractError> {
        query(self.deps.as_ref(), self.env.clone(), msg)
            .map(|v| from_binary::<T>(&v))?
            .map_err(|e| e.into())
    }

    pub fn router_enroll(
        &mut self,
        sender: &Addr,
        domain: u32,
        router: Binary,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::Router(router::RouterMsg::EnrollRemoteRouter {
                set: router::RouterSet { domain, router },
            }),
        )
    }

    pub fn mailbox_handle(
        &mut self,
        sender: &Addr,
        handle_msg: mailbox::HandleMsg,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::Handle(handle_msg),
        )
    }

    pub fn transfer_remote(
        &mut self,
        sender: &Addr,
        fund: Coin,
        dest_domain: u32,
        recipient: Binary,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[fund]),
            ExecuteMsg::TransferRemote {
                dest_domain,
                recipient,
            },
        )
    }
}
