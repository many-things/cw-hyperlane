#[allow(dead_code)]
mod constants;
mod contracts;
mod event;
mod validator;

use cosmwasm_std::Uint128;
use hpl_interface::{igp::oracle::RemoteGasDataConfig, warp};
use osmosis_test_tube::OsmosisTestApp;
use rstest::rstest;
use test_tube::{Module, Wasm};

use crate::{
    constants::*,
    contracts::{cw, eth},
    validator::TestValidators,
};

#[tokio::test]
async fn test_cw20_colleteral() -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        None::<&str>,
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    let wasm = Wasm::new(&osmo_app);

    let warp = cw::deploy_warp_route_collateral(
        &wasm,
        &osmo.acc_owner,
        &osmo.acc_deployer,
        &osmo.core.mailbox,
        "osmo",
        &osmo.codes,
        "osmo122ryl7pez7yjprtvjckltu2uvjxrq3kqt4nvclax2la7maj6757qg054ga".into(),
    )?;

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_cw20_bridged() -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        None::<&str>,
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    let wasm = Wasm::new(&osmo_app);

    let warp = cw::deploy_warp_route_bridged(
        &wasm,
        &osmo.acc_owner,
        &osmo.acc_deployer,
        &osmo.core.mailbox,
        "osmo",
        &osmo.codes,
        "cw20-denom".into(),
        warp::TokenType::CW20 {
            contract: "".into(),
        },
    )?;

    Ok(())
}

#[rstest]
#[case("ibc/B5CB286F69D48B2C4F6F8D8CF59011C40590DCF8A91617A5FBA9FF0A7B21307F")]
#[case("uosmo")]
#[tokio::test]
async fn test_native_collateral(#[case] denom: &str) -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        None::<&str>,
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    let wasm = Wasm::new(&osmo_app);

    let warp = cw::deploy_warp_route_collateral(
        &wasm,
        &osmo.acc_owner,
        &osmo.acc_deployer,
        &osmo.core.mailbox,
        "osmo",
        &osmo.codes,
        denom.into(),
    )?;

    Ok(())
}

#[rstest]
#[case("ibc/B5CB286F69D48B2C4F6F8D8CF59011C40590DCF8A91617A5FBA9FF0A7B21307F")]
#[case("uosmo")]
#[tokio::test]
async fn test_native_bridged(#[case] denom: &str) -> eyre::Result<()> {
    let osmo_app = OsmosisTestApp::new();
    let osmo = cw::setup_env(
        &osmo_app,
        |app, coins| app.init_account(coins).unwrap(),
        None::<&str>,
        "osmo",
        DOMAIN_OSMO,
        &[TestValidators::new(DOMAIN_EVM, 5, 3)],
        &[RemoteGasDataConfig {
            remote_domain: DOMAIN_EVM,
            token_exchange_rate: Uint128::from(10u128.pow(4)),
            gas_price: Uint128::from(10u128.pow(9)),
        }],
    )?;

    let wasm = Wasm::new(&osmo_app);

    let warp = cw::deploy_warp_route_bridged(
        &wasm,
        &osmo.acc_owner,
        &osmo.acc_deployer,
        &osmo.core.mailbox,
        "osmo",
        &osmo.codes,
        denom.into(),
        warp::TokenType::Native(warp::TokenTypeNative::Fungible { denom: "".into() }),
    )?;

    Ok(())
}
