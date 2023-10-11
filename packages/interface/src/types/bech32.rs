use bech32::{FromBase32, ToBase32};
use cosmwasm_std::{Addr, StdError, StdResult};

pub fn bech32_to_h256(target: &str) -> StdResult<[u8; 32]> {
    let raw_addr = bech32_decode(target)?;

    let mut addr = [0u8; 32];
    addr[32 - raw_addr.len()..].copy_from_slice(&raw_addr);

    Ok(addr)
}

pub fn bech32_decode(target: &str) -> StdResult<Vec<u8>> {
    let (_, raw_addr_u5, _) = bech32::decode(target)
        .map_err(|e| StdError::generic_err(format!("invalid bech32 bytes. err: {e}")))?;

    let raw_addr = Vec::<u8>::from_base32(&raw_addr_u5)
        .map_err(|e| StdError::generic_err(format!("failed to parse [u5] to [u8]. err: {e}")))?;

    Ok(raw_addr)
}

pub fn bech32_encode(hrp: &str, raw_addr: &[u8]) -> StdResult<Addr> {
    if raw_addr.len() != 32 && raw_addr.len() != 20 {
        return Err(StdError::generic_err(format!(
            "invalid raw address length. expected: 32 or 20. got: {}",
            raw_addr.len()
        )));
    }

    if raw_addr.len() == 32 {
        let mut bz = 0u128.to_be_bytes();
        bz[4..].copy_from_slice(&raw_addr[0..12]);
        let check = u128::from_be_bytes(bz);

        if check == 0 {
            return bech32_encode(hrp, &raw_addr[12..]);
        }
    }

    let enc_addr = bech32::encode(hrp, raw_addr.to_base32(), bech32::Variant::Bech32)
        .map_err(|e| StdError::generic_err(format!("invalid bech32 address. err: {e}")))?;

    Ok(Addr::unchecked(enc_addr))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::types::bech32_to_h256;

    use super::{bech32_decode, bech32_encode};

    #[rstest]
    #[case(
        "osmo",
        "osmo1466nf3zuxpya8q9emxukd7vftaf6h4psr0a07srl5zw74zh84yjqkk0zfx"
    )]
    #[case("osmo", "osmo14n3a65fnqz9jve85l23al6m3pjugf0atvrfqh5")]
    #[case(
        "neutron",
        "neutron1gajw625kz8el4ayk8fwpy7r6ew0m7zrg9jdd6grg85fle39shuxqezuz2c"
    )]
    #[case("neutron", "neutron1xle8l3h0wkcp6tsxmkc6n4vqyfkhwnukevwwsk")]
    fn test_decode_encode(#[case] hrp: &str, #[case] src_addr: &str) {
        let dec_addr = bech32_to_h256(src_addr).unwrap();
        let enc_addr = bech32_encode(hrp, &dec_addr).unwrap();

        assert_eq!(src_addr, enc_addr.as_str());
    }

    #[rstest]
    #[case("osmo", "osmo1d6a3j0kkpc8eac0j8h6ypyevfz8hd3qnuqjrc5")]
    #[case("neutron", "neutron1d6a3j0kkpc8eac0j8h6ypyevfz8hd3qnsyg35p")]
    fn test_decode_encode_acc(#[case] hrp: &str, #[case] src_addr: &str) {
        let dec_addr = bech32_decode(src_addr).unwrap();
        let enc_addr = bech32_encode(hrp, &dec_addr).unwrap();

        assert_eq!(src_addr, enc_addr.as_str());
    }

    #[rstest]
    #[case(&[
        "cosmos1d6a3j0kkpc8eac0j8h6ypyevfz8hd3qn5mpnwx",
        "axelar1d6a3j0kkpc8eac0j8h6ypyevfz8hd3qns4hm98",
        "osmo1d6a3j0kkpc8eac0j8h6ypyevfz8hd3qnuqjrc5",
        "neutron1d6a3j0kkpc8eac0j8h6ypyevfz8hd3qnsyg35p"
    ])]
    fn test_acc_cross(#[case] addrs: &[&str]) {
        for (i, test_target_addr) in addrs.iter().enumerate() {
            let mut test_addr_group = addrs.to_vec();
            test_addr_group.remove(i);

            let test_target_addr_bin = bech32_decode(test_target_addr).unwrap();

            for test_addr in test_addr_group {
                let hrp = test_addr.split('1').collect::<Vec<_>>()[0];

                let encoded = bech32_encode(hrp, &test_target_addr_bin).unwrap();
                assert_eq!(test_addr, encoded.as_str());
            }
        }
    }
}
