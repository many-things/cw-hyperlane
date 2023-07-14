use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use hpl_interface::igp_gas_oracle::{InstantiateMsg, RemoteGasDataConfig};
use hpl_test_helper::IGPGasOracle;

use crate::contract::{execute, instantiate, query};

fn igp_gas_oracle_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
}

fn setup_igp_gas_oracle<'a>(
    app: &'a mut App,
    deployer: &'a Addr,
) -> anyhow::Result<IGPGasOracle<'a>> {
    let oracle_label = "igp_gas_oracle";
    let oracle_code = app.store_code(igp_gas_oracle_contract());
    let oracle_addr = app.instantiate_contract(
        oracle_code,
        deployer.clone(),
        &InstantiateMsg {},
        &[],
        oracle_label,
        None,
    )?;

    Ok(IGPGasOracle::new(app, oracle_addr))
}

#[test]
fn test_owner() -> anyhow::Result<()> {
    let mut app = App::default();

    let deployer = Addr::unchecked("deployer");
    let next_owner = Addr::unchecked("next_owner");

    let mut oracle = setup_igp_gas_oracle(&mut app, &deployer)?;

    let oracle_config = oracle.get_config()?;
    assert_eq!(oracle_config.owner, deployer.to_string());
    assert_eq!(oracle_config.pending_owner, None);

    // test init transfer ownership
    oracle.init_ownership_transfer(&deployer, &next_owner)?;

    let oracle_config = oracle.get_config()?;
    assert_eq!(oracle_config.owner, deployer.to_string());
    assert_eq!(oracle_config.pending_owner, Some(next_owner.to_string()));

    // test revoke transfer ownership
    oracle.revoke_ownership_transfer(&deployer)?;

    let oracle_config = oracle.get_config()?;
    assert_eq!(oracle_config.owner, deployer.to_string());
    assert_eq!(oracle_config.pending_owner, None);

    // test claim transfer ownership
    oracle.init_ownership_transfer(&deployer, &next_owner)?;
    oracle.claim_ownership(&next_owner)?;

    let oracle_config = oracle.get_config()?;
    assert_eq!(oracle_config.owner, next_owner.to_string());
    assert_eq!(oracle_config.pending_owner, None);

    Ok(())
}

#[test]
fn test_gas_data() -> anyhow::Result<()> {
    let mut app = App::default();

    let deployer = Addr::unchecked("deployer");

    let mut oracle = setup_igp_gas_oracle(&mut app, &deployer)?;

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

mod test_execute_condition {

    use super::*;

    #[test]
    fn test_init_ownership_transfer() -> anyhow::Result<()> {
        let mut app = App::default();

        let deployer = Addr::unchecked("deployer");
        let next_owner = Addr::unchecked("next_owner");

        let mut oracle = setup_igp_gas_oracle(&mut app, &deployer)?;

        // ok
        oracle.init_ownership_transfer(&deployer, &next_owner)?;

        // fail - sender is not owner
        oracle
            .init_ownership_transfer(&next_owner, &deployer)
            .unwrap_err();

        // fail - pending_owner is not empty
        oracle
            .init_ownership_transfer(&deployer, &next_owner)
            .unwrap_err();

        Ok(())
    }

    #[test]
    fn test_revoke_ownership_transfer() -> anyhow::Result<()> {
        let mut app = App::default();

        let deployer = Addr::unchecked("deployer");
        let next_owner = Addr::unchecked("next_owner");

        let mut oracle = setup_igp_gas_oracle(&mut app, &deployer)?;

        // fail - pending_owner is empty
        oracle.revoke_ownership_transfer(&deployer).unwrap_err();

        // initiate ownership transfer
        oracle.init_ownership_transfer(&deployer, &next_owner)?;

        // fail - sender is not owner
        oracle.revoke_ownership_transfer(&next_owner).unwrap_err();

        // ok
        oracle.revoke_ownership_transfer(&deployer)?;

        Ok(())
    }

    #[test]
    fn test_claim_ownership() -> anyhow::Result<()> {
        let mut app = App::default();

        let deployer = Addr::unchecked("deployer");
        let next_owner = Addr::unchecked("next_owner");

        let mut oracle = setup_igp_gas_oracle(&mut app, &deployer)?;

        // fail - pending_owner is empty
        oracle.claim_ownership(&next_owner).unwrap_err();

        // initiate ownership transfer
        oracle.init_ownership_transfer(&deployer, &next_owner)?;

        // fail - sender is not pending_owner
        oracle.claim_ownership(&deployer).unwrap_err();

        // ok
        oracle.claim_ownership(&next_owner)?;

        Ok(())
    }

    #[test]
    fn test_set_remote_gas_data_configs() -> anyhow::Result<()> {
        let mut app = App::default();

        let deployer = Addr::unchecked("deployer");
        let abuser = Addr::unchecked("abuser");

        let mut oracle = setup_igp_gas_oracle(&mut app, &deployer)?;

        // fail - sender is not owner
        oracle
            .set_remote_gas_data_configs(&abuser, vec![])
            .unwrap_err();

        // ok
        oracle
            .set_remote_gas_data_configs(&deployer, vec![])
            .unwrap_err();

        Ok(())
    }

    #[test]
    fn test_set_remote_gas_data() -> anyhow::Result<()> {
        let mut app = App::default();

        let deployer = Addr::unchecked("deployer");
        let abuser = Addr::unchecked("abuser");

        let mut oracle = setup_igp_gas_oracle(&mut app, &deployer)?;

        let gas_config = RemoteGasDataConfig {
            remote_domain: 1u32,
            token_exchange_rate: Uint128::new(103202),
            gas_price: Uint128::new(120943),
        };

        // fail - sender is not owner
        oracle
            .set_remote_gas_data(&abuser, gas_config.clone())
            .unwrap_err();

        // ok
        oracle.set_remote_gas_data(&deployer, gas_config)?;

        Ok(())
    }
}
