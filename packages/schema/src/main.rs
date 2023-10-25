use std::{
    env::current_dir,
    fs::{create_dir_all, remove_dir_all, write},
};

use cosmwasm_schema::generate_api;
use hpl_interface::{
    core as hpl_core, hook as hpl_hook, igp as hpl_igp, ism as hpl_ism, warp as hpl_warp,
};

macro_rules! fetch_api {
    ($module:ty, $name:tt) => {
        generate_api! {
            name: $name,
            instantiate: $module::InstantiateMsg,
            execute: $module::ExecuteMsg,
            query: $module::QueryMsg,
        }
    };
}

pub fn main() {
    let apis = vec![
        // core
        fetch_api!(hpl_core::mailbox, "mailbox"),
        fetch_api!(hpl_core::va, "validator_announce"),
        // hooks
        fetch_api!(hpl_hook::aggregate, "hook_aggregate"),
        fetch_api!(hpl_hook::merkle, "hook_merkle"),
        fetch_api!(hpl_hook::pausable, "hook_pausable"),
        fetch_api!(hpl_hook::routing, "hook_routing"),
        fetch_api!(hpl_hook::routing_custom, "hook_routing_custom"),
        fetch_api!(hpl_hook::routing_fallback, "hook_routing_fallback"),
        // igps
        fetch_api!(hpl_igp::core, "igp"),
        fetch_api!(hpl_igp::oracle, "igp_oracle"),
        // isms
        fetch_api!(hpl_ism::aggregate, "ism_aggregate"),
        fetch_api!(hpl_ism::multisig, "ism_multisig"),
        fetch_api!(hpl_ism::routing, "ism_routing"),
        // warps
        fetch_api!(hpl_warp::cw20, "warp_cw20"),
        fetch_api!(hpl_warp::native, "warp_native"),
    ];

    let mut base = current_dir().unwrap();
    base.push("schema");

    let _ = remove_dir_all(&base);
    create_dir_all(&base).unwrap();

    for api in apis {
        let filename = api.contract_name.clone();
        let api_str = api.render().to_string().unwrap();

        create_dir_all(base.join(&filename)).unwrap();
        write(base.join(format!("{filename}/{filename}.json")), api_str).unwrap();
    }
}
