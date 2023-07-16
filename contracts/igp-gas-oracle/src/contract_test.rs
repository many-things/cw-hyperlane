use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Api, CustomQuery, Empty, Env, MessageInfo, OwnedDeps, Querier, Response, Storage,
    Uint128,
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

pub struct IGPGasOracle<S: Storage, A: Api, Q: Querier, C: CustomQuery = Empty> {
    pub deps: OwnedDeps<S, A, Q, C>,
    pub env: Env,
}

impl<S, A, Q, C> IGPGasOracle<S, A, Q, C>
where
    S: Storage,
    A: Api,
    Q: Querier,
    C: CustomQuery,
{
    pub fn new(deps: OwnedDeps<S, A, Q, C>, env: Env) -> Self {
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

#[test]
fn test_gas_data() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");

    let mut oracle = IGPGasOracle::new(mock_dependencies(), mock_env());
    oracle.init(&deployer)?;

    // test single
    let gas_config = RemoteGasDataConfig {
        remote_domain: 1u32,
        token_exchange_rate: Uint128::new(120921),
        gas_price: Uint128::new(9120321),
    };

    oracle.set_remote_gas_data(&deployer, gas_config.clone())?;

    let ret = oracle.get_exchange_rate_and_gas_price(gas_config.remote_domain)?;
    assert_eq!(ret.exchange_rate, gas_config.token_exchange_rate);
    assert_eq!(ret.gas_price, gas_config.gas_price);

    // test multi
    let gas_config = RemoteGasDataConfig {
        remote_domain: 2u32,
        token_exchange_rate: Uint128::new(120921),
        gas_price: Uint128::new(9120321),
    };

    oracle.set_remote_gas_data_configs(&deployer, vec![gas_config.clone()])?;

    let ret = oracle.get_exchange_rate_and_gas_price(gas_config.remote_domain)?;
    assert_eq!(ret.exchange_rate, gas_config.token_exchange_rate);
    assert_eq!(ret.gas_price, gas_config.gas_price);

    Ok(())
}

#[test]
fn test_set_remote_gas_data_configs() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let abuser = Addr::unchecked("abuser");

    let mut oracle = IGPGasOracle::new(mock_dependencies(), mock_env());
    oracle.init(&deployer)?;

    // fail - sender is not owner
    let err = oracle
        .set_remote_gas_data_configs(&abuser, vec![])
        .unwrap_err();
    assert!(matches!(err, ContractError::Unauthorized {}));

    // ok
    oracle.set_remote_gas_data_configs(&deployer, vec![])?;

    Ok(())
}

#[test]
fn test_set_remote_gas_data() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let abuser = Addr::unchecked("abuser");

    let mut oracle = IGPGasOracle::new(mock_dependencies(), mock_env());
    oracle.init(&deployer)?;

    let gas_config = RemoteGasDataConfig {
        remote_domain: 1u32,
        token_exchange_rate: Uint128::new(103202),
        gas_price: Uint128::new(120943),
    };

    // fail - sender is not owner
    let err = oracle
        .set_remote_gas_data(&abuser, gas_config.clone())
        .unwrap_err();
    assert!(matches!(err, ContractError::Unauthorized {}));

    // ok
    oracle.set_remote_gas_data(&deployer, gas_config)?;

    Ok(())
}
