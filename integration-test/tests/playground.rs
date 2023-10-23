use std::collections::BTreeMap;

use cosmwasm_std::{
    coin, wasm_execute, BankMsg, Binary, CosmosMsg, Empty, Event, Uint128, WasmMsg,
};
use hpl_interface::igp::oracle::RemoteGasDataConfig;
use osmosis_test_tube::OsmosisTestApp;
use test_tube::{Module, Wasm};

use crate::{
    constants::{DOMAIN_EVM, DOMAIN_OSMO},
    contracts::cw::{self, instantiate},
    validator::TestValidators,
};

#[allow(dead_code)]
mod constants;
mod contracts;
mod event;
mod validator;

fn events(events: Vec<Event>) -> BTreeMap<String, BTreeMap<String, String>> {
    events
        .into_iter()
        .map(|v| {
            (
                v.ty,
                v.attributes
                    .into_iter()
                    .map(|x| (x.key, x.value))
                    .collect::<BTreeMap<_, _>>(),
            )
        })
        .collect::<BTreeMap<_, _>>()
}

#[tokio::test]
async fn test_playground() -> eyre::Result<()> {
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

    let wasm = Wasm::new(osmo.app);

    let multicall = instantiate(
        &wasm,
        osmo.codes.test_mock_multicall,
        &osmo.acc_deployer,
        "multicall",
        &Empty {},
    )
    .data
    .address;

    let bz: Binary = serde_json::to_vec(&CosmosMsg::<Empty>::Bank(BankMsg::Send {
        to_address: multicall.clone(),
        amount: vec![coin(1, "uosmo")],
    }))?
    .into();

    let bzf: Binary = format!(
        "'{}'",
        serde_json::to_string(&CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: multicall.clone(),
            amount: vec![coin(1, "uosmo")],
        }))?
    )
    .as_bytes()
    .to_vec()
    .into();

    let _resp = wasm
        .execute(
            &multicall,
            &CosmosMsg::<Empty>::Wasm(WasmMsg::Execute {
                contract_addr: multicall.clone(),
                msg: bzf,
                funds: vec![],
            }),
            &[coin(1, "uosmo")],
            &osmo.acc_deployer,
        )
        .unwrap_err();

    let resp = wasm.execute(
        &multicall,
        &CosmosMsg::<Empty>::Wasm(WasmMsg::Execute {
            contract_addr: multicall.clone(),
            msg: bz,
            funds: vec![],
        }),
        &[coin(1, "uosmo")],
        &osmo.acc_deployer,
    )?;
    println!("{}", serde_json::to_string_pretty(&events(resp.events))?);

    Ok(())
}
