use cosmwasm_std::{to_binary, Addr, ContractResult, HexBinary, SystemResult, WasmQuery};
use hpl_interface::{
    ism::{routing::IsmSet, IsmType, VerifyResponse},
    types::Message,
};
use hpl_ownable::get_owner;

use crate::{state::MODULES, ContractError};

use super::IsmRouting;

fn make_default_message() -> Message {
    Message {
        sender: [0u8; 32].into(),
        recipient: [0u8; 32].into(),

        version: 0,
        nonce: 0,
        origin_domain: 0,
        dest_domain: 0,
        body: Default::default(),
    }
}

#[test]
fn test_init() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let owner = Addr::unchecked("owner");
    let isms = vec![
        IsmSet {
            domain: 1,
            address: "ism1".to_string(),
        },
        IsmSet {
            domain: 2,
            address: "ism2".to_string(),
        },
    ];

    let mut ism = IsmRouting::default();

    ism.init(&deployer, &owner, isms)?;

    let storage = ism.deps.as_ref().storage;
    assert_eq!(owner, get_owner(storage)?);
    assert_eq!(Addr::unchecked("ism1"), MODULES.load(storage, 1)?);
    assert_eq!(Addr::unchecked("ism2"), MODULES.load(storage, 2)?);

    Ok(())
}

#[test]
fn test_set() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let owner = Addr::unchecked("owner");

    let mut ism = IsmRouting::default();

    ism.init(&deployer, &owner, vec![])?;

    let target = IsmSet {
        domain: 1,
        address: "ism1".to_string(),
    };

    // fails if sender is not owner
    let err = ism.set(&deployer, &target).unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // ok
    ism.set(&owner, &target)?;

    // check state

    let storage = ism.deps.as_ref().storage;
    assert_eq!(Addr::unchecked("ism1"), MODULES.load(storage, 1)?);

    Ok(())
}

#[test]
fn test_query() -> anyhow::Result<()> {
    let deployer = Addr::unchecked("deployer");
    let owner = Addr::unchecked("owner");

    let mut ism = IsmRouting::default();

    // register mock handler
    ism.deps.querier.update_wasm(|v| match v {
        WasmQuery::Smart { contract_addr, .. } => SystemResult::Ok(ContractResult::Ok(
            to_binary(&VerifyResponse {
                verified: contract_addr == "ism1",
            })
            .unwrap(),
        )),
        _ => panic!("not implemented"),
    });

    // temp values
    let chain_src_domain = 1;
    let chain_a_domain = 2;
    let chain_b_domain = 3;

    // init
    let isms = vec![
        IsmSet {
            domain: chain_a_domain,
            address: "ism1".to_string(),
        },
        IsmSet {
            domain: chain_b_domain,
            address: "ism2".to_string(),
        },
    ];

    ism.init(&deployer, &owner, isms)?;

    // check module type query
    assert_eq!(IsmType::Routing, ism.get_module_type()?.typ);

    // check verify query
    let err_not_found = ContractError::RouteNotFound {};
    for (domain, expect) in [
        (chain_a_domain, Ok(true)),
        (chain_b_domain, Ok(false)),
        (chain_src_domain, Err(err_not_found)),
    ] {
        let res = ism.query_verify(
            HexBinary::default(),
            Message {
                origin_domain: domain,
                dest_domain: chain_src_domain,
                ..make_default_message()
            }
            .into(),
        );
        assert_eq!(res.map(|v| v.verified), expect);
    }

    // check route query
    let err_not_found = ContractError::RouteNotFound {};
    for (domain, expect) in [
        (chain_a_domain, Ok("ism1")),
        (chain_b_domain, Ok("ism2")),
        (chain_src_domain, Err(err_not_found)),
    ] {
        let res = ism.query_route(
            Message {
                origin_domain: domain,
                dest_domain: chain_src_domain,
                ..make_default_message()
            }
            .into(),
        );
        assert_eq!(res.map(|v| v.ism), expect.map(|v| v.to_string()));
    }

    Ok(())
}
