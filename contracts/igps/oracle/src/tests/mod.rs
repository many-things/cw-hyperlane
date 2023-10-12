use cosmwasm_std::{
    from_binary, testing::mock_info, Addr, Api, Empty, Env, MessageInfo, OwnedDeps, Querier,
    Response, Storage,
};
use hpl_interface::igp_gas_oracle::{
    ConfigResponse, ExecuteMsg, GetExchangeRateAndGasPriceResponse, InstantiateMsg, QueryMsg,
    RemoteGasDataConfig,
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
};

mod contract;

pub struct IGPGasOracle<S: Storage, A: Api, Q: Querier> {
    pub deps: OwnedDeps<S, A, Q, Empty>,
    pub env: Env,
}

impl<S, A, Q> IGPGasOracle<S, A, Q>
where
    S: Storage,
    A: Api,
    Q: Querier,
{
    pub fn new(deps: OwnedDeps<S, A, Q>, env: Env) -> Self {
        Self { deps, env }
    }

    pub fn with_env(&mut self, env: Env) {
        self.env = env
    }

    pub fn init(&mut self, sender: &Addr) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {},
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

    pub fn set_remote_gas_data_configs(
        &mut self,
        sender: &Addr,
        configs: Vec<RemoteGasDataConfig>,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::SetRemoteGasDataConfigs { configs },
        )
    }

    pub fn set_remote_gas_data(
        &mut self,
        sender: &Addr,
        config: RemoteGasDataConfig,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::SetRemoteGasData { config },
        )
    }

    pub fn get_config(&self) -> Result<ConfigResponse, ContractError> {
        self.query(QueryMsg::Config {})
    }

    pub fn get_exchange_rate_and_gas_price(
        &self,
        dest_domain: u32,
    ) -> Result<GetExchangeRateAndGasPriceResponse, ContractError> {
        self.query(QueryMsg::GetExchangeRateAndGasPrice { dest_domain })
    }
}
