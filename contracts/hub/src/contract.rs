#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use hpl_interface::{
    hub::{ExecuteMsg, InstantiateMsg, MigrateMsg, OriginDomainResponse, QueryMsg},
    mailbox,
};

use crate::{error::ContractError, CONTRACT_NAME, CONTRACT_VERSION};
use crate::{
    event::emit_instantiated,
    state::{MAILBOX_CODE, ORIGIN_DOMAIN},
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ORIGIN_DOMAIN.save(deps.storage, &msg.origin_domain)?;
    MAILBOX_CODE.save(deps.storage, &msg.mailbox_code)?;

    Ok(Response::new().add_event(emit_instantiated(info.sender)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Instantiate { owner, default_ism } => {
            deps.api.addr_validate(&owner)?;
            deps.api.addr_validate(&default_ism)?;

            let mailbox_code = MAILBOX_CODE.load(deps.storage)?;

            let resp = Response::new().add_message(WasmMsg::Instantiate {
                admin: Some(env.contract.address.into_string()),
                code_id: mailbox_code,
                msg: to_binary(&mailbox::InstantiateMsg { owner, default_ism })?,
                funds: vec![],
                label: "hyperlane-mailbox".to_string(),
            });

            Ok(resp)
        }
        Migrate {} => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        OriginDomain {} => to_binary(&OriginDomainResponse {
            domain: ORIGIN_DOMAIN.load(deps.storage)?,
        }),
    }
}
