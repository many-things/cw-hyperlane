use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use hpl_interface::igp_gas_oracle::InstantiateMsg;

use crate::contract::{execute, instantiate, query};

fn igp_gas_oracle_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
}

#[test]
fn test_igp_gas_oracle() -> Result<(), anyhow::Error> {
    let mut app = App::default();

    let deployer = Addr::unchecked("deployer");

    let oracle_label = "igp_gas_oracle";
    let oracle_code = app.store_code(igp_gas_oracle_contract());
    let oracle_addr = app.instantiate_contract(
        oracle_code,
        deployer,
        &InstantiateMsg {},
        &[],
        oracle_label,
        None,
    )?;

    print!("{oracle_addr}");

    Ok(())
}
