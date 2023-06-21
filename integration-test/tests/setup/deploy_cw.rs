use std::{collections::BTreeMap, fs, path::Path};

use cosmwasm_schema::cw_serde;
use osmosis_test_tube::{Account, Module, OsmosisTestApp, SigningAccount, Wasm};

#[derive(Debug)]
pub struct HplCwDeploymentCodes {
    pub hub: u64,
    pub ism_multisig: u64,
    pub mailbox: u64,
    pub multicall: u64,

    pub test_mock_ism: u64,
    pub test_mock_msg_receiver: u64,
}

#[derive(Debug)]
pub struct HplCwDeploymentAddrs {
    pub hub: String,
    pub ism: String,
    pub mailbox: String,
    pub receiver: String,
}

#[derive(Debug)]
pub struct HplCwDeployment {
    pub codes: HplCwDeploymentCodes,
    pub addrs: HplCwDeploymentAddrs,
}

pub async fn deploy_cw_hyperlane(
    app: &OsmosisTestApp,
    deployer: &SigningAccount,
) -> eyre::Result<HplCwDeployment> {
    let wasm = Wasm::new(app);

    // store codes
    let base_path = Path::new("../target/wasm32-unknown-unknown/release/");
    let artifacts = [
        ("hub"),
        ("ism_multisig"),
        ("mailbox"),
        ("multicall"),
        ("test_mock_ism"),
        ("test_mock_msg_receiver"),
    ]
    .into_iter()
    .map(|name| {
        let filename = format!("hpl_{name}.wasm");
        let path = base_path.join(filename);
        let code = fs::read(path).unwrap();
        let store_resp = wasm.store_code(&code, None, deployer).unwrap();
        let code_id = store_resp.data.code_id;

        (name, code_id)
    })
    .collect::<BTreeMap<_, _>>();

    let codes = HplCwDeploymentCodes {
        hub: *artifacts.get("hub").unwrap(),
        ism_multisig: *artifacts.get("ism_multisig").unwrap(),
        mailbox: *artifacts.get("mailbox").unwrap(),
        multicall: *artifacts.get("multicall").unwrap(),
        test_mock_ism: *artifacts.get("test_mock_ism").unwrap(),
        test_mock_msg_receiver: *artifacts.get("test_mock_msg_receiver").unwrap(),
    };

    #[cw_serde]
    struct EmptyMsg {}

    let hub = wasm
        .instantiate(
            codes.hub,
            &hpl_interface::hub::InstantiateMsg {
                origin_domain: 2,
                mailbox_code: codes.mailbox,
            },
            None,
            None,
            &[],
            deployer,
        )
        .unwrap()
        .data
        .address;

    let ism = wasm
        .instantiate(codes.test_mock_ism, &EmptyMsg {}, None, None, &[], deployer)
        .unwrap()
        .data
        .address;

    let mailbox_deploy_res = wasm
        .execute(
            &hub,
            &hpl_interface::hub::ExecuteMsg::Instantiate {
                owner: deployer.address(),
                default_ism: ism.clone(),
            },
            &[],
            deployer,
        )
        .unwrap();

    let mailbox = mailbox_deploy_res
        .events
        .into_iter()
        .find(|v| v.ty == "wasm-mailbox_instantiated")
        .unwrap()
        .attributes
        .get(0)
        .unwrap()
        .value
        .clone();

    let msg_receiver = wasm
        .instantiate(
            codes.test_mock_msg_receiver,
            &EmptyMsg {},
            None,
            None,
            &[],
            deployer,
        )
        .unwrap()
        .data
        .address;

    let addrs = HplCwDeploymentAddrs {
        hub,
        ism,
        mailbox,
        receiver: msg_receiver,
    };

    Ok(HplCwDeployment { codes, addrs })
}
