#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, Addr, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response, StdError,
};
use cw_storage_plus::Item;
use hpl_interface::{
    hook::{
        merkle::{self, ExecuteMsg, InstantiateMsg, MerkleHookQueryMsg, QueryMsg},
        HookQueryMsg, MailboxResponse, PostDispatchMsg, QuoteDispatchResponse,
    },
    to_binary,
    types::{MerkleTree, Message},
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

pub const MESSAGE_TREE_KEY: &str = "message_tree";
pub const MESSAGE_TREE: Item<MerkleTree> = Item::new(MESSAGE_TREE_KEY);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_hook_merkle::{}", name))
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

    MAILBOX.save(deps.storage, &mailbox)?;
    MESSAGE_TREE.save(deps.storage, &MerkleTree::default())?;

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
        ExecuteMsg::PostDispatch(PostDispatchMsg { message, .. }) => {
            ensure_eq!(
                MAILBOX.load(deps.storage)?,
                info.sender,
                ContractError::Unauthorized {}
            );

            let decoded_msg: Message = message.into();

            MESSAGE_TREE.update(deps.storage, |mut tree| {
                tree.insert(decoded_msg.id())?;

                Ok::<_, ContractError>(tree)
            })?;

            // do nothing
            Ok(Response::new().add_event(
                new_event("post_dispatch").add_attribute("message_id", decoded_msg.id().to_hex()),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use MerkleHookQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::Mailbox {} => to_binary(get_mailbox(deps)),
            HookQueryMsg::QuoteDispatch(_) => to_binary(quote_dispatch()),
        },
        QueryMsg::MerkleHook(msg) => match msg {
            Count {} => to_binary(get_tree_count(deps)),
            Root {} => to_binary(get_tree_root(deps)),
            Branch {} => to_binary(get_tree_branch(deps)),
            Tree {} => to_binary(get_tree(deps)),
            CheckPoint {} => to_binary(get_tree_checkpoint(deps)),
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

fn get_tree_count(deps: Deps) -> Result<merkle::CountResponse, ContractError> {
    let tree = MESSAGE_TREE.load(deps.storage)?;

    Ok(merkle::CountResponse {
        count: tree.count as u32,
    })
}

fn get_tree_root(deps: Deps) -> Result<merkle::RootResponse, ContractError> {
    let tree = MESSAGE_TREE.load(deps.storage)?;

    Ok(merkle::RootResponse { root: tree.root()? })
}

fn get_tree_branch(deps: Deps) -> Result<merkle::BranchResponse, ContractError> {
    let tree = MESSAGE_TREE.load(deps.storage)?;

    Ok(merkle::BranchResponse {
        branch: tree.branch,
    })
}

fn get_tree(deps: Deps) -> Result<merkle::TreeResponse, ContractError> {
    let tree = MESSAGE_TREE.load(deps.storage)?;

    Ok(merkle::TreeResponse {
        branch: tree.branch,
        count: tree.count as u32,
    })
}

fn get_tree_checkpoint(deps: Deps) -> Result<merkle::CheckPointResponse, ContractError> {
    let tree = MESSAGE_TREE.load(deps.storage)?;

    Ok(merkle::CheckPointResponse {
        root: tree.root()?,
        count: tree.count as u32,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        HexBinary, OwnedDeps,
    };

    use hpl_interface::{build_test_executor, build_test_querier, hook::QuoteDispatchMsg};
    use hpl_ownable::get_owner;
    use ibcx_test_utils::gen_bz;
    use rstest::{fixture, rstest};

    use crate::{execute, instantiate};

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    build_test_executor!(self::execute);
    build_test_querier!(self::query);

    #[fixture]
    fn deps(
        #[default(Addr::unchecked("deployer"))] sender: Addr,
        #[default(Addr::unchecked("owner"))] owner: Addr,
        #[default(Addr::unchecked("mailbox"))] mailbox: Addr,
    ) -> TestDeps {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                owner: owner.to_string(),
                mailbox: mailbox.to_string(),
            },
        )
        .unwrap();

        deps
    }

    #[rstest]
    fn test_init(deps: TestDeps) {
        assert_eq!("owner", get_owner(deps.as_ref().storage).unwrap().as_str());
        assert_eq!(
            "mailbox",
            MAILBOX.load(deps.as_ref().storage).unwrap().as_str()
        );
        assert_eq!(
            MerkleTree::default(),
            MESSAGE_TREE.load(deps.as_ref().storage).unwrap()
        );
    }

    #[rstest]
    #[case("mailbox")]
    #[should_panic(expected = "unauthorized")]
    #[case("owner")]
    fn test_post_dispatch(mut deps: TestDeps, #[case] sender: &str) {
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

        let tree = MESSAGE_TREE.load(deps.as_ref().storage).unwrap();
        assert_ne!(tree, MerkleTree::default());
    }

    #[rstest]
    fn test_queries(deps: TestDeps) {
        let res: MailboxResponse =
            test_query(deps.as_ref(), QueryMsg::Hook(HookQueryMsg::Mailbox {}));
        assert_eq!("mailbox", res.mailbox.as_str());

        let res: QuoteDispatchResponse = test_query(
            deps.as_ref(),
            QueryMsg::Hook(HookQueryMsg::QuoteDispatch(QuoteDispatchMsg::default())),
        );
        assert_eq!(res.gas_amount, None);

        let res: merkle::CountResponse = test_query(
            deps.as_ref(),
            QueryMsg::MerkleHook(MerkleHookQueryMsg::Count {}),
        );
        assert_eq!(res.count, 0);

        let res: merkle::RootResponse = test_query(
            deps.as_ref(),
            QueryMsg::MerkleHook(MerkleHookQueryMsg::Root {}),
        );
        assert_eq!(res.root, MerkleTree::default().root().unwrap());

        let res: merkle::BranchResponse = test_query(
            deps.as_ref(),
            QueryMsg::MerkleHook(MerkleHookQueryMsg::Branch {}),
        );
        assert_eq!(res.branch, MerkleTree::default().branch);

        let res: merkle::TreeResponse = test_query(
            deps.as_ref(),
            QueryMsg::MerkleHook(MerkleHookQueryMsg::Tree {}),
        );
        assert_eq!(res.branch, MerkleTree::default().branch);

        let res: merkle::CheckPointResponse = test_query(
            deps.as_ref(),
            QueryMsg::MerkleHook(MerkleHookQueryMsg::CheckPoint {}),
        );
        assert_eq!(res.root, MerkleTree::default().root().unwrap());
    }
}
