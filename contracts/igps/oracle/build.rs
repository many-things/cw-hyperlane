use std::{env::current_dir, fs::remove_dir_all};

use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use hpl_interface::igp::oracle::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        migrate: Empty,
        execute: ExecuteMsg,
        query: QueryMsg,
    }

    let mut raw_dir = current_dir().unwrap();
    raw_dir.push("schema");
    remove_dir_all(raw_dir.join("raw")).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}