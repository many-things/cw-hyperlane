use cosmwasm_std::{
    from_binary,
    testing::{mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Binary, Coin, Deps, DepsMut, Empty, Env, MessageInfo, OwnedDeps, Response,
};
use hpl_interface::igp_core::{
    ExecuteMsg, GasOracleConfig, GetExchangeRateAndGasPriceResponse, InstantiateMsg, QueryMsg,
    QuoteGasPaymentResponse,
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
};

mod contract;

pub struct IGP {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl IGP {
    pub fn new(deps: OwnedDeps<MockStorage, MockApi, MockQuerier>, env: Env) -> Self {
        Self { deps, env }
    }

    pub fn with_env(&mut self, env: Env) {
        self.env = env
    }

    pub fn init(
        &mut self,
        sender: &Addr,
        owner: &Addr,
        gas_token: &str,
        beneficiary: &Addr,
    ) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                owner: owner.to_string(),
                gas_token: gas_token.to_string(),
                beneficiary: beneficiary.to_string(),
            },
        )
    }

    fn execute(&mut self, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    fn query<T: DeserializeOwned>(&self, msg: QueryMsg) -> Result<T, ContractError> {
        query(self.deps.as_ref(), self.env.clone(), msg)
            .map(|v| from_binary::<T>(&v))?
            .map_err(|e| e.into())
    }

    pub fn deps_mut(&mut self) -> DepsMut {
        self.deps.as_mut()
    }

    pub fn deps_ref(&self) -> Deps {
        self.deps.as_ref()
    }

    pub fn set_gas_oracles(
        &mut self,
        sender: &Addr,
        configs: Vec<GasOracleConfig>,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::SetGasOracles { configs },
        )
    }

    pub fn set_beneficiary(
        &mut self,
        sender: &Addr,
        beneficiary: &Addr,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::SetBeneficiary {
                beneficiary: beneficiary.to_string(),
            },
        )
    }

    pub fn claim(&mut self, sender: &Addr) -> Result<Response, ContractError> {
        self.execute(mock_info(sender.as_str(), &[]), ExecuteMsg::Claim {})
    }

    pub fn pay_for_gas(
        &mut self,
        sender: &Addr,
        funds: &[Coin],
        message_id: &Binary,
        dest_domain: u32,
        gas_amount: u128,
        refund_address: &Addr,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), funds),
            ExecuteMsg::PayForGas {
                message_id: message_id.clone(),
                dest_domain,
                gas_amount: gas_amount.into(),
                refund_address: refund_address.to_string(),
            },
        )
    }

    pub fn get_quote_gas_payment(
        &self,
        dest_domain: u32,
        gas_amount: u128,
    ) -> Result<QuoteGasPaymentResponse, ContractError> {
        self.query(QueryMsg::QuoteGasPayment {
            dest_domain,
            gas_amount: gas_amount.into(),
        })
    }

    pub fn get_exchange_rate_and_gas_price(
        &self,
        dest_domain: u32,
    ) -> Result<GetExchangeRateAndGasPriceResponse, ContractError> {
        self.query(QueryMsg::GetExchangeRateAndGasPrice { dest_domain })
    }
}
