use bech32::FromBase32;
use cosmwasm_std::{Addr, Binary};

pub fn bech32_decode(addr: Addr) -> Binary {
    let (_hrp, data, _variant) = bech32::decode(addr.as_str()).unwrap();

    Vec::<u8>::from_base32(&data).unwrap().into()
}
