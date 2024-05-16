#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Empty, Env, Event, MessageInfo,
    QueryResponse, Response, StdError,
};
use cw_storage_plus::Item;
use hpl_interface::{
    hook::{
        fee::{ExecuteMsg, FeeHookMsg, FeeHookQueryMsg, FeeResponse, InstantiateMsg, QueryMsg},
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

    #[error("{0}")]
    MigrationError(#[from] hpl_utils::MigrationError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("hook paused")]
    Paused {},
}

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const COIN_FEE_KEY: &str = "coin_fee";
pub const COIN_FEE: Item<Coin> = Item::new(COIN_FEE_KEY);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_hook_fee::{}", name))
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

    hpl_ownable::initialize(deps.storage, &owner)?;
    COIN_FEE.save(deps.storage, &msg.fee)?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("fee_denom", msg.fee.denom)
            .add_attribute("fee_amount", msg.fee.amount),
    ))
}

fn get_fee(deps: Deps) -> Result<FeeResponse, ContractError> {
    let fee = COIN_FEE.load(deps.storage)?;

    Ok(FeeResponse { fee })
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
        ExecuteMsg::FeeHook(msg) => match msg {
            FeeHookMsg::SetFee { fee } => {
                let owner = hpl_ownable::get_owner(deps.storage)?;
                ensure_eq!(owner, info.sender, StdError::generic_err("unauthorized"));

                COIN_FEE.save(deps.storage, &fee)?;

                Ok(Response::new().add_event(
                    new_event("set_fee")
                        .add_attribute("fee_denom", fee.denom)
                        .add_attribute("fee_amount", fee.amount),
                ))
            }
            FeeHookMsg::Claim { recipient } => {
                let owner = hpl_ownable::get_owner(deps.storage)?;
                ensure_eq!(owner, info.sender, StdError::generic_err("unauthorized"));

                let recipient = recipient.unwrap_or(owner);
                let balances = deps.querier.query_all_balances(&env.contract.address)?;

                let claim_msg: CosmosMsg = BankMsg::Send {
                    to_address: recipient.into_string(),
                    amount: balances,
                }
                .into();

                Ok(Response::new()
                    .add_message(claim_msg)
                    .add_event(new_event("claim")))
            }
        },
        ExecuteMsg::PostDispatch(_) => {
            let fee = COIN_FEE.load(deps.storage)?;
            let supplied = cw_utils::must_pay(&info, &fee.denom)?;

            ensure!(
                supplied.u128() >= fee.amount.u128(),
                // TODO: improve error
                StdError::generic_err("insufficient funds")
            );

            Ok(Response::new().add_event(
                new_event("post_dispatch")
                    .add_attribute("paid_denom", fee.denom)
                    .add_attribute("paid_amount", supplied),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::Mailbox {} => to_binary(get_mailbox(deps)),
            HookQueryMsg::QuoteDispatch(_) => to_binary(quote_dispatch(deps)),
        },
        QueryMsg::FeeHook(FeeHookQueryMsg::Fee {}) => to_binary(get_fee(deps)),
    }
}

fn get_mailbox(_deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: "unrestricted".to_string(),
    })
}

fn quote_dispatch(deps: Deps) -> Result<QuoteDispatchResponse, ContractError> {
    let fee = COIN_FEE.load(deps.storage)?;
    Ok(QuoteDispatchResponse { fees: vec![fee] })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    hpl_utils::migrate(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use cosmwasm_schema::serde::{de::DeserializeOwned, Serialize};
    use cosmwasm_std::{
        coin, from_json,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_json_binary, Addr, HexBinary, OwnedDeps,
    };
    use hpl_interface::hook::{PostDispatchMsg, QuoteDispatchMsg};
    use hpl_ownable::get_owner;
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::{fixture, rstest};

    use super::*;

    type TestDeps = OwnedDeps<MockStorage, MockApi, MockQuerier>;

    fn query<S: Serialize, T: DeserializeOwned>(deps: Deps, msg: S) -> T {
        let req: QueryMsg = from_json(to_json_binary(&msg).unwrap()).unwrap();
        let res = crate::query(deps, mock_env(), req).unwrap();
        from_json(res).unwrap()
    }

    #[fixture]
    fn deps(
        #[default(addr("deployer"))] sender: Addr,
        #[default(addr("owner"))] owner: Addr,
        #[default(coin(100, "uusd"))] fee: Coin,
    ) -> TestDeps {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                owner: owner.to_string(),
                fee,
            },
        )
        .unwrap();

        deps
    }

    #[rstest]
    fn test_init(deps: TestDeps) {
        assert_eq!("uusd", get_fee(deps.as_ref()).unwrap().fee.denom.as_str());
        assert_eq!("owner", get_owner(deps.as_ref().storage).unwrap().as_str());
    }

    #[rstest]
    #[case(&[coin(100, "uusd")])]
    #[should_panic(expected = "Generic error: insufficient funds")]
    #[case(&[coin(99, "uusd")])]
    fn test_post_dispatch(mut deps: TestDeps, #[case] funds: &[Coin]) {
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", funds),
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
        assert_eq!("unrestricted", res.mailbox.as_str());

        let res: QuoteDispatchResponse = query(
            deps.as_ref(),
            QueryMsg::Hook(HookQueryMsg::QuoteDispatch(QuoteDispatchMsg::default())),
        );
        assert_eq!(res.fees, vec![coin(100, "uusd")]);
    }

    #[rstest]
    #[case(addr("owner"), coin(200, "uusd"))]
    #[should_panic(expected = "unauthorized")]
    #[case(addr("deployer"), coin(200, "uusd"))]
    fn test_set_fee(mut deps: TestDeps, #[case] sender: Addr, #[case] fee: Coin) {
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::FeeHook(FeeHookMsg::SetFee { fee: fee.clone() }),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        assert_eq!(fee, get_fee(deps.as_ref()).unwrap().fee);
    }

    #[rstest]
    #[case(addr("owner"), Some(addr("deployer")))]
    #[case(addr("owner"), None)]
    #[should_panic(expected = "unauthorized")]
    #[case(addr("deployer"), None)]
    fn test_claim(mut deps: TestDeps, #[case] sender: Addr, #[case] recipient: Option<Addr>) {
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::FeeHook(FeeHookMsg::Claim {
                recipient: recipient.clone(),
            }),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        assert_eq!(
            CosmosMsg::Bank(BankMsg::Send {
                to_address: recipient.unwrap_or_else(|| addr("owner")).into_string(),
                amount: vec![],
            }),
            res.messages[0].msg
        );
        println!("{:?}", res);
    }
}
