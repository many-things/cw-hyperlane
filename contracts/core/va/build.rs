use std::{env::current_dir, fs::remove_dir_all};

use cosmwasm_schema::write_api;

use hpl_interface::core::va::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    write_api! {
        instantiate: InstantiateMsg,
        migrate: MigrateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }

    let mut raw_dir = current_dir().unwrap();
    raw_dir.push("schema");
    remove_dir_all(raw_dir.join("raw")).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
