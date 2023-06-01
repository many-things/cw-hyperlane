use std::fmt::Error;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, HexBinary};

const SIGNATURE_LENGTH: usize = 65;

#[cw_serde]
pub struct MerkleRootMultisigIsmMetadata {
    pub origin_mailbox: Binary, // bytes32
    pub checkpoint_index: u32,
    pub message_id: Binary, // bytes32
    pub proof: Binary,      // bytes32[32]
    pub signatures: Binary, // threshold * 65
}

impl From<MerkleRootMultisigIsmMetadata> for Binary {
    fn from(v: MerkleRootMultisigIsmMetadata) -> Self {
        v.origin_mailbox
            .0
            .iter()
            .chain(v.checkpoint_index.to_be_bytes().iter())
            .chain(v.message_id.0.iter())
            .chain(v.proof.0.iter())
            .chain(v.signatures.0.iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<MerkleRootMultisigIsmMetadata> for HexBinary {
    fn from(v: MerkleRootMultisigIsmMetadata) -> Self {
        Binary::from(v).into()
    }
}

impl From<Binary> for MerkleRootMultisigIsmMetadata {
    fn from(v: Binary) -> Self {
        Self {
            origin_mailbox: Binary(v[0..32].to_vec()),
            checkpoint_index: u32::from_be_bytes(v[32..36].try_into().unwrap()),
            message_id: Binary(v[36..68].to_vec()),
            proof: Binary(v[68..1092].to_vec()),
            signatures: Binary(v[1092..].to_vec()),
        }
    }
}

impl From<HexBinary> for MerkleRootMultisigIsmMetadata {
    fn from(v: HexBinary) -> Self {
        Binary(v.into()).into()
    }
}

impl MerkleRootMultisigIsmMetadata {
    pub fn signatures_len(&self) -> Result<usize, &'static str> {
        if self.signatures.len() % SIGNATURE_LENGTH != 0 {
            return Err("Invalid signatures length");
        }

        Ok(self.signatures.len() / SIGNATURE_LENGTH)
    }

    pub fn signature_at(&self, index: usize) -> Binary {
        Binary(self.signatures[index * SIGNATURE_LENGTH..(index + 1) * SIGNATURE_LENGTH].to_vec())
    }
}
