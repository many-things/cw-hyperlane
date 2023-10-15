use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Empty, Env, HexBinary, MessageInfo, OwnedDeps, Response,
};
use hpl_interface::ism::{
    routing::{ExecuteMsg, ISMSet, InstantiateMsg, QueryMsg, RouteResponse, RoutingIsmQueryMsg},
    ISMQueryMsg, ModuleTypeResponse, VerifyResponse,
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    ContractError,
};

mod contract;

pub struct ISMRouting {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl Default for ISMRouting {
    fn default() -> Self {
        Self {
            deps: mock_dependencies(),
            env: mock_env(),
        }
    }
}

impl ISMRouting {
    #[allow(dead_code)]
    pub fn new(deps: OwnedDeps<MockStorage, MockApi, MockQuerier>, env: Env) -> Self {
        Self { deps, env }
    }

    pub fn init(
        &mut self,
        sender: &Addr,
        owner: &Addr,
        isms: Vec<ISMSet>,
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

    pub fn set(&mut self, sender: &Addr, ism: &ISMSet) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::Set { ism: ism.clone() },
        )
    }

    pub fn get_module_type(&self) -> Result<ModuleTypeResponse, ContractError> {
        self.query(QueryMsg::ISM(ISMQueryMsg::ModuleType {}))
    }

    pub fn query_verify(
        &self,
        metadata: HexBinary,
        message: HexBinary,
    ) -> Result<VerifyResponse, ContractError> {
        self.query(QueryMsg::ISM(ISMQueryMsg::Verify { metadata, message }))
    }

    pub fn query_route(&self, message: HexBinary) -> Result<RouteResponse, ContractError> {
        self.query(QueryMsg::RoutingIsm(RoutingIsmQueryMsg::Route { message }))
    }
}
