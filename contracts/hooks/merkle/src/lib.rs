#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, Addr, Deps, DepsMut, Empty, Env, Event, MessageInfo, QueryResponse, Response,
    StdError,
};
use cw_storage_plus::Item;
use hpl_interface::{
    core::mailbox::{LatestDispatchedIdResponse, MailboxQueryMsg},
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

    #[error("unauthorized. reason: {0}")]
    Unauthorized(String),

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("hook paused")]
    Paused {},
}

impl ContractError {
    pub fn unauthorized(reason: &str) -> Self {
        ContractError::Unauthorized(reason.into())
    }
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

    let mailbox = deps.api.addr_validate(&msg.mailbox)?;

    MAILBOX.save(deps.storage, &mailbox)?;
    MESSAGE_TREE.save(deps.storage, &MerkleTree::default())?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("mailbox", mailbox),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::PostDispatch(PostDispatchMsg { message, .. }) => {
            let mailbox = MAILBOX.load(deps.storage)?;

            let latest_dispatch_id = deps
                .querier
                .query_wasm_smart::<LatestDispatchedIdResponse>(
                    &mailbox,
                    &MailboxQueryMsg::LatestDispatchId {}.wrap(),
                )?
                .message_id;

            let decoded_msg: Message = message.into();

            ensure_eq!(
                latest_dispatch_id,
                decoded_msg.id(),
                ContractError::unauthorized("message is not dispatching")
            );

            let mut tree = MESSAGE_TREE.load(deps.storage)?;
            let index = tree.count;
            tree.insert(decoded_msg.id())?;
            MESSAGE_TREE.save(deps.storage, &tree)?;

            // do nothing
            Ok(Response::new()
                .add_event(
                    new_event("post_dispatch")
                        .add_attribute("message_id", decoded_msg.id().to_hex())
                        .add_attribute("index", index.to_string()),
                )
                .add_event(
                    new_event("inserted_into_tree").add_attribute("index", index.to_string()),
                ))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use MerkleHookQueryMsg::*;

    match msg {
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
    Ok(QuoteDispatchResponse { fees: vec![] })
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
        count: if tree.count == 0 {
            0
        } else {
            tree.count as u32 - 1
        },
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use super::*;

    use cosmwasm_std::{
        from_json,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        HexBinary, OwnedDeps, WasmQuery,
    };

    use hpl_interface::{
        build_test_executor, build_test_querier, core::mailbox, hook::QuoteDispatchMsg,
    };
    use ibcx_test_utils::hex;
    use rstest::{fixture, rstest};

    use crate::{execute, instantiate};

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    build_test_executor!(self::execute);
    build_test_querier!(self::query);

    #[fixture]
    fn deps(
        #[default(Addr::unchecked("deployer"))] sender: Addr,
        #[default(Addr::unchecked("mailbox"))] mailbox: Addr,
    ) -> TestDeps {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                mailbox: mailbox.to_string(),
            },
        )
        .unwrap();

        deps
    }

    #[rstest]
    fn test_init(deps: TestDeps) {
        assert_eq!(
            "mailbox",
            MAILBOX.load(deps.as_ref().storage).unwrap().as_str()
        );
        assert_eq!(
            MerkleTree::default(),
            MESSAGE_TREE.load(deps.as_ref().storage).unwrap()
        );
    }

    const TEST_MESSAGE: &str = "dc7b240deb74cca40636435ade8514b7ac35176e085f810e92dbc8bdb54a3d554ef32b9f724df19861d7e9b89a8ed11a4ecb35512f58b18b6607689cb9ba36dcf0f4af3cc1c7128c6cf0b47ea1f1aa07a4fe64502edd9a2b2e2dddf770776040efa24f19";
    const TEST_MESSAGE_FAIL: &str = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

    #[rstest]
    #[case("mailbox", None)]
    #[should_panic(expected = "unauthorized")]
    #[case("mailbox", Some(hex(TEST_MESSAGE_FAIL)))]
    fn test_post_dispatch(
        mut deps: TestDeps,
        #[case] sender: &str,
        #[case] message: Option<HexBinary>,
    ) {
        deps.querier.update_wasm(|query| {
            use cosmwasm_std::{to_json_binary, ContractResult, SystemResult};

            let (_contract_addr, msg) = match query {
                WasmQuery::Smart { contract_addr, msg } => (contract_addr, from_json(msg).unwrap()),
                _ => unreachable!("noo"),
            };

            match msg {
                mailbox::QueryMsg::Mailbox(MailboxQueryMsg::LatestDispatchId {}) => {
                    let res = LatestDispatchedIdResponse {
                        message_id: hex(
                            "a6d8af738f99da8a0a8a3611e6c777bc9ebf42b1f685a5ff6b1ff1f2b7b70f45",
                        ),
                    };
                    SystemResult::Ok(ContractResult::Ok(to_json_binary(&res).unwrap()))
                }
                _ => unreachable!("unwrap noo"),
            }
        });

        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(sender, &[]),
            ExecuteMsg::PostDispatch(PostDispatchMsg {
                metadata: HexBinary::default(),
                message: message.unwrap_or(hex(TEST_MESSAGE)),
            }),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        assert_eq!(
            res.events
                .iter()
                .find(|v| v.ty == "hpl_hook_merkle::inserted_into_tree")
                .unwrap()
                .attributes
                .last()
                .unwrap()
                .value,
            "0"
        );

        let tree = MESSAGE_TREE.load(deps.as_ref().storage).unwrap();
        assert_ne!(tree, MerkleTree::default());
        assert_eq!(tree.count, 1);
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
        assert_eq!(res.fees, vec![]);

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
