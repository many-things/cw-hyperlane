use cosmwasm_std::{
    from_binary,
    testing::{mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, OwnedDeps, Response,
};
use hpl_interface::va::{
    ExecuteMsg, GetAnnounceStorageLocationsResponse, GetAnnouncedValidatorsResponse,
    InstantiateMsg, QueryMsg,
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
};

mod contracts;

pub struct VA {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl VA {
    pub fn new(deps: OwnedDeps<MockStorage, MockApi, MockQuerier>, env: Env) -> Self {
        Self { deps, env }
    }

    #[allow(dead_code)]
    pub fn with_env(&mut self, env: Env) {
        self.env = env
    }

    #[allow(dead_code)]
    pub fn deps_mut(&mut self) -> DepsMut {
        self.deps.as_mut()
    }

    pub fn deps(&self) -> Deps {
        self.deps.as_ref()
    }

    pub fn init(
        &mut self,
        sender: &Addr,
        hrp: &str,
        mailbox: &Addr,
        local_domain: u32,
    ) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                addr_prefix: hrp.to_string(),
                mailbox: mailbox.to_string(),
                local_domain,
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

    pub fn announce(
        &mut self,
        sender: &Addr,
        validator: &Addr,
        storage_location: &str,
        signature: Binary,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::Announce {
                validator: validator.to_string(),
                storage_location: storage_location.to_string(),
                signature,
            },
        )
    }

    #[allow(dead_code)]
    pub fn get_announce_storage_locations(
        &self,
        validators: &[&Addr],
    ) -> Result<GetAnnounceStorageLocationsResponse, ContractError> {
        self.query(QueryMsg::GetAnnounceStorageLocations {
            validators: validators.iter().map(|v| v.to_string()).collect(),
        })
    }

    #[allow(dead_code)]
    pub fn get_announced_validators(
        &self,
    ) -> Result<GetAnnouncedValidatorsResponse, ContractError> {
        self.query(QueryMsg::GetAnnouncedValidators {})
    }
}
