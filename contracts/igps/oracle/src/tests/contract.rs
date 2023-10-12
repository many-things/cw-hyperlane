use cosmwasm_std::{
    testing::{mock_dependencies, mock_env},
    Addr, Uint128,
};
use hpl_interface::igp::oracle::RemoteGasDataConfig;

use crate::{error::ContractError, tests::IGPGasOracle};

#[test]
fn test_gas_data() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");

    let mut oracle = IGPGasOracle::new(mock_dependencies(), mock_env());
    oracle.init(&deployer)?;

    // test single
    let gas_config = RemoteGasDataConfig {
        remote_domain: 1u32,
        token_exchange_rate: Uint128::new(120921),
        gas_price: Uint128::new(9120321),
    };

    oracle.set_remote_gas_data(&deployer, gas_config.clone())?;

    let ret = oracle.get_exchange_rate_and_gas_price(gas_config.remote_domain)?;
    assert_eq!(ret.exchange_rate, gas_config.token_exchange_rate);
    assert_eq!(ret.gas_price, gas_config.gas_price);

    // test multi
    let gas_config = RemoteGasDataConfig {
        remote_domain: 2u32,
        token_exchange_rate: Uint128::new(120921),
        gas_price: Uint128::new(9120321),
    };

    oracle.set_remote_gas_data_configs(&deployer, vec![gas_config.clone()])?;

    let ret = oracle.get_exchange_rate_and_gas_price(gas_config.remote_domain)?;
    assert_eq!(ret.exchange_rate, gas_config.token_exchange_rate);
    assert_eq!(ret.gas_price, gas_config.gas_price);

    Ok(())
}

#[test]
fn test_set_remote_gas_data_configs() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let abuser = Addr::unchecked("abuser");

    let mut oracle = IGPGasOracle::new(mock_dependencies(), mock_env());
    oracle.init(&deployer)?;

    // fail - sender is not owner
    let err = oracle
        .set_remote_gas_data_configs(&abuser, vec![])
        .unwrap_err();
    assert!(matches!(err, ContractError::Unauthorized {}));

    // ok
    oracle.set_remote_gas_data_configs(&deployer, vec![])?;

    Ok(())
}

#[test]
fn test_set_remote_gas_data() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let abuser = Addr::unchecked("abuser");

    let mut oracle = IGPGasOracle::new(mock_dependencies(), mock_env());
    oracle.init(&deployer)?;

    let gas_config = RemoteGasDataConfig {
        remote_domain: 1u32,
        token_exchange_rate: Uint128::new(103202),
        gas_price: Uint128::new(120943),
    };

    // fail - sender is not owner
    let err = oracle
        .set_remote_gas_data(&abuser, gas_config.clone())
        .unwrap_err();
    assert!(matches!(err, ContractError::Unauthorized {}));

    // ok
    oracle.set_remote_gas_data(&deployer, gas_config)?;

    Ok(())
}
