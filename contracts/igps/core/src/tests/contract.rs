use cosmwasm_std::{
    coin,
    testing::{mock_dependencies, mock_env},
    to_binary, Addr, ContractResult, HexBinary, QuerierResult, Response, SystemResult, Uint128,
    Uint256,
};
use cw_utils::PaymentError;
use hpl_interface::igp::{core::GasOracleConfig, oracle};

use crate::{
    state::{BENEFICIARY, GAS_TOKEN},
    ContractError,
};

use super::IGP;

struct TestData<'a> {
    pub deployer: Addr,
    pub owner: Addr,
    pub gas_token: &'a str,
    pub beneficiary: Addr,
}

impl<'a> Default for TestData<'a> {
    fn default() -> Self {
        Self {
            deployer: Addr::unchecked("deployer"),
            owner: Addr::unchecked("owner"),
            gas_token: "gas",
            beneficiary: Addr::unchecked("beneficiary"),
        }
    }
}

impl<'a> TestData<'a> {
    pub fn init(&self, igp: &mut IGP) -> Result<Response, ContractError> {
        igp.init(
            &self.deployer,
            &self.owner,
            self.gas_token,
            &self.beneficiary,
        )
    }
}

#[test]
fn test_init() -> anyhow::Result<()> {
    let testdata = TestData::default();

    let mut igp = IGP::new(mock_dependencies(), mock_env());

    testdata.init(&mut igp)?;

    let storage = igp.deps_ref().storage;
    assert_eq!(hpl_ownable::get_owner(storage)?, testdata.owner);
    assert_eq!(BENEFICIARY.load(storage)?, testdata.beneficiary);
    assert_eq!(GAS_TOKEN.load(storage)?, testdata.gas_token);

    Ok(())
}

#[test]
fn test_setter() -> anyhow::Result<()> {
    let testdata = TestData::default();

    let mut igp = IGP::new(mock_dependencies(), mock_env());

    testdata.init(&mut igp)?;

    // test set_gas_oracles
    let configs: Vec<GasOracleConfig> = (1..4)
        .map(|v: u32| (v, format!("oracle-{v}")).into())
        .collect();

    // fails if caller is not owner
    let err = igp
        .set_gas_oracles(&testdata.deployer, configs.clone())
        .unwrap_err();
    assert!(matches!(err, ContractError::Unauthorized {}));

    // passes if caller is owner
    igp.set_gas_oracles(&testdata.owner, configs.clone())?;

    // check state mutation
    let storage = igp.deps_ref().storage;
    let actual_configs = hpl_router::get_routes::<Addr>(storage, None, None, None)?
        .into_iter()
        .map(|v| GasOracleConfig {
            remote_domain: v.domain,
            gas_oracle: v.route.unwrap().to_string(),
        })
        .collect::<Vec<_>>();

    assert_eq!(configs, actual_configs);

    // test set_beneficiary
    let next_beneficiary = Addr::unchecked("next-beneficiary");

    // fails if caller is not owner
    let err = igp
        .set_beneficiary(&testdata.deployer, &next_beneficiary)
        .unwrap_err();
    assert!(matches!(err, ContractError::Unauthorized {}));

    // passes if caller is owner
    igp.set_beneficiary(&testdata.owner, &next_beneficiary)?;

    // check state mutation
    let storage = igp.deps_ref().storage;
    let actual_beneficiary = BENEFICIARY.load(storage)?;

    assert_eq!(next_beneficiary, actual_beneficiary);

    Ok(())
}

#[test]
fn test_gas_query() -> anyhow::Result<()> {
    let testdata = TestData::default();

    let mut igp = IGP::new(mock_dependencies(), mock_env());

    testdata.init(&mut igp)?;

    // mock gas oracle query
    igp.deps.querier.update_wasm(|v| -> QuerierResult {
        match v {
            cosmwasm_std::WasmQuery::Smart { .. } => SystemResult::Ok(ContractResult::Ok(
                to_binary(&oracle::GetExchangeRateAndGasPriceResponse {
                    gas_price: Uint128::new(150 * 10u128.pow(9)), // 150 gwei gas price
                    exchange_rate: Uint128::new(2 * 10u128.pow(9)), // 0.2 exchange rate (remote token less valuable)
                })
                .unwrap(),
            )),
            _ => panic!("not implemented"),
        }
    });

    // register gas oracle config
    let yes_domain = 1;
    let no_domain = 2;

    igp.set_gas_oracles(
        &testdata.owner,
        vec![GasOracleConfig {
            remote_domain: yes_domain,
            gas_oracle: "igp-gas-oracle".to_string(),
        }],
    )?;

    // test quote gas payment

    let test_gas_amount = 300_000;

    // fails if config does not exists for domain
    let err = igp
        .get_quote_gas_payment(no_domain, test_gas_amount)
        .unwrap_err();
    assert!(matches!(err, ContractError::GasOracleNotFound {}));

    // ok
    let resp = igp.get_quote_gas_payment(yes_domain, test_gas_amount)?;
    assert_eq!(resp.gas_needed, Uint256::from_u128(9 * 10u128.pow(15)));

    // test query exchange rate and gas price

    // fails if config does not exists for domain
    let err = igp.get_exchange_rate_and_gas_price(no_domain).unwrap_err();
    assert!(matches!(err, ContractError::GasOracleNotFound {}));

    // ok
    let resp = igp.get_exchange_rate_and_gas_price(yes_domain)?;
    assert_eq!(resp.gas_price, Uint128::new(150 * 10u128.pow(9)));
    assert_eq!(resp.exchange_rate, Uint128::new(2 * 10u128.pow(9)));

    Ok(())
}

#[test]
fn test_pay_for_gas() -> anyhow::Result<()> {
    let testdata = TestData::default();

    let mut igp = IGP::new(mock_dependencies(), mock_env());

    testdata.init(&mut igp)?;

    // mock gas oracle query
    igp.deps.querier.update_wasm(|v| -> QuerierResult {
        match v {
            cosmwasm_std::WasmQuery::Smart { .. } => SystemResult::Ok(ContractResult::Ok(
                to_binary(&oracle::GetExchangeRateAndGasPriceResponse {
                    gas_price: Uint128::new(150 * 10u128.pow(9)), // 150 gwei gas price
                    exchange_rate: Uint128::new(2 * 10u128.pow(9)), // 0.2 exchange rate (remote token less valuable)
                })
                .unwrap(),
            )),
            _ => panic!("not implemented"),
        }
    });

    // register gas oracle config
    let yes_domain = 1;
    let no_domain = 2;

    igp.set_gas_oracles(
        &testdata.owner,
        vec![GasOracleConfig {
            remote_domain: yes_domain,
            gas_oracle: "igp-gas-oracle".to_string(),
        }],
    )?;

    // test pay_for_gas
    let user = Addr::unchecked("user");
    let refund_addr = Addr::unchecked("refund");
    let gas_paid_exact = coin(9 * 10u128.pow(15), testdata.gas_token);
    let gas_paid_over = coin(10 * 10u128.pow(15), testdata.gas_token);
    let gas_amount = 300_000;

    let message_id = HexBinary::from_hex("deadbeefdeadbeefdeadbeef")?;

    // check missing oracle error
    let err = igp
        .pay_for_gas(
            &user,
            &[gas_paid_exact.clone()],
            &message_id,
            no_domain,
            gas_amount,
            &refund_addr,
        )
        .unwrap_err();
    assert_eq!(err, ContractError::GasOracleNotFound {});

    // check payment errors
    for (funds, expected_err) in [
        (
            // fails if received nothing
            vec![],
            ContractError::PaymentError(PaymentError::NoFunds {}),
        ),
        (
            // fails if received multiple denoms
            vec![coin(1, "token1"), coin(1, "token2")],
            ContractError::PaymentError(PaymentError::MultipleDenoms {}),
        ),
        (
            // fails if received wrong denom
            vec![coin(1, "token1")],
            ContractError::PaymentError(PaymentError::MissingDenom(testdata.gas_token.to_string())),
        ),
        (
            // fails if received less than required amount
            vec![coin(1, testdata.gas_token)],
            ContractError::PaymentError(PaymentError::NonPayable {}),
        ),
    ] {
        let err = igp
            .pay_for_gas(
                &user,
                funds.as_slice(),
                &message_id,
                yes_domain,
                gas_amount,
                &refund_addr,
            )
            .unwrap_err();
        assert_eq!(err, expected_err);
    }

    // ok with no message
    let resp = igp.pay_for_gas(
        &user,
        &[gas_paid_exact],
        &message_id,
        yes_domain,
        gas_amount,
        &refund_addr,
    )?;
    assert_eq!(resp.messages.len(), 0);

    // ok with refund message
    let resp = igp.pay_for_gas(
        &user,
        &[gas_paid_over],
        &message_id,
        yes_domain,
        gas_amount,
        &refund_addr,
    )?;
    assert_eq!(resp.messages.len(), 1);

    Ok(())
}
