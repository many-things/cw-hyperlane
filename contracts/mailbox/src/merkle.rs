use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, HexBinary, StdResult};
use hpl_interface::types::keccak256_hash;

use crate::state::assert_full_merkle_tree;

pub const HASH_LENGTH: usize = 32;
pub const TREE_DEPTH: usize = 32;
pub const MAX_LEAVES: u32 = (2_u32.pow(TREE_DEPTH as u32)) - 1;

pub const ZERO_BYTES: &str = "0000000000000000000000000000000000000000000000000000000000000000";
pub const ZERO_HASHES: [&str; HASH_LENGTH] = [
    "0000000000000000000000000000000000000000000000000000000000000000",
    "ad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5",
    "b4c11951957c6f8f642c4af61cd6b24640fec6dc7fc607ee8206a99e92410d30",
    "21ddb9a356815c3fac1026b6dec5df3124afbadb485c9ba5a3e3398a04b7ba85",
    "e58769b32a1beaf1ea27375a44095a0d1fb664ce2dd358e7fcbfb78c26a19344",
    "0eb01ebfc9ed27500cd4dfc979272d1f0913cc9f66540d7e8005811109e1cf2d",
    "887c22bd8750d34016ac3c66b5ff102dacdd73f6b014e710b51e8022af9a1968",
    "ffd70157e48063fc33c97a050f7f640233bf646cc98d9524c6b92bcf3ab56f83",
    "9867cc5f7f196b93bae1e27e6320742445d290f2263827498b54fec539f756af",
    "cefad4e508c098b9a7e1d8feb19955fb02ba9675585078710969d3440f5054e0",
    "f9dc3e7fe016e050eff260334f18a5d4fe391d82092319f5964f2e2eb7c1c3a5",
    "f8b13a49e282f609c317a833fb8d976d11517c571d1221a265d25af778ecf892",
    "3490c6ceeb450aecdc82e28293031d10c7d73bf85e57bf041a97360aa2c5d99c",
    "c1df82d9c4b87413eae2ef048f94b4d3554cea73d92b0f7af96e0271c691e2bb",
    "5c67add7c6caf302256adedf7ab114da0acfe870d449a3a489f781d659e8becc",
    "da7bce9f4e8618b6bd2f4132ce798cdc7a60e7e1460a7299e3c6342a579626d2",
    "2733e50f526ec2fa19a22b31e8ed50f23cd1fdf94c9154ed3a7609a2f1ff981f",
    "e1d3b5c807b281e4683cc6d6315cf95b9ade8641defcb32372f1c126e398ef7a",
    "5a2dce0a8a7f68bb74560f8f71837c2c2ebbcbf7fffb42ae1896f13f7c7479a0",
    "b46a28b6f55540f89444f63de0378e3d121be09e06cc9ded1c20e65876d36aa0",
    "c65e9645644786b620e2dd2ad648ddfcbf4a7e5b1a3a4ecfe7f64667a3f0b7e2",
    "f4418588ed35a2458cffeb39b93d26f18d2ab13bdce6aee58e7b99359ec2dfd9",
    "5a9c16dc00d6ef18b7933a6f8dc65ccb55667138776f7dea101070dc8796e377",
    "4df84f40ae0c8229d0d6069e5c8f39a7c299677a09d367fc7b05e3bc380ee652",
    "cdc72595f74c7b1043d0e1ffbab734648c838dfb0527d971b602bc216c9619ef",
    "0abf5ac974a1ed57f4050aa510dd9c74f508277b39d7973bb2dfccc5eeb0618d",
    "b8cd74046ff337f0a7bf2c8e03e10f642c1886798d71806ab1e888d9e5ee87d0",
    "838c5655cb21c6cb83313b5a631175dff4963772cce9108188b34ac87c81c41e",
    "662ee4dd2dd7b2bc707961b1e646c4047669dcb6584f0d8d770daf5d7e7deb2e",
    "388ab20e2573d171a88108e79d820e98f26c0b84aa8b2f4aa4968dbb818ea322",
    "93237c50ba75ee485f4c22adf2f741400bdf8d6a9cc7df7ecae576221665d735",
    "8448818bb4ae4562849e949e17ac16e0be16688e156b5cf15e098c627c0056a9",
];

#[cw_serde]
pub struct MerkleTree {
    pub branch: [Binary; TREE_DEPTH],
    pub count: u32,
}

impl MerkleTree {
    pub fn insert(&mut self, node: Binary) {
        assert_full_merkle_tree(self.count, MAX_LEAVES).unwrap();

        self.count += 1;

        let mut node = node;
        let mut size = self.count;
        for (i, next) in self.branch.iter().enumerate() {
            if (size & 1) == 1 {
                self.branch[i] = node;
                return;
            }
            node = keccak256_hash(&[next.clone().0, node.0].concat());
            size /= 2;
        }
        panic!("unreachable code")
    }

    pub fn root_with_ctx(&self, zeroes: [[u8; HASH_LENGTH]; TREE_DEPTH]) -> StdResult<Binary> {
        let idx = self.count;

        Ok(zeroes
            .iter()
            .enumerate()
            .fold(MerkleTree::zero()?.into(), |current, (i, zero)| {
                let ith_bit = (idx >> i) & 1;
                let next = self.branch[i].clone();
                if ith_bit == 1 {
                    keccak256_hash(&[next.0, current.0].concat())
                } else {
                    keccak256_hash(&[current.0, zero.to_vec()].concat())
                }
            }))
    }

    pub fn root(&self) -> StdResult<Binary> {
        self.root_with_ctx(MerkleTree::zeroes()?)
    }

    pub fn branch_root(item: Binary, branch: [Binary; TREE_DEPTH], idx: u128) -> Binary {
        branch
            .into_iter()
            .enumerate()
            .fold(item, |current, (i, next)| match (idx >> i) & 1 {
                1 => keccak256_hash(&[next.0, current.0].concat()),
                _ => keccak256_hash(&[current.0, next.0].concat()),
            })
    }

    pub fn zero() -> StdResult<[u8; HASH_LENGTH]> {
        Ok(HexBinary::from_hex(ZERO_BYTES)?
            .to_vec()
            .try_into()
            .expect("invalid length"))
    }

    pub fn zeroes() -> StdResult<[[u8; HASH_LENGTH]; TREE_DEPTH]> {
        Ok(ZERO_HASHES
            .into_iter()
            .map(|v| {
                Ok(HexBinary::from_hex(v)?
                    .to_vec()
                    .try_into()
                    .expect("invalid hash"))
            })
            .collect::<StdResult<Vec<_>>>()?
            .try_into()
            .expect("invalid depth"))
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Binary, HexBinary};
    use hpl_interface::types::keccak256_hash;

    #[test]
    fn test_compatibility() {
        let digest = Binary(
            [
                keccak256_hash("hello_world".as_bytes()).0,
                keccak256_hash("world_hello".as_bytes()).0,
            ]
            .concat(),
        );

        assert_eq!(
            format!("0x{}", HexBinary::from(digest).to_hex()),
            // abi.encodePacked(bytes32(keccak256("hello_world")), bytes32(keccak256("world_hello")));
            "0x5b07e077a81ffc6b47435f65a8727bcc542bc6fc0f25a56210efb1a74b88a5ae5e3b3917b0a11fc9edfc594b3aabbc95167d176fcc17aa76c01d7bda956862cd",
        );
    }
}
