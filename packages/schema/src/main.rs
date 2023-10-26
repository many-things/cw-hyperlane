use std::{
    env::current_dir,
    fs::{create_dir_all, remove_dir_all, write},
};

use cosmwasm_schema::generate_api;
use cosmwasm_std::Empty;
use hpl_interface::{
    core as hpl_core, hook as hpl_hook, igp as hpl_igp, ism as hpl_ism, warp as hpl_warp,
};

pub fn main() {
    let mut apis = vec![];

    {
        use hpl_core::mailbox::*;

        apis.push(generate_api! {
            name: "hpl_mailbox",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_core::va::*;

        apis.push(generate_api! {
            name: "hpl_validator_announce",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_hook::merkle::*;

        apis.push(generate_api! {
            name: "hpl_hook_merkle",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_hook::pausable::*;

        apis.push(generate_api! {
            name: "hpl_hook_pausable",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_hook::routing::*;

        apis.push(generate_api! {
            name: "hpl_hook_routing",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_hook::routing_custom::*;

        apis.push(generate_api! {
            name: "hpl_hook_routing_custom",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_hook::routing_fallback::*;

        apis.push(generate_api! {
            name: "hpl_hook_routing_fallback",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_igp::core::*;

        apis.push(generate_api! {
            name: "hpl_igp",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_igp::oracle::*;

        apis.push(generate_api! {
            name: "hpl_igp_oracle",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_ism::multisig::*;

        apis.push(generate_api! {
            name: "hpl_ism_multisig",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    {
        use hpl_ism::routing::*;

        apis.push(generate_api! {
            name: "hpl_ism_routing",
            instantiate: InstantiateMsg,
            migrate: Empty,
            execute: ExecuteMsg,
            query: QueryMsg,
        });
    }

    // {
    //     use hpl_warp::cw20::*;

    //     apis.push(generate_api! {
    //         name: "hpl_warp_cw20",
    //         instantiate: InstantiateMsg,
    //         migrate: Empty,
    //         execute: ExecuteMsg,
    //         query: QueryMsg,
    //     });
    // }

    // {
    //     use hpl_warp::native::*;

    //     apis.push(generate_api! {
    //         name: "hpl_warp_native",
    //         instantiate: InstantiateMsg,
    //         migrate: Empty,
    //         execute: ExecuteMsg,
    //         query: QueryMsg,
    //     });
    // }

    let mut base = current_dir().unwrap();
    base.push("schema");

    let _ = remove_dir_all(&base);
    create_dir_all(&base).unwrap();

    for api in apis {
        let filename = api.contract_name.clone();
        let api_str = api.render().to_string().unwrap();

        write(base.join(format!("{filename}.json")), api_str).unwrap();
    }
}
