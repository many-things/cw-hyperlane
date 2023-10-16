use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Empty, Env, HexBinary, MessageInfo, OwnedDeps, Response,
};
use hpl_interface::ism::{
    routing::{ExecuteMsg, InstantiateMsg, IsmSet, QueryMsg, RouteResponse, RoutingIsmQueryMsg},
    IsmQueryMsg, ModuleTypeResponse, VerifyResponse,
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    ContractError,
};

mod contract;

pub struct IsmRouting {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl Default for IsmRouting {
    fn default() -> Self {
        Self {
            deps: mock_dependencies(),
            env: mock_env(),
        }
    }
}

impl IsmRouting {
    #[allow(dead_code)]
    pub fn new(deps: OwnedDeps<MockStorage, MockApi, MockQuerier>, env: Env) -> Self {
        Self { deps, env }
    }

    pub fn init(
        &mut self,
        sender: &Addr,
        owner: &Addr,
        isms: Vec<IsmSet>,
    ) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                owner: owner.to_string(),
                isms,
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

    pub fn set(&mut self, sender: &Addr, ism: &IsmSet) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::Set { ism: ism.clone() },
        )
    }

    pub fn get_module_type(&self) -> Result<ModuleTypeResponse, ContractError> {
        self.query(QueryMsg::Ism(IsmQueryMsg::ModuleType {}))
    }

    pub fn query_verify(
        &self,
        metadata: HexBinary,
        message: HexBinary,
    ) -> Result<VerifyResponse, ContractError> {
        self.query(QueryMsg::Ism(IsmQueryMsg::Verify { metadata, message }))
    }

    pub fn query_route(&self, message: HexBinary) -> Result<RouteResponse, ContractError> {
        self.query(QueryMsg::RoutingIsm(RoutingIsmQueryMsg::Route { message }))
    }
}
