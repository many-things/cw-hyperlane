use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    to_binary, Addr, Binary, Empty, Env, MessageInfo, OwnedDeps, Response, Uint128,
};
use hpl_interface::{
    mailbox, router,
    token::TokenMode,
    token_cw20::{ExecuteMsg, QueryMsg, ReceiveMsg},
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
    msg::{InstantiateMsg, TokenOption},
};

mod contracts;

pub struct TokenCW20 {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl Default for TokenCW20 {
    fn default() -> Self {
        Self {
            deps: mock_dependencies(),
            env: mock_env(),
        }
    }
}

impl TokenCW20 {
    pub fn init(
        &mut self,
        sender: &Addr,
        owner: &Addr,
        mailbox: &Addr,
        token: Option<TokenOption>,
        mode: TokenMode,
    ) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                token,
                mode,
                owner: owner.to_string(),
                mailbox: mailbox.to_string(),
            },
        )
    }

    fn execute(&mut self, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    #[allow(dead_code)]
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
        token: &Addr,
        amount: Uint128,
        dest_domain: u32,
        recipient: Binary,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(token.as_str(), &[]),
            ExecuteMsg::Receive(cw20::Cw20ReceiveMsg {
                sender: sender.to_string(),
                amount,
                msg: to_binary(&ReceiveMsg::TransferRemote {
                    dest_domain,
                    recipient,
                })?,
            }),
        )
    }
}
