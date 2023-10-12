use cosmwasm_std::{
    from_binary,
    testing::{mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Deps, DepsMut, Empty, Env, HexBinary, MessageInfo, OwnedDeps, Response,
};
use hpl_interface::core::va::{
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
    ) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                addr_prefix: hrp.to_string(),
                mailbox: mailbox.to_string(),
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
        validator: HexBinary,
        storage_location: &str,
        signature: HexBinary,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::Announce {
                validator,
                storage_location: storage_location.to_string(),
                signature,
            },
        )
    }

    #[allow(dead_code)]
    pub fn get_announce_storage_locations(
        &self,
        validators: &[HexBinary],
    ) -> Result<GetAnnounceStorageLocationsResponse, ContractError> {
        self.query(QueryMsg::GetAnnounceStorageLocations {
            validators: validators.to_vec(),
        })
    }

    #[allow(dead_code)]
    pub fn get_announced_validators(
        &self,
    ) -> Result<GetAnnouncedValidatorsResponse, ContractError> {
        self.query(QueryMsg::GetAnnouncedValidators {})
    }
}
