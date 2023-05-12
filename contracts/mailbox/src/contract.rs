#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use hpl_interface::mailbox::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::{
    error::ContractError,
    event::emit_instantiated,
    state::{Config, CONFIG, NONCE, PAUSE},
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

    assert!(!PAUSE.load(deps.storage)?, "paused");

    match msg {
        Pause => gov::pause(deps, info),
        Unpause => gov::unpause(deps, info),
        SetDefaultISM(new_default_ism) => gov::set_default_ism(deps, info, new_default_ism),

        Dispatch {
            dest_domain,
            recipient_addr,
            msg_body,
        } => core::dispatch(deps, info, dest_domain, recipient_addr, msg_body),
        Process { metadata, message } => core::process(deps, metadata, message),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Find matched incoming message variant and query them your custom logic
        // and then construct your query response with the type usually defined
        // `msg.rs` alongside with the query message itself.
        //
        // use `cosmwasm_std::to_binary` to serialize query response to json binary.
    }
}
