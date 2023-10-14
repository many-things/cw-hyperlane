#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, Addr, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response,
    StdError,
};
use cw_storage_plus::Item;
use hpl_interface::{
    hook::{
        pausable::{ExecuteMsg, InstantiateMsg, QueryMsg},
        HookQueryMsg, MailboxResponse, QuoteDispatchResponse,
    },
    to_binary,
};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("hook paused")]
    Paused {},
}

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_hook_pausable::{}", name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    let mailbox = deps.api.addr_validate(&msg.mailbox)?;

    hpl_ownable::initialize(deps.storage, &owner)?;
    hpl_pausable::initialize(deps.storage, &msg.paused)?;

    MAILBOX.save(deps.storage, &mailbox)?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("mailbox", mailbox),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::Pausable(msg) => Ok(hpl_pausable::handle(deps, env, info, msg)?),
        ExecuteMsg::PostDispatch(_) => {
            ensure_eq!(
                MAILBOX.load(deps.storage)?,
                info.sender,
                ContractError::Unauthorized {}
            );

            ensure!(
                !hpl_pausable::get_pause_info(deps.storage)?,
                ContractError::Paused {}
            );

            // do nothing
            Ok(Response::new())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::Pausable(msg) => Ok(hpl_pausable::handle_query(deps, env, msg)?),
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::Mailbox {} => to_binary(get_mailbox(deps)),
            HookQueryMsg::QuoteDispatch(_) => to_binary(quote_dispatch()),
        },
    }
}

fn get_mailbox(deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: MAILBOX.load(deps.storage)?.into(),
    })
}

fn quote_dispatch() -> Result<QuoteDispatchResponse, ContractError> {
    Ok(QuoteDispatchResponse { gas_amount: None })
}

#[cfg(test)]
mod test {
    use cosmwasm_schema::serde::{de::DeserializeOwned, Serialize};
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        HexBinary, OwnedDeps,
    };
    use hpl_interface::hook::{PostDispatchMsg, QuoteDispatchMsg};
    use hpl_ownable::get_owner;
    use hpl_pausable::get_pause_info;
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::{fixture, rstest};

    use super::*;

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    fn query<S: Serialize, T: DeserializeOwned>(deps: Deps, msg: S) -> T {
        let req: QueryMsg = from_binary(&cosmwasm_std::to_binary(&msg).unwrap()).unwrap();
        let res = crate::query(deps, mock_env(), req).unwrap();
        from_binary(&res).unwrap()
    }

    #[fixture]
    fn deps(
        #[default(addr("deployer"))] sender: Addr,
        #[default(addr("owner"))] owner: Addr,
        #[default(addr("mailbox"))] mailbox: Addr,
        #[default(false)] paused: bool,
    ) -> TestDeps {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                owner: owner.to_string(),
                mailbox: mailbox.to_string(),
                paused,
            },
        )
        .unwrap();

        deps
    }

    #[rstest]
    fn test_init(deps: TestDeps) {
        assert!(!get_pause_info(deps.as_ref().storage).unwrap());
        assert_eq!("owner", get_owner(deps.as_ref().storage).unwrap().as_str());
        assert_eq!(
            "mailbox",
            MAILBOX.load(deps.as_ref().storage).unwrap().as_str()
        );
    }

    #[rstest]
    #[case("mailbox", false)]
    #[should_panic(expected = "hook paused")]
    #[case("mailbox", true)]
    #[should_panic(expected = "unauthorized")]
    #[case("owner", false)]
    fn test_post_dispatch(mut deps: TestDeps, #[case] sender: &str, #[case] paused: bool) {
        if paused {
            hpl_pausable::pause(deps.as_mut().storage, &addr("owner")).unwrap();
        }

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(sender, &[]),
            ExecuteMsg::PostDispatch(PostDispatchMsg {
                metadata: HexBinary::default(),
                message: gen_bz(100),
            }),
        )
        .map_err(|e| e.to_string())
        .unwrap();
    }

    #[rstest]
    fn test_query(deps: TestDeps) {
        let res: MailboxResponse = query(deps.as_ref(), QueryMsg::Hook(HookQueryMsg::Mailbox {}));
        assert_eq!("mailbox", res.mailbox.as_str());

        let res: QuoteDispatchResponse = query(
            deps.as_ref(),
            QueryMsg::Hook(HookQueryMsg::QuoteDispatch(QuoteDispatchMsg::default())),
        );
        assert_eq!(res.gas_amount, None);
    }
}
