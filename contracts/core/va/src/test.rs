use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Deps, DepsMut, Empty, Env, HexBinary, MessageInfo, OwnedDeps, Response,
};
use hpl_interface::{
    core::va::{
        ExecuteMsg, GetAnnounceStorageLocationsResponse, GetAnnouncedValidatorsResponse,
        InstantiateMsg, QueryMsg,
    },
    types::bech32_encode,
};
use k256::{
    ecdsa::{RecoveryId, Signature, SigningKey},
    elliptic_curve::{rand_core::OsRng, sec1::ToEncodedPoint},
    SecretKey,
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{announcement_hash, domain_hash, execute, instantiate, query},
    error::ContractError,
    eth_hash, pub_to_addr, pub_to_addr_binary,
    state::{HRP, LOCAL_DOMAIN, MAILBOX, STORAGE_LOCATIONS, VALIDATORS},
};

pub struct VA {
    pub deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    pub env: Env,
}

impl VA {
    pub fn new(deps: OwnedDeps<MockStorage, MockApi, MockQuerier>, env: Env) -> Self {
        Self { deps, env }
    }

    #[allow(dead_code)]
    pub fn with_env(&mut self, env: Env) {
        self.env = env
    }

    #[allow(dead_code)]
    pub fn deps_mut(&mut self) -> DepsMut {
        self.deps.as_mut()
    }

    pub fn deps(&self) -> Deps {
        self.deps.as_ref()
    }

    pub fn init(
        &mut self,
        sender: &Addr,
        hrp: &str,
        mailbox: &Addr,
    ) -> Result<Response, ContractError> {
        instantiate(
            self.deps.as_mut(),
            self.env.clone(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                hrp: hrp.to_string(),
                mailbox: mailbox.to_string(),
            },
        )
    }

    fn execute(&mut self, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
        execute(self.deps.as_mut(), self.env.clone(), info, msg)
    }

    fn query<T: DeserializeOwned>(&self, msg: QueryMsg) -> Result<T, ContractError> {
        query(self.deps.as_ref(), self.env.clone(), msg)
            .map(|v| from_binary::<T>(&v))?
            .map_err(|e| e.into())
    }

    pub fn announce(
        &mut self,
        sender: &Addr,
        validator: HexBinary,
        storage_location: &str,
        signature: HexBinary,
    ) -> Result<Response, ContractError> {
        self.execute(
            mock_info(sender.as_str(), &[]),
            ExecuteMsg::Announce {
                validator,
                storage_location: storage_location.to_string(),
                signature,
            },
        )
    }

    #[allow(dead_code)]
    pub fn get_announce_storage_locations(
        &self,
        validators: &[HexBinary],
    ) -> Result<GetAnnounceStorageLocationsResponse, ContractError> {
        self.query(QueryMsg::GetAnnounceStorageLocations {
            validators: validators.to_vec(),
        })
    }

    #[allow(dead_code)]
    pub fn get_announced_validators(
        &self,
    ) -> Result<GetAnnouncedValidatorsResponse, ContractError> {
        self.query(QueryMsg::GetAnnouncedValidators {})
    }
}

struct TestData<'a> {
    pub deployer: Addr,
    pub hrp: &'a str,
    pub mailbox: Addr,
    pub local_domain: u32,
    pub storage_location: &'a str,
}

impl<'a> Default for TestData<'a> {
    fn default() -> Self {
        let hrp = "osmo";

        Self {
            deployer: Addr::unchecked("deployer"),
            hrp,
            mailbox: bech32_encode(hrp, "mailbox_____________".as_bytes()).unwrap(),
            local_domain: 1,
            storage_location: "s3://bucket/key",
        }
    }
}

impl<'a> TestData<'a> {
    pub fn init(&self, va: &mut VA) -> Result<Response, ContractError> {
        va.init(&self.deployer, self.hrp, &self.mailbox)
    }
}

fn pack_signature((rs, v): (Signature, RecoveryId)) -> HexBinary {
    let mut bz = rs.to_bytes().to_vec();
    bz.push(v.to_byte() + 27u8);
    bz.into()
}

#[test]
fn test_init() -> anyhow::Result<()> {
    let testdata = TestData::default();

    let mut va = VA::new(mock_dependencies(), mock_env());

    testdata.init(&mut va)?;

    let storage = va.deps().storage;
    assert_eq!(HRP.load(storage)?, testdata.hrp);
    assert_eq!(MAILBOX.load(storage)?, testdata.mailbox);
    assert_eq!(LOCAL_DOMAIN.load(storage)?, testdata.local_domain);

    Ok(())
}

#[test]
fn test_announce() -> anyhow::Result<()> {
    let testdata = TestData::default();

    let mut va = VA::new(mock_dependencies(), mock_env());

    testdata.init(&mut va)?;

    let secret_key = SecretKey::random(&mut OsRng);
    let public_key = secret_key.public_key();
    let signing_key = SigningKey::from(secret_key);

    let public_key_bz = public_key.to_encoded_point(true).as_bytes().to_vec();
    let addr_binary = pub_to_addr_binary(public_key_bz.clone().into())?;
    let public_key_addr = Addr::unchecked(pub_to_addr(public_key_bz.into(), testdata.hrp)?);

    let verify_digest = eth_hash(announcement_hash(
        domain_hash(testdata.local_domain, testdata.mailbox.as_str())?.to_vec(),
        testdata.storage_location,
    ))?;
    let signature = signing_key
        .sign_prehash_recoverable(&verify_digest)
        .unwrap();
    let signature = pack_signature(signature);

    va.announce(
        &testdata.deployer,
        addr_binary,
        testdata.storage_location,
        signature,
    )?;

    // check state
    assert!(VALIDATORS.has(va.deps().storage, public_key_addr.clone()));
    assert_eq!(
        STORAGE_LOCATIONS.load(va.deps().storage, public_key_addr)?,
        vec![testdata.storage_location]
    );

    Ok(())
}

#[test]
fn test_announce_external() -> anyhow::Result<()> {
    let cases = [
        (
            "0xf9e25a6be80f6d48727e42381fc3c3b7834c0cb4",
            "0x62634b0c56b57fef1c27f25039cfb872875a9eeeb42d80a034f8d6b55ed20d09",
            26658,
            "file:///var/folders/3v/g38z040x54x8l6b160vv66b40000gn/T/.tmp7XoxND/checkpoint",
            "0x6c30e1072f0e23694d3a3a96dc41fc4d17636ce145e83adef3224a6f4732c2db715407b42478c581b6ac1b79e64807a7748935d398a33bf4b73d37924c293c941b",
        ),
        (
            "0xf9e25a6be80f6d48727e42381fc3c3b7834c0cb4",
            "0x62634b0c56b57fef1c27f25039cfb872875a9eeeb42d80a034f8d6b55ed20d09",
            26657,
            "file:///var/folders/3v/g38z040x54x8l6b160vv66b40000gn/T/.tmpBJPK8C/checkpoint",
            "0x76c637d605f683734c672c0437f14ae48520e85fb68b0c0b9c28069f183e3bfc46f0de0655f06937c74b5a0a15f5b8fe37f1d1ad4dd8b64dc55307a2103fedad1c",
        ),
    ];

    let remove_hex_prefix = |v: String| -> String {
        if v.starts_with("0x") {
            v.strip_prefix("0x").unwrap().to_string()
        } else {
            v
        }
    };

    for (validator, mailbox, domain, location, signature) in cases {
        let testdata = TestData {
            mailbox: Addr::unchecked(bech32_encode(
                "osmo",
                HexBinary::from_hex(&remove_hex_prefix(mailbox.to_string()))?.as_slice(),
            )?),
            local_domain: domain,

            ..Default::default()
        };

        let mut va = VA::new(mock_dependencies(), mock_env());

        testdata.init(&mut va)?;

        va.announce(
            &testdata.deployer,
            HexBinary::from_hex(&remove_hex_prefix(validator.to_string()))?,
            location,
            HexBinary::from_hex(&remove_hex_prefix(signature.to_string()))?,
        )?;
    }

    Ok(())
}
