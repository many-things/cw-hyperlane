use cosmwasm_std::{
    testing::{mock_dependencies, mock_info},
    Binary, HexBinary,
};
use ethers::types::{Address, H160};
use ethers::utils::hex::FromHex;
use hpl_interface::{
    ism::multisig::{ThresholdSet, ValidatorSet},
    types::{
        bech32_encode, eth_addr, eth_hash, pub_to_addr, Message, MessageIdMultisigIsmMetadata,
    },
};
use ibcx_test_utils::{addr, gen_bz};
use k256::{
    ecdsa::{RecoveryId, SigningKey, VerifyingKey},
    elliptic_curve::rand_core::OsRng,
};

#[derive(Clone)]
pub struct TestValidator {
    pub priv_key: SigningKey,
    pub pub_key: VerifyingKey,
}

impl TestValidator {
    fn random() -> Self {
        let priv_key = SigningKey::random(&mut OsRng);
        Self {
            pub_key: VerifyingKey::from(&priv_key),
            priv_key,
        }
    }

    fn from_key(priv_key_hex: &str) -> Self {
        let h = <Vec<u8>>::from_hex(priv_key_hex).unwrap();
        let priv_key = SigningKey::from_bytes(h.as_slice().into()).unwrap();
        let pub_key = VerifyingKey::from(&priv_key);

        Self { priv_key, pub_key }
    }

    fn pub_key_to_binary(&self) -> HexBinary {
        self.pub_key
            .to_encoded_point(true)
            .as_bytes()
            .to_vec()
            .into()
    }

    pub fn addr(&self, hrp: &str) -> String {
        bech32_encode(
            hrp,
            pub_to_addr(self.pub_key_to_binary()).unwrap().as_slice(),
        )
        .unwrap()
        .into()
    }

    pub fn to_val(&self, domain: u32) -> ValidatorSet {
        ValidatorSet {
            domain,
            validator: eth_addr(self.pub_key.to_encoded_point(false).as_bytes().into()).unwrap(),
        }
    }

    pub fn sign(&self, digest: [u8; 32]) -> (Binary, RecoveryId) {
        let (sign, recov_id) = self.priv_key.sign_prehash_recoverable(&digest).unwrap();

        (Binary(sign.to_bytes().to_vec()), recov_id)
    }
}

#[derive(Clone)]
pub struct TestValidators {
    pub domain: u32,
    pub validators: Vec<TestValidator>,
    pub threshold: u8,
}

impl TestValidators {
    pub fn new(domain: u32, num: u8, threshold: u8) -> Self {
        assert!(num >= threshold);

        let validators = vec![0; num as usize]
            .into_iter()
            .map(|_| TestValidator::random())
            .collect::<Vec<_>>();

        Self {
            domain,
            validators,
            threshold,
        }
    }

    #[allow(dead_code)]
    pub fn from_keys(domain: u32, keys: &[String], threshold: u8) -> Self {
        assert!(keys.len() as u8 >= threshold);

        let validators = keys
            .iter()
            .map(|k| TestValidator::from_key(k))
            .collect::<Vec<_>>();

        Self {
            domain,
            validators,
            threshold,
        }
    }

    pub fn to_set(&self) -> Vec<ValidatorSet> {
        self.validators
            .iter()
            .map(|v| v.to_val(self.domain))
            .collect::<Vec<_>>()
    }

    pub fn sign(&self, num: u8, digest: [u8; 32]) -> Vec<HexBinary> {
        let num = num as usize;
        assert!(self.validators.len() >= num);

        let signatures = &self.validators[0..num]
            .iter()
            .map(|v| {
                let (mut signature, recov_id) = v.sign(digest);
                signature.0.extend(vec![recov_id.to_byte() + 27]);
                signature.into()
            })
            .collect::<Vec<_>>();

        signatures.clone()
    }

    pub fn make_metadata(
        &self,
        origin_merkle_tree: Address,
        merkle_root: [u8; 32],
        merkle_index: u32,
        message_id: [u8; 32],
        is_passed: bool,
    ) -> eyre::Result<MessageIdMultisigIsmMetadata> {
        let mut addr = [0u8; 32];
        addr[32 - origin_merkle_tree.0.len()..].copy_from_slice(&origin_merkle_tree.0);

        let multisig_hash = hpl_ism_multisig::multisig_hash(
            hpl_ism_multisig::domain_hash(self.domain, addr.to_vec().into())?.to_vec(),
            merkle_root.to_vec(),
            merkle_index,
            message_id.to_vec(),
        )?;

        let hashed_message = eth_hash(multisig_hash)?;

        let signatures = if is_passed {
            self.sign(self.threshold, hashed_message.as_slice().try_into()?)
        } else {
            self.sign(self.threshold - 1, hashed_message.as_slice().try_into()?)
        };

        Ok(MessageIdMultisigIsmMetadata {
            origin_merkle_tree: origin_merkle_tree.0.to_vec().into(),
            merkle_root: merkle_root.to_vec().into(),
            merkle_index: merkle_index.to_be_bytes().to_vec().into(),
            signatures,
        })
    }
}

#[test]
fn test_validator() {
    let owner = addr("owner");
    let validators = TestValidators::new(2, 5, 3);

    let mut deps = mock_dependencies();

    hpl_ownable::initialize(deps.as_mut().storage, &owner).unwrap();

    hpl_ism_multisig::execute::enroll_validators(
        deps.as_mut(),
        mock_info(owner.as_str(), &[]),
        validators.to_set(),
    )
    .unwrap();

    hpl_ism_multisig::execute::set_threshold(
        deps.as_mut(),
        mock_info(owner.as_str(), &[]),
        ThresholdSet {
            domain: validators.domain,
            threshold: validators.threshold,
        },
    )
    .unwrap();

    let mut message: Message = gen_bz(100).into();
    message.origin_domain = validators.domain;

    let message_id = message.id();

    let metadata = validators
        .make_metadata(
            H160::from_slice(gen_bz(20).as_slice()),
            gen_bz(32).as_slice().try_into().unwrap(),
            0,
            message_id.as_slice().try_into().unwrap(),
            true,
        )
        .unwrap();

    let res =
        hpl_ism_multisig::query::verify_message(deps.as_ref(), metadata.into(), message.into())
            .unwrap();
    assert!(res.verified);
}
