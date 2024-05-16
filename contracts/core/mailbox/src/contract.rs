#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ensure, Deps, DepsMut, Empty, Env, MessageInfo, QueryResponse, Response};

use hpl_interface::{
    core::mailbox::{ExecuteMsg, InstantiateMsg, MailboxHookQueryMsg, MailboxQueryMsg, QueryMsg},
    to_binary,
};

use crate::{
    error::ContractError,
    event::emit_instantiated,
    state::{Config, CONFIG, NONCE},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // check hrp is lowercase
    ensure!(
        msg.hrp.chars().all(|v| v.is_lowercase()),
        ContractError::invalid_config("hrp must be lowercase")
    );

    let config = Config {
        hrp: msg.hrp,
        local_domain: msg.domain,
        default_ism: None,
        default_hook: None,
        required_hook: None,
    };

    let owner = deps.api.addr_validate(&msg.owner)?;

    CONFIG.save(deps.storage, &config)?;
    NONCE.save(deps.storage, &0u32)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    Ok(Response::new().add_event(emit_instantiated(owner)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use crate::execute;
    use ExecuteMsg::*;

    match msg {
        Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),

        SetDefaultIsm { ism } => execute::set_default_ism(deps, info, ism),
        SetDefaultHook { hook } => execute::set_default_hook(deps, info, hook),
        SetRequiredHook { hook } => execute::set_required_hook(deps, info, hook),

        Dispatch(msg) => execute::dispatch(deps, info, msg),
        Process { metadata, message } => execute::process(deps, env, info, metadata, message),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query::*;
    use MailboxQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Hook(msg) => match msg {
            MailboxHookQueryMsg::QuoteDispatch { sender, msg } => {
                to_binary(quote_dispatch(deps, sender, msg))
            }
        },
        QueryMsg::Mailbox(msg) => match msg {
            Hrp {} => to_binary(get_hrp(deps)),
            LocalDomain {} => to_binary(get_local_domain(deps)),
            DefaultIsm {} => to_binary(get_default_ism(deps)),
            DefaultHook {} => to_binary(get_default_hook(deps)),
            RequiredHook {} => to_binary(get_required_hook(deps)),
            MessageDelivered { id } => to_binary(get_delivered(deps, id)),
            RecipientIsm { recipient_addr } => to_binary(get_recipient_ism(deps, recipient_addr)),
            Nonce {} => to_binary(get_nonce(deps)),
            LatestDispatchId {} => to_binary(get_latest_dispatch_id(deps)),
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("owner", "osmo", 1)]
    #[case("owner", "neutron", 2)]
    fn init(#[case] owner: String, #[case] hrp: String, #[case] domain: u32) {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", &[]),
            InstantiateMsg { owner, hrp, domain },
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

        let config = CONFIG.load(deps.as_ref().storage).unwrap();

        assert_eq!(config.default_hook, None);
        assert_eq!(config.default_ism, None);
    }
}
