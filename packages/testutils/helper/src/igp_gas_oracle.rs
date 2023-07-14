use cosmwasm_std::Addr;
use cw_multi_test::{App, AppResponse, Executor};
use hpl_interface::{
    igp_gas_oracle::{
        ConfigResponse, ExecuteMsg::*, GetExchangeRateAndGasPriceResponse, QueryMsg,
        RemoteGasDataConfig,
    },
    ownable::OwnableMsg,
};

pub struct IGPGasOracle<'a> {
    pub app: &'a mut App,
    pub address: Addr,
}

impl<'a> IGPGasOracle<'a> {
    pub fn new(app: &'a mut App, address: Addr) -> Self {
        Self { app, address }
    }

    pub fn addr(&self) -> Addr {
        self.address.clone()
    }
}

impl<'a> IGPGasOracle<'a> {
    pub fn init_ownership_transfer(
        &mut self,
        sender: &Addr,
        next_owner: impl Into<String>,
    ) -> anyhow::Result<AppResponse> {
        self.app.execute_contract(
            sender.clone(),
            self.address.clone(),
            &Ownership(OwnableMsg::InitOwnershipTransfer {
                next_owner: next_owner.into(),
            }),
            &[],
        )
    }

    pub fn revoke_ownership_transfer(&mut self, sender: &Addr) -> anyhow::Result<AppResponse> {
        self.app.execute_contract(
            sender.clone(),
            self.address.clone(),
            &Ownership(OwnableMsg::RevokeOwnershipTransfer {}),
            &[],
        )
    }

    pub fn claim_ownership(&mut self, sender: &Addr) -> anyhow::Result<AppResponse> {
        self.app.execute_contract(
            sender.clone(),
            self.address.clone(),
            &Ownership(OwnableMsg::ClaimOwnership {}),
            &[],
        )
    }

    pub fn set_remote_gas_data_configs(
        &mut self,
        sender: &Addr,
        configs: Vec<RemoteGasDataConfig>,
    ) -> anyhow::Result<AppResponse> {
        self.app.execute_contract(
            sender.clone(),
            self.address.clone(),
            &SetRemoteGasDataConfigs { configs },
            &[],
        )
    }

    pub fn set_remote_gas_data(
        &mut self,
        sender: &Addr,
        config: RemoteGasDataConfig,
    ) -> anyhow::Result<AppResponse> {
        self.app.execute_contract(
            sender.clone(),
            self.address.clone(),
            &SetRemoteGasData { config },
            &[],
        )
    }
}

impl<'a> IGPGasOracle<'a> {
    pub fn get_config(&self) -> anyhow::Result<ConfigResponse> {
        Ok(self
            .app
            .wrap()
            .query_wasm_smart(&self.address, &QueryMsg::Config {})?)
    }

    pub fn get_exchange_rate_and_gas_price(
        &self,
        dest_domain: u32,
    ) -> anyhow::Result<GetExchangeRateAndGasPriceResponse> {
        Ok(self.app.wrap().query_wasm_smart(
            &self.address,
            &QueryMsg::GetExchangeRateAndGasPrice { dest_domain },
        )?)
    }
}
