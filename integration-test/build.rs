use std::{env::current_dir, path::PathBuf};

use ethers::prelude::Abigen;

fn generate_bind(name: &str, abi_file: &str, bind_out: PathBuf) {
    if bind_out.exists() {
        std::fs::remove_file(&bind_out).unwrap();
    }

    Abigen::new(name, abi_file)
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(bind_out)
        .unwrap();
}

fn main() {
    let abi_base = current_dir().unwrap().join("abis");
    let bind_base = current_dir()
        .unwrap()
        .join("tests")
        .join("contracts/eth/bind");
    let deployments = [
        ("Mailbox", "mailbox"),
        ("FastHypERC20", "fast_hyp_erc20"),
        ("FastHypERC20Collateral", "fast_hyp_erc20_collateral"),
        ("TestMultisigIsm", "test_mock_ism"),
        ("TestRecipient", "test_mock_msg_receiver"),
    ];

    for (abi_file, bind_out) in deployments {
        generate_bind(
            abi_file,
            abi_base.join(format!("{abi_file}.json")).to_str().unwrap(),
            bind_base.join(format!("{bind_out}.rs")),
        );
    }
}
