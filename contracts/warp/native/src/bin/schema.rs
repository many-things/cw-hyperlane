use std::{env::current_dir, fs::remove_dir_all};

use cosmwasm_schema::write_api;

use hpl_interface::token_native::{ExecuteMsg, QueryMsg};
use hpl_token_native::msg::{InstantiateMsg, MigrateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        migrate: MigrateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }

    let mut raw_dir = current_dir().unwrap();
    raw_dir.push("schema");
    remove_dir_all(raw_dir.join("raw")).unwrap();
}
