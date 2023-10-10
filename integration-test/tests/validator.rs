use cosmwasm_std::Binary;
use ethers::types::Address;
use ethers::utils::hex::FromHex;
use hpl_interface::{ism::multisig::ValidatorSet, types::metadata::MessageIdMultisigIsmMetadata};
use k256::{
    ecdsa::{RecoveryId, SigningKey, VerifyingKey},
    elliptic_curve::rand_core::OsRng,
};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

pub fn sha256_digest(bz: impl AsRef<[u8]>) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(bz);

    hasher.finalize().as_slice().try_into().unwrap()
}

pub fn ripemd160_digest(bz: impl AsRef<[u8]>) -> [u8; 20] {
    let mut hasher = Ripemd160::new();

    hasher.update(bz);

    hasher.finalize().as_slice().try_into().unwrap()
}

fn pub_to_addr(pub_key: Binary, prefix: &str) -> String {
    let sha_hash = sha256_digest(pub_key);
    let rip_hash = ripemd160_digest(sha_hash);

    let addr = hpl_interface::types::bech32_encode(prefix, &rip_hash).unwrap();

    addr.to_string()
}

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

    fn pub_key_to_binary(&self) -> Binary {
        Binary(self.pub_key.to_encoded_point(true).as_bytes().to_vec())
    }

    pub fn addr(&self, hrp: &str) -> String {
        pub_to_addr(self.pub_key_to_binary(), hrp)
    }

    pub fn to_val(&self, domain: u32, hrp: &str) -> ValidatorSet {
        ValidatorSet {
            domain,
            validator: self.addr(hrp),
            validator_pubkey: self.pub_key_to_binary(),
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

    pub fn to_set(&self, hrp: &str) -> Vec<ValidatorSet> {
        self.validators
            .iter()
            .map(|v| v.to_val(self.domain, hrp))
            .collect::<Vec<_>>()
    }

    pub fn sign(&self, num: u8, digest: [u8; 32]) -> Vec<Binary> {
        let num = num as usize;
        assert!(self.validators.len() >= num);

        let signatures = &self.validators[0..num]
            .iter()
            .map(|v| {
                let (mut signature, recov_id) = v.sign(digest);
                signature.0.extend(vec![recov_id.to_byte()]);
                signature
            })
            .collect::<Vec<_>>();

        signatures.clone()
    }

    pub fn make_metadata(
        &self,
        origin_mailbox: Address,
        merkle_root: [u8; 32],
        message_id: [u8; 32],
        is_passed: bool,
    ) -> eyre::Result<MessageIdMultisigIsmMetadata> {
        let mut addr = [0u8; 32];
        addr[32 - origin_mailbox.0.len()..].copy_from_slice(&origin_mailbox.0);

        let multisig_hash = hpl_ism_multisig::multisig_hash(
            hpl_ism_multisig::domain_hash(self.domain, Binary(addr.to_vec()))?.to_vec(),
            merkle_root.to_vec(),
            0,
            message_id.to_vec(),
        )?;

        let hashed_message = hpl_ism_multisig::eth_hash(multisig_hash)?;

        let signatures = if is_passed {
            self.sign(self.threshold, hashed_message.as_slice().try_into()?)
        } else {
            self.sign(self.threshold - 1, hashed_message.as_slice().try_into()?)
        };

        Ok(MessageIdMultisigIsmMetadata {
            origin_mailbox: Binary::from(addr),
            merkle_root: Binary::from(merkle_root),
            signatures: signatures.iter().fold(Binary::default(), |acc, item| {
                Binary([acc.0, item.0.clone()].concat())
            }),
        })
    }
}
