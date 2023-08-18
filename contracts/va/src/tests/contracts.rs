use cosmwasm_std::{
    testing::{mock_dependencies, mock_env},
    Addr, Binary, Response,
};
use hpl_interface::types::bech32_encode;
use k256::{
    ecdsa::{RecoveryId, Signature, SigningKey},
    elliptic_curve::{rand_core::OsRng, sec1::ToEncodedPoint},
    schnorr::signature::hazmat::PrehashSigner,
    SecretKey,
};

use crate::{
    contract::{announcement_hash, domain_hash},
    error::ContractError,
    pub_to_addr, pub_to_addr_binary,
    state::{ADDR_PREFIX, LOCAL_DOMAIN, MAILBOX, STORAGE_LOCATIONS, VALIDATORS},
};

use super::VA;

struct TestData<'a> {
    pub deployer: Addr,
    pub addr_prefix: &'a str,
    pub mailbox: Addr,
    pub local_domain: u32,
    pub storage_location: &'a str,
}

impl<'a> Default for TestData<'a> {
    fn default() -> Self {
        let addr_prefix = "osmo";

        Self {
            deployer: Addr::unchecked("deployer"),
            addr_prefix,
            mailbox: bech32_encode(addr_prefix, "mailbox_____________".as_bytes()).unwrap(),
            local_domain: 1,
            storage_location: "s3://bucket/key",
        }
    }
}

impl<'a> TestData<'a> {
    pub fn init(&self, va: &mut VA) -> Result<Response, ContractError> {
        va.init(
            &self.deployer,
            self.addr_prefix,
            &self.mailbox,
            self.local_domain,
        )
    }
}

fn pack_signature((rs, v): (Signature, RecoveryId)) -> Binary {
    let mut bz = rs.to_bytes().to_vec();
    bz.push(v.to_byte());
    Binary(bz)
}

#[test]
fn test_init() -> anyhow::Result<()> {
    let testdata = TestData::default();

    let mut va = VA::new(mock_dependencies(), mock_env());

    testdata.init(&mut va)?;

    let storage = va.deps().storage;
    assert_eq!(ADDR_PREFIX.load(storage)?, testdata.addr_prefix);
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

    let public_key_bz = Binary(public_key.to_encoded_point(false).as_bytes().to_vec());
    let addr_binary = pub_to_addr_binary(public_key_bz.clone())?;
    let public_key_addr = Addr::unchecked(pub_to_addr(public_key_bz, testdata.addr_prefix)?);

    let announcement_hash = announcement_hash(
        domain_hash(testdata.local_domain, testdata.mailbox.as_str())?.0,
        testdata.storage_location,
    );
    let signature = pack_signature(signing_key.sign_prehash(&announcement_hash)?);

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
