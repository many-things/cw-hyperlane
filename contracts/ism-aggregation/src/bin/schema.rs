use std::{env::current_dir, fs::remove_dir_all};

use cosmwasm_schema::write_api;

use hpl_interface::ism::{
    owned::{ExecuteMsg, InstantiateMsg, MigrateMsg},
    ISMQueryMsg,
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        migrate: MigrateMsg,
        execute: ExecuteMsg,
        query: ISMQueryMsg,
    }

    let mut raw_dir = current_dir().unwrap();
    raw_dir.push("schema");
    remove_dir_all(raw_dir.join("raw")).unwrap();
}
