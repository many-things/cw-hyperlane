#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};
use cw2::set_contract_version;
use hpl_interface::mailbox::{
    CheckPointResponse, CountResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, NonceResponse,
    PausedResponse, QueryMsg, RootResponse,
};
use serde::Serialize;

use crate::{
    error::ContractError,
    event::emit_instantiated,
    merkle::MerkleTree,
    state::{assert_paused, Config, CONFIG, MESSAGE_TREE, NONCE, PAUSE},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        factory: info.sender,
        default_ism: deps.api.addr_validate(&msg.default_ism)?,
    };

    MESSAGE_TREE.save(deps.storage, &MerkleTree::default())?;
    CONFIG.save(deps.storage, &config)?;
    PAUSE.save(deps.storage, &false)?;
    NONCE.save(deps.storage, &0)?;

    Ok(Response::new().add_event(emit_instantiated(config.owner)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::core;
    use crate::gov;
    use ExecuteMsg::*;

    assert_paused(deps.storage)?;

    match msg {
        Pause {} => gov::pause(deps, info),
        Unpause {} => gov::unpause(deps, info),
        SetDefaultISM {
            ism: new_default_ism,
        } => gov::set_default_ism(deps, info, new_default_ism),

        Dispatch {
            dest_domain,
            recipient_addr,
            msg_body,
        } => core::dispatch(deps, info, dest_domain, recipient_addr, msg_body),
        Process { metadata, message } => core::process(deps, metadata, message),
    }
}

fn to_binary<T: Serialize>(res: Result<T, ContractError>) -> Result<QueryResponse, ContractError> {
    match res {
        Ok(v) => Ok(cosmwasm_std::to_binary(&v)?),
        Err(e) => Err(e),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use QueryMsg::*;

    match msg {
        Root {} => to_binary(Ok(&RootResponse {
            root: MESSAGE_TREE.load(deps.storage)?.root()?.into(),
        })),
        Count {} => to_binary(Ok(&CountResponse {
            count: MESSAGE_TREE.load(deps.storage)?.count,
        })),
        CheckPoint {} => to_binary({
            let tree = MESSAGE_TREE.load(deps.storage)?;

            Ok(&CheckPointResponse {
                root: tree.root()?.into(),
                count: tree.count,
            })
        }),
        Paused {} => to_binary(Ok(&PausedResponse {
            paused: PAUSE.load(deps.storage)?,
        })),
        Nonce {} => to_binary(Ok(&NonceResponse {
            nonce: NONCE.load(deps.storage)?,
        })),
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    const OWNER: &str = "owner";
    const DEFAULT_ISM: &str = "default_ism";

    #[test]
    fn init() {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", &[]),
            InstantiateMsg {
                owner: OWNER.to_string(),
                default_ism: DEFAULT_ISM.to_string(),
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(
            version,
            cw2::ContractVersion {
                contract: CONTRACT_NAME.to_string(),
                version: CONTRACT_VERSION.to_string()
            }
        );
    }
}
