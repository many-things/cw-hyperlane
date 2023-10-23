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
    pub origin_merkle_tree: HexBinary,

    pub merkle_root: HexBinary,

    pub merkle_index: HexBinary,

    pub signatures: Vec<HexBinary>,
}

impl From<MessageIdMultisigIsmMetadata> for HexBinary {
    fn from(v: MessageIdMultisigIsmMetadata) -> Self {
        let mut origin_merkle_tree = [0u8; 32];
        origin_merkle_tree[32 - v.origin_merkle_tree.len()..]
            .copy_from_slice(&v.origin_merkle_tree);

        origin_merkle_tree
            .to_vec()
            .iter()
            .chain(v.merkle_root.to_vec().iter())
            .chain(v.merkle_index.to_vec().iter())
            .chain(
                v.signatures
                    .iter()
                    .flat_map(|x| x.to_vec())
                    .collect::<Vec<_>>()
                    .iter(),
            )
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<HexBinary> for MessageIdMultisigIsmMetadata {
    fn from(v: HexBinary) -> Self {
        let signatures = v[68..]
            .to_vec()
            .chunks_exact(SIGNATURE_LENGTH)
            .map(|v| v.into())
            .collect::<Vec<HexBinary>>();

        Self {
            origin_merkle_tree: v[0..32].to_vec().into(),
            merkle_root: v[32..64].to_vec().into(),
            merkle_index: v[64..68].to_vec().into(),
            signatures,
        }
    }
}

impl MessageIdMultisigIsmMetadata {
    pub fn merkle_index(&self) -> u32 {
        u32::from_be_bytes(self.merkle_index.to_vec().try_into().unwrap())
    }
}

use std::convert::AsMut;

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

#[cw_serde]
pub struct AggregateMetadata(BTreeMap<Addr, HexBinary>);

impl AggregateMetadata {
    pub const RANGE_SIZE: usize = 4;

    pub fn new(set: Vec<(Addr, HexBinary)>) -> Self {
        Self(set.into_iter().collect())
    }
}

impl Iterator for AggregateMetadata {
    type Item = (Addr, HexBinary);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_first()
    }
}

impl AggregateMetadata {
    pub fn from_hex(v: HexBinary, addrs: Vec<Addr>) -> Self {
        Self(
            addrs
                .into_iter()
                .enumerate()
                .map(|(i, ism)| {
                    let start = i * Self::RANGE_SIZE * 2;
                    let mid = start + Self::RANGE_SIZE;
                    let end = mid + Self::RANGE_SIZE;

                    let mut meta_start = [0u8; 8];
                    meta_start[8 - (mid - start)..].copy_from_slice(&v[start..mid]);
                    let mut meta_end = [0u8; 8];
                    meta_end[8 - (end - mid)..].copy_from_slice(&v[mid..end]);

                    let meta_start = usize::from_be_bytes(meta_start);
                    let meta_end = usize::from_be_bytes(meta_end);

                    (ism, v[meta_start..meta_end].to_vec().into())
                })
                .collect(),
        )
    }
}

impl From<AggregateMetadata> for HexBinary {
    fn from(v: AggregateMetadata) -> Self {
        let pos_start = v.0.len() * AggregateMetadata::RANGE_SIZE * 2;

        let ls: Vec<(
            [u8; AggregateMetadata::RANGE_SIZE],
            [u8; AggregateMetadata::RANGE_SIZE],
            HexBinary,
        )> =
            v.0.values()
                .fold(vec![] as Vec<(usize, usize, HexBinary)>, |mut acc, m| {
                    let l = acc.last().map(|v| v.1).unwrap_or(pos_start);

                    acc.push((l, l + m.len(), m.clone()));
                    acc
                })
                .into_iter()
                .map(|(start, end, metadata)| {
                    (
                        clone_into_array(&start.to_be_bytes()[AggregateMetadata::RANGE_SIZE..]),
                        clone_into_array(&end.to_be_bytes()[AggregateMetadata::RANGE_SIZE..]),
                        metadata,
                    )
                })
                .collect();

        let mut pos = vec![];
        let mut metadata = vec![];

        for (start, end, meta) in ls {
            pos.extend_from_slice(&start);
            pos.extend_from_slice(&end);
            metadata.extend_from_slice(meta.as_slice());
        }

        [pos, metadata].concat().into()
    }
}

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

#[cfg(test)]
mod test {
    use ibcx_test_utils::{addr, gen_bz, hex};

    use super::*;

    #[test]
    fn test_aggregate() {
        let set = vec![
            (addr("test1"), gen_bz(12)),
            (addr("test2"), gen_bz(12)),
            (addr("test3"), gen_bz(12)),
        ];

        let metadata = AggregateMetadata::new(set);
        let isms = metadata.0.clone().into_keys().collect();

        let metadata_bz: HexBinary = metadata.clone().into();

        let new_metadata = AggregateMetadata::from_hex(metadata_bz, isms);
        assert_eq!(metadata, new_metadata);
    }

    #[test]
    fn test_message_id_multisig_metadata() {
        let testdata = hex("fadafdf4db5e6264d450bafa5951b2180b8fe8aac2e012f280784ae841e9a7f732a2601709a27a5e370a59f98a67b5da6baa522b6421edf2ea240d94d84511a800000000df4eaf1947af0858139b90054561d5ab2a423b4ad8d75a5ec7f9e860fd3de1bb3924e2593e29b595aae2717538c0af6d6ae9fc20477da49d223a0d928a1efb311bdf4eaf1947af0858139b90054561d5ab2a423b4ad8d75a5ec7f9e860fd3de1bb3924e2593e29b595aae2717538c0af6d6ae9fc20477da49d223a0d928a1efb311b");

        let metadata: MessageIdMultisigIsmMetadata = testdata.clone().into();

        assert_eq!(metadata.signatures.len(), 2);
        assert_eq!(
            metadata.signatures.iter().flat_map(|v| v.to_vec()).count(),
            SIGNATURE_LENGTH * 2
        );

        let recovered: HexBinary = metadata.into();

        assert_eq!(recovered, testdata);
    }
}
