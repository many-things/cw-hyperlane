use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary, Uint256};

use super::bech32_encode;

const SIGNATURE_LENGTH: usize = 65;

#[cw_serde]
pub struct MerkleRootMultisigIsmMetadata {
    pub origin_mailbox: HexBinary,
    // bytes32
    pub checkpoint_index: u32,
    pub message_id: HexBinary,
    // bytes32
    pub proof: HexBinary,
    // bytes32[32]
    pub signatures: HexBinary, // threshold * 65
}

impl From<MerkleRootMultisigIsmMetadata> for HexBinary {
    fn from(v: MerkleRootMultisigIsmMetadata) -> Self {
        v.origin_mailbox
            .to_vec()
            .iter()
            .chain(v.checkpoint_index.to_be_bytes().iter())
            .chain(v.message_id.to_vec().iter())
            .chain(v.proof.to_vec().iter())
            .chain(v.signatures.to_vec().iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<HexBinary> for MerkleRootMultisigIsmMetadata {
    fn from(v: HexBinary) -> Self {
        Self {
            origin_mailbox: v[0..32].to_vec().into(),
            checkpoint_index: u32::from_be_bytes(v[32..36].try_into().unwrap()),
            message_id: v[36..68].to_vec().into(),
            proof: v[68..1092].to_vec().into(),
            signatures: v[1092..].to_vec().into(),
        }
    }
}

impl MerkleRootMultisigIsmMetadata {
    pub fn signatures_len(&self) -> Result<usize, &'static str> {
        if self.signatures.len() % SIGNATURE_LENGTH != 0 {
            return Err("Invalid signatures length");
        }

        Ok(self.signatures.len() / SIGNATURE_LENGTH)
    }

    pub fn signature_at(&self, index: usize) -> HexBinary {
        // FIXME: handle index out of length
        self.signatures[index * SIGNATURE_LENGTH..(index + 1) * SIGNATURE_LENGTH]
            .to_vec()
            .into()
    }
}

#[cw_serde]
pub struct MessageIdMultisigIsmMetadata {
    pub origin_mailbox: HexBinary,
    // byte32
    pub merkle_root: HexBinary,
    //bytes32
    pub signatures: HexBinary, // 65 * length
}

impl From<MessageIdMultisigIsmMetadata> for HexBinary {
    fn from(v: MessageIdMultisigIsmMetadata) -> Self {
        v.origin_mailbox
            .to_vec()
            .iter()
            .chain(v.merkle_root.to_vec().iter())
            .chain(v.signatures.to_vec().iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<HexBinary> for MessageIdMultisigIsmMetadata {
    fn from(v: HexBinary) -> Self {
        Self {
            origin_mailbox: v[0..32].to_vec().into(),
            merkle_root: v[32..64].to_vec().into(),
            signatures: v[64..].to_vec().into(),
        }
    }
}

impl MessageIdMultisigIsmMetadata {
    pub fn signatures_len(&self) -> Result<usize, &'static str> {
        if self.signatures.len() % SIGNATURE_LENGTH != 0 {
            return Err("Invalid signatures length");
        }

        Ok(self.signatures.len() / SIGNATURE_LENGTH)
    }

    pub fn signature_at(&self, index: usize) -> HexBinary {
        // FIXME: handle index out of length
        self.signatures[index * SIGNATURE_LENGTH..(index + 1) * SIGNATURE_LENGTH]
            .to_vec()
            .into()
    }
}

#[cw_serde]
pub struct AggregateIsmMetadata(pub BTreeMap<Addr, HexBinary>);

impl AggregateIsmMetadata {
    const RANGE_SIZE: usize = 4;

    pub fn from_hex(v: HexBinary, isms: Vec<Addr>) -> Self {
        Self(
            isms.into_iter()
                .enumerate()
                .map(|(i, ism)| {
                    let start = i * Self::RANGE_SIZE * 2;
                    let mid = start + Self::RANGE_SIZE;
                    let end = mid + Self::RANGE_SIZE;

                    let meta_start = usize::from_be_bytes(v[start..mid].try_into().unwrap());
                    let meta_end = usize::from_be_bytes(v[mid..end].try_into().unwrap());

                    (ism, v[meta_start..meta_end].to_vec().into())
                })
                .collect(),
        )
    }
}

// impl From<AggregateIsmMetadata> for HexBinary {
//     fn from(v: AggregateIsmMetadata) -> Self {
//         v.0.into_iter().fold(vec![], |acc, (ism, metaedata)| {

//         })
//     }
// }

#[cw_serde]
pub struct IGPMetadata {
    pub gas_limit: Uint256,
    pub refund_address: HexBinary,
}

impl From<IGPMetadata> for HexBinary {
    fn from(v: IGPMetadata) -> Self {
        v.gas_limit
            .to_be_bytes()
            .iter()
            .chain(v.refund_address.to_vec().iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<HexBinary> for IGPMetadata {
    fn from(v: HexBinary) -> Self {
        Self {
            gas_limit: Uint256::from_be_bytes(v[0..32].try_into().unwrap()),
            refund_address: v[32..].to_vec().into(),
        }
    }
}

impl IGPMetadata {
    pub fn get_refund_address(&self, hrp: &str, default: Addr) -> Addr {
        if self.refund_address.to_vec().len() != 20 && self.refund_address.to_vec().len() != 32 {
            return default;
        }

        let raw_addr = match self
            .refund_address
            .to_vec()
            .iter()
            .take(16)
            .all(|&byte| byte == 0)
        {
            true => self.refund_address.to_vec()[16..].to_vec(),
            false => self.refund_address.to_vec(),
        };

        bech32_encode(hrp, &raw_addr).unwrap()
    }
}
