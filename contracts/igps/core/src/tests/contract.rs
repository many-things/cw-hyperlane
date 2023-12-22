use cosmwasm_std::{
    coin, from_json,
    testing::{mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info},
    to_json_binary, Addr, BankMsg, Coin, ContractResult, Empty, HexBinary, Order, QuerierResult,
    QueryRequest, StdResult, SubMsg, SystemResult, Uint128, Uint256, WasmQuery,
};
use hpl_interface::{
    igp::{
        core::{
            DefaultGasResponse, ExecuteMsg, GasForDomainResponse, GasOracleConfig, IgpQueryMsg,
            QueryMsg,
        },
        oracle,
    },
    types::{IGPMetadata, Message},
};
use hpl_ownable::get_owner;
use hpl_router::get_routes;
use ibcx_test_utils::{addr, gen_bz};
use rstest::{fixture, rstest};

use crate::{get_default_gas, BENEFICIARY, DEFAULT_GAS_USAGE, GAS_TOKEN, HRP};

use super::IGP;

const DEC_15: u128 = 10u128.pow(15);
const DEC_9: u128 = 10u128.pow(9);

fn test_mock_querier(v: &WasmQuery) -> QuerierResult {
    let (contract_addr, msg) = match v {
        WasmQuery::Smart { contract_addr, msg } => (contract_addr, from_json(msg).unwrap()),
        _ => unreachable!("only smart query"),
    };

    let mut split = contract_addr.split('/').collect::<Vec<_>>();

    match *split.first().unwrap() {
        "oracle" => match msg {
            oracle::QueryMsg::Oracle(
                oracle::IgpGasOracleQueryMsg::GetExchangeRateAndGasPrice { .. },
            ) => {
                let gas_price = split.pop().unwrap().parse::<u128>().unwrap();
                let exchange_rate = split.pop().unwrap().parse::<u128>().unwrap();

                let res = to_json_binary(&oracle::GetExchangeRateAndGasPriceResponse {
                    gas_price: Uint128::new(gas_price * DEC_9), // 150 gwei gas price
                    exchange_rate: Uint128::new(exchange_rate * DEC_9), // 0.2 exchange rate (remote token less valuable)
                })
                .unwrap();

                SystemResult::Ok(ContractResult::Ok(res))
            }
            _ => unreachable!("unsupported query"),
        },
        _ => unreachable!("unsupported query"),
    }
}

macro_rules! arg_fixture {
    ($name:ident, $typ:ty, $default:expr) => {
        #[fixture]
        fn $name(#[default($default)] v: $typ) -> $typ {
            v
        }
    };
}

arg_fixture!(deployer, Addr, addr("deployer"));
arg_fixture!(hrp, &'static str, "test");
arg_fixture!(owner, Addr, addr("owner"));
arg_fixture!(gas_token, &'static str, "utest");
arg_fixture!(beneficiary, Addr, addr("beneficiary"));

#[fixture]
fn igp(deployer: Addr, hrp: &str, owner: Addr, gas_token: &str, beneficiary: Addr) -> IGP {
    let mut igp = IGP::new(mock_dependencies(), mock_env());

    igp.init(&deployer, hrp, &owner, gas_token, &beneficiary)
        .unwrap();

    igp
}

#[fixture]
fn igp_routes(
    #[default(vec![])] routes: Vec<(u32, String)>,
    mut igp: IGP,
) -> (IGP, Vec<(u32, String)>) {
    let configs: Vec<GasOracleConfig> = routes.iter().map(|v| v.clone().into()).collect();

    igp.set_gas_oracles(&addr("owner"), configs).unwrap();

    (igp, routes)
}

#[rstest]
fn test_init(igp: IGP) {
    let storage = igp.deps_ref().storage;
    assert_eq!(get_owner(storage).unwrap(), "owner");
    assert_eq!(BENEFICIARY.load(storage).unwrap(), "beneficiary");
    assert_eq!(GAS_TOKEN.load(storage).unwrap(), "utest");
    assert_eq!(HRP.load(storage).unwrap(), "test");
}

#[rstest]
#[case(addr("owner"))]
#[should_panic(expected = "unauthorized")]
#[case(addr("mailbox"))]
fn test_set_gas_oracles(mut igp: IGP, #[case] sender: Addr) {
    let configs: Vec<GasOracleConfig> = (1..4)
        .map(|v: u32| (v, format!("oracle/{v}/{v}")).into())
        .collect();

    igp.set_gas_oracles(&sender, configs.clone()).unwrap();

    // check state mutation
    let storage = igp.deps_ref().storage;
    let actual_configs = get_routes::<Addr>(storage, None, None, None)
        .unwrap()
        .into_iter()
        .map(|v| (v.domain, v.route.unwrap().to_string()).into())
        .collect::<Vec<_>>();

    assert_eq!(configs, actual_configs);
}

#[rstest]
#[case(addr("owner"))]
#[should_panic(expected = "unauthorized")]
#[case(addr("mailbox"))]
fn test_set_beneficiary(mut igp: IGP, #[case] sender: Addr) {
    let next_beneficiary = Addr::unchecked("next-beneficiary");

    igp.set_beneficiary(&sender, &next_beneficiary)
        .map_err(|e| e.to_string())
        .unwrap();

    let storage = igp.deps_ref().storage;
    let actual_beneficiary = BENEFICIARY.load(storage).unwrap();

    assert_eq!(next_beneficiary, actual_beneficiary);
}

#[rstest]
fn test_get_beneficiary(igp: IGP) {
    let storage = igp.deps_ref().storage;
    let actual_beneficiary = BENEFICIARY.load(storage).unwrap();

    assert_eq!(addr("beneficiary"), actual_beneficiary);
}

#[rstest]
#[case(1, 300_000)]
#[should_panic(expected = "gas oracle not found for 2")]
#[case(2, 300_000)]
fn test_get_quote_gas_payment(
    #[with(vec![(1, "oracle/2/150".into())])] igp_routes: (IGP, Vec<(u32, String)>),
    #[case] dest_domain: u32,
    #[case] gas_amount: u128,
) {
    let (mut igp, _) = igp_routes;

    igp.deps.querier.update_wasm(test_mock_querier);

    let resp = igp
        .get_quote_gas_payment(dest_domain, gas_amount)
        .map_err(|e| e.to_string())
        .unwrap();
    assert_eq!(resp.gas_needed, Uint256::from_u128(9 * 10u128.pow(15)))
}

#[rstest]
#[case(1)]
#[should_panic(expected = "gas oracle not found for 2")]
#[case(2)]
fn test_gas_exchange(
    #[with(vec![(1, "oracle/2/150".into())])] igp_routes: (IGP, Vec<(u32, String)>),
    #[case] dest_domain: u32,
) {
    let (mut igp, _) = igp_routes;

    igp.deps.querier.update_wasm(test_mock_querier);

    let resp = igp
        .get_exchange_rate_and_gas_price(dest_domain)
        .map_err(|e| e.to_string())
        .unwrap();
    assert_eq!(resp.gas_price, Uint128::new(150 * DEC_9));
    assert_eq!(resp.exchange_rate, Uint128::new(2 * DEC_9));
}

#[rstest]
#[case(1, vec![coin(9 * DEC_15, "utest")])] // exact
#[case(1, vec![coin(10 * DEC_15, "utest")])] // over
#[should_panic(expected = "No funds sent")]
#[case(1, vec![])]
#[should_panic(expected = "Sent more than one denomination")]
#[case(1, vec![coin(1, "token1"), coin(1, "token2")])]
#[should_panic(expected = "Must send reserve token 'utest'")]
#[case(1, vec![coin(1, "test1")])]
#[should_panic(expected = "insufficient funds")]
#[case(1, vec![coin(1, "utest")])] // exact
#[should_panic(expected = "gas oracle not found for 2")]
#[case(2, vec![coin(9 * DEC_15, "utest")])]
fn test_pay_for_gas(
    #[with(vec![(1, "oracle/2/150".into())])] igp_routes: (IGP, Vec<(u32, String)>),
    #[case] dest_domain: u32,
    #[case] gas_paid: Vec<Coin>,
) {
    let (mut igp, _) = igp_routes;

    igp.deps.querier.update_wasm(test_mock_querier);

    let user_payer = addr("user-payer");
    let user_refund = addr("user-refund");
    let gas_amount = 300_000;

    let message_id = gen_bz(8);

    let res = igp
        .pay_for_gas(
            &user_payer,
            &gas_paid,
            &message_id,
            dest_domain,
            gas_amount,
            &user_refund,
        )
        .map_err(|e| e.to_string())
        .unwrap();

    if gas_paid[0].amount.u128() > 9 * DEC_15 {
        assert_eq!(
            res.messages,
            vec![SubMsg::new(BankMsg::Send {
                to_address: user_refund.to_string(),
                amount: vec![coin(gas_paid[0].amount.u128() - 9 * DEC_15, "utest")]
            })]
        );
    } else {
        assert_eq!(res.messages, vec![]);
    }
}

#[rstest]
#[case(addr("mailbox"), true, Some(300_000))]
#[case(addr("mailbox"), true, None)]
#[case(addr("mailbox"), false, None)]
fn test_post_dispatch(
    #[values("osmo", "neutron")] hrp: &str,
    #[with(vec![(1, "oracle/2/150".into())])] igp_routes: (IGP, Vec<(u32, String)>),
    #[case] sender: Addr,
    #[case] refund_diff: bool,
    #[case] gas_limit: Option<u128>,
) {
    let (mut igp, _) = igp_routes;

    igp.deps.querier.update_wasm(test_mock_querier);

    HRP.save(igp.deps_mut().storage, &hrp.into()).unwrap();

    let addr_sender = gen_bz(32);
    let addr_refund = gen_bz(32);

    let metadata = gas_limit
        .map(|v| {
            IGPMetadata {
                gas_limit: Uint256::from_u128(v),
                refund_address: if refund_diff {
                    addr_refund.clone()
                } else {
                    HexBinary::default()
                },
            }
            .into()
        })
        .unwrap_or_default();

    let mut rand_msg: Message = gen_bz(100).into();
    rand_msg.sender = addr_sender;
    rand_msg.dest_domain = 1;

    let res = igp
        .post_dispatch(
            &sender,
            metadata,
            rand_msg.into(),
            vec![coin(9 * DEC_15, "utest")],
        )
        .map_err(|e| e.to_string())
        .unwrap();

    let event = res
        .events
        .into_iter()
        .find(|v| v.ty == "igp-core-pay-for-gas")
        .unwrap();

    let gas_amount_log = event
        .attributes
        .into_iter()
        .find(|v| v.key == "gas_amount")
        .unwrap()
        .value
        .parse::<u128>()
        .unwrap();

    assert_eq!(
        gas_limit.unwrap_or(DEFAULT_GAS_USAGE.load(igp.deps.as_mut().storage).unwrap()),
        gas_amount_log
    );
}

#[rstest]
#[case(addr("beneficiary"), vec![coin(10, "utest")])]
#[should_panic(expected = "unauthorized")]
#[case(addr("owner"), vec![coin(10, "utest")])]
fn test_claim(mut igp: IGP, #[case] sender: Addr, #[case] funds: Vec<Coin>) {
    igp.deps
        .querier
        .update_balance(mock_env().contract.address, funds.clone());

    let res = igp.claim(&sender).map_err(|e| e.to_string()).unwrap();

    assert_eq!(
        *res.messages.first().unwrap(),
        SubMsg::new(BankMsg::Send {
            to_address: sender.to_string(),
            amount: funds
        })
    )
}

#[rstest]
#[case(addr("owner"))]
#[should_panic(expected = "unauthorized")]
#[case(addr("someone"))]
fn test_set_default_gas(mut igp: IGP, #[case] sender: Addr) {
    let _resp = igp
        .execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::SetDefaultGas { gas: 99_999 },
        )
        .map_err(|v| v.to_string())
        .unwrap();

    let storage = igp.deps.as_ref().storage;

    assert_eq!(crate::DEFAULT_GAS_USAGE.load(storage).unwrap(), 99_999);
}

#[rstest]
#[case(addr("owner"))]
#[should_panic(expected = "unauthorized")]
#[case(addr("someone"))]
fn test_set_gas_for_domain(mut igp: IGP, #[case] sender: Addr) {
    let config = (1u32..5u32)
        .map(|i| (i, (i * 100_000) as u128))
        .collect::<Vec<_>>();

    let _resp = igp
        .execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::SetGasForDomain {
                config: config.clone(),
            },
        )
        .map_err(|v| v.to_string())
        .unwrap();

    let storage = igp.deps.as_ref().storage;

    assert_eq!(
        crate::GAS_FOR_DOMAIN
            .range(storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<(u32, u128)>>>()
            .unwrap(),
        config
    );
}

#[rstest]
#[case(addr("owner"))]
#[should_panic(expected = "unauthorized")]
#[case(addr("someone"))]
fn test_unset_gas_for_domain(mut igp: IGP, #[case] sender: Addr) {
    let config = (1u32..5u32)
        .map(|i| (i, (i * 100_000) as u128))
        .collect::<Vec<_>>();

    let _resp = igp
        .execute(
            mock_info("owner", &[]),
            ExecuteMsg::SetGasForDomain {
                config: config.clone(),
            },
        )
        .map_err(|v| v.to_string())
        .unwrap();

    let _resp = igp
        .execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::UnsetGasForDomain {
                domains: config.iter().map(|v| v.0).collect(),
            },
        )
        .map_err(|v| v.to_string())
        .unwrap();

    let storage = igp.deps.as_ref().storage;

    assert!(crate::GAS_FOR_DOMAIN.is_empty(storage));
}

#[rstest]
fn test_get_default_gas(mut igp: IGP) {
    let _resp = igp
        .execute(
            mock_info("owner", &[]),
            ExecuteMsg::SetGasForDomain {
                config: vec![(1, 123_456)],
            },
        )
        .map_err(|v| v.to_string())
        .unwrap();

    let storage = igp.deps.as_ref().storage;

    assert_eq!(get_default_gas(storage, 1).unwrap(), 123_456);
    assert_eq!(get_default_gas(storage, 2).unwrap(), 250_000);
}

#[rstest]
fn test_gas_query(mut igp: IGP) {
    let config = (1u32..100u32)
        .map(|i| (i, (i * 100_000) as u128))
        .collect::<Vec<_>>();

    let _resp = igp
        .execute(
            mock_info("owner", &[]),
            ExecuteMsg::SetGasForDomain { config },
        )
        .map_err(|v| v.to_string())
        .unwrap();

    let DefaultGasResponse { gas: default_gas } = igp
        .query(QueryMsg::Igp(IgpQueryMsg::DefaultGas {}))
        .unwrap();
    assert_eq!(default_gas, 250_000);

    let domain_range = 1u32..4u32;
    let GasForDomainResponse {
        gas: gas_for_domain,
    } = igp
        .query(QueryMsg::Igp(IgpQueryMsg::GasForDomain {
            domains: domain_range.clone().collect(),
        }))
        .unwrap();
    assert_eq!(
        domain_range
            .map(|i| (i, (i * 100_000) as u128))
            .collect::<Vec<_>>(),
        gas_for_domain
    );

    let domain_range = 1u32..11u32;
    let GasForDomainResponse {
        gas: gas_for_domain,
    } = igp
        .query(QueryMsg::Igp(IgpQueryMsg::ListGasForDomains {
            offset: None,
            limit: None,
            order: None,
        }))
        .unwrap();
    assert_eq!(
        domain_range
            .map(|i| (i, (i * 100_000) as u128))
            .collect::<Vec<_>>(),
        gas_for_domain
    );

    let domain_range = (90u32..100u32).rev();
    let GasForDomainResponse {
        gas: gas_for_domain,
    } = igp
        .query(QueryMsg::Igp(IgpQueryMsg::ListGasForDomains {
            offset: None,
            limit: None,
            order: Some(hpl_interface::Order::Desc),
        }))
        .unwrap();
    assert_eq!(
        domain_range
            .map(|i| (i, (i * 100_000) as u128))
            .collect::<Vec<_>>(),
        gas_for_domain
    );
}
