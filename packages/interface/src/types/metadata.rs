use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, HexBinary, Uint256};

use super::bech32_encode;

const SIGNATURE_LENGTH: usize = 65;

#[cw_serde]
pub struct MerkleRootMultisigIsmMetadata {
    pub origin_mailbox: Binary,
    // bytes32
    pub checkpoint_index: u32,
    pub message_id: Binary,
    // bytes32
    pub proof: Binary,
    // bytes32[32]
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
        // FIXME: handle index out of length
        Binary(self.signatures[index * SIGNATURE_LENGTH..(index + 1) * SIGNATURE_LENGTH].to_vec())
    }
}

#[cw_serde]
pub struct MessageIdMultisigIsmMetadata {
    pub origin_mailbox: Binary,
    // byte32
    pub merkle_root: Binary,
    //bytes32
    pub signatures: Binary,     // 65 * length
}

impl From<MessageIdMultisigIsmMetadata> for Binary {
    fn from(v: MessageIdMultisigIsmMetadata) -> Self {
        v.origin_mailbox
            .0
            .iter()
            .chain(v.merkle_root.0.iter())
            .chain(v.signatures.0.iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<MessageIdMultisigIsmMetadata> for HexBinary {
    fn from(v: MessageIdMultisigIsmMetadata) -> Self {
        Binary::from(v).into()
    }
}

impl From<Binary> for MessageIdMultisigIsmMetadata {
    fn from(v: Binary) -> Self {
        Self {
            origin_mailbox: Binary(v[0..32].to_vec()),
            merkle_root: Binary(v[32..64].to_vec()),
            signatures: Binary(v[64..].to_vec()),
        }
    }
}

impl From<HexBinary> for MessageIdMultisigIsmMetadata {
    fn from(v: HexBinary) -> Self {
        Binary(v.into()).into()
    }
}

impl MessageIdMultisigIsmMetadata {
    pub fn signatures_len(&self) -> Result<usize, &'static str> {
        if self.signatures.len() % SIGNATURE_LENGTH != 0 {
            return Err("Invalid signatures length");
        }

        Ok(self.signatures.len() / SIGNATURE_LENGTH)
    }

    pub fn signature_at(&self, index: usize) -> Binary {
        // FIXME: handle index out of length
        Binary(self.signatures[index * SIGNATURE_LENGTH..(index + 1) * SIGNATURE_LENGTH].to_vec())
    }
}

#[cw_serde]
pub struct IGPMetadata {
    pub gas_limit: Uint256,
    pub refund_address: Binary,
}

impl From<IGPMetadata> for Binary {
    fn from(v: IGPMetadata) -> Self {
        v.gas_limit
            .to_be_bytes()
            .iter()
            .chain(v.refund_address.0.iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<IGPMetadata> for HexBinary {
    fn from(v: IGPMetadata) -> Self {
        Binary::from(v).into()
    }
}

impl From<Binary> for IGPMetadata {
    fn from(v: Binary) -> Self {
        Self {
            gas_limit: Uint256::from_be_bytes(v[0..32].try_into().unwrap()),
            refund_address: Binary(v[32..].to_vec()),
        }
    }
}

impl From<HexBinary> for IGPMetadata {
    fn from(v: HexBinary) -> Self {
        Binary(v.into()).into()
    }
}

impl IGPMetadata {
    pub fn get_refund_address(&self, hrp: String, default: Addr) -> Addr {
        if self.refund_address.0.len() != 20 && self.refund_address.0.len() != 32 {
            return default;
        }

        let raw_addr = match self.refund_address.0.iter().take(16).all(|&byte| byte == 0) {
            true => self.refund_address.0[16..].to_vec(),
            false => self.refund_address.0.clone(),
        };

        bech32_encode(&hrp, &raw_addr).unwrap()
    }
}
