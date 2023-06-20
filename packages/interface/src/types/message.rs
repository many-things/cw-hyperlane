use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api, Binary, CanonicalAddr, HexBinary, StdResult};

#[cw_serde]
pub struct Message {
    pub version: u8,
    pub nonce: u32,
    pub origin_domain: u32,
    pub sender: Binary,
    pub dest_domain: u32,
    pub recipient: Binary,
    pub body: Binary,
}

impl From<Message> for Binary {
    fn from(v: Message) -> Self {
        v.version
            .to_be_bytes()
            .iter()
            .chain(v.nonce.to_be_bytes().iter())
            .chain(v.origin_domain.to_be_bytes().iter())
            .chain(v.sender.0.iter())
            .chain(v.dest_domain.to_be_bytes().iter())
            .chain(v.recipient.0.iter())
            .chain(v.body.0.iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<Message> for HexBinary {
    fn from(v: Message) -> Self {
        Binary::from(v).into()
    }
}

impl From<Binary> for Message {
    fn from(v: Binary) -> Self {
        Self {
            version: v[0],
            nonce: u32::from_be_bytes(v[1..5].try_into().unwrap()),
            origin_domain: u32::from_be_bytes(v[5..9].try_into().unwrap()),
            sender: Binary(v[9..41].to_vec()),
            dest_domain: u32::from_be_bytes(v[41..45].try_into().unwrap()),
            recipient: Binary(v[45..77].to_vec()),
            body: Binary(v[77..].to_vec()),
        }
    }
}

impl From<HexBinary> for Message {
    fn from(v: HexBinary) -> Self {
        Binary(v.into()).into()
    }
}

impl Message {
    pub fn id(&self) -> Binary {
        super::keccak256_hash(&Binary::from(self.clone()))
    }

    pub fn recipient_addr(&self, api: &dyn Api) -> StdResult<Addr> {
        let addr: CanonicalAddr = self.recipient.clone().into();
        api.addr_humanize(&addr)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Binary, HexBinary};

    use super::Message;

    #[test]
    fn test_encode_decode() {
        let encode_expected: HexBinary = HexBinary::from_hex("00000021500000aef3000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f1600aa36a70000000000000000000000005d56b8a669f50193b54319442c6eee5edd66238148656c6c6f21").unwrap();

        let hex = |v: &str| -> Binary { HexBinary::from_hex(v).unwrap().into() };

        let decode_actual: Message = Binary(encode_expected.clone().into()).into();
        let decode_expected = Message {
            version: 0,
            nonce: 8528,
            origin_domain: 44787,
            sender: hex("000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f16"),
            dest_domain: 11155111,
            recipient: hex("0000000000000000000000005d56b8a669f50193b54319442c6eee5edd662381"),
            body: hex("48656c6c6f21"),
        };
        let encode_actual: Binary = decode_expected.clone().into();

        assert_eq!(decode_expected, decode_actual);
        assert_eq!(encode_expected, HexBinary::from(encode_actual));
    }
}
