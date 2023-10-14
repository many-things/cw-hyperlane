use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary, StdResult};

use super::bech32_encode;

#[cw_serde]
pub struct Message {
    pub version: u8,
    pub nonce: u32,
    pub origin_domain: u32,
    pub sender: HexBinary,
    pub dest_domain: u32,
    pub recipient: HexBinary,
    pub body: HexBinary,
}

impl Message {
    pub fn id(&self) -> HexBinary {
        super::keccak256_hash(&HexBinary::from(self.clone()))
    }

    pub fn sender_addr(&self, hrp: &str) -> StdResult<Addr> {
        bech32_encode(hrp, &self.sender)
    }
    pub fn recipient_addr(&self, hrp: &str) -> StdResult<Addr> {
        bech32_encode(hrp, &self.recipient)
    }
}

impl From<Message> for HexBinary {
    fn from(v: Message) -> Self {
        v.version
            .to_be_bytes()
            .iter()
            .chain(v.nonce.to_be_bytes().iter())
            .chain(v.origin_domain.to_be_bytes().iter())
            .chain(v.sender.to_vec().iter())
            .chain(v.dest_domain.to_be_bytes().iter())
            .chain(v.recipient.to_vec().iter())
            .chain(v.body.to_vec().iter())
            .cloned()
            .collect::<Vec<u8>>()
            .into()
    }
}

impl From<HexBinary> for Message {
    fn from(v: HexBinary) -> Self {
        Self {
            version: v[0],
            nonce: u32::from_be_bytes(v[1..5].try_into().unwrap()),
            origin_domain: u32::from_be_bytes(v[5..9].try_into().unwrap()),
            sender: v[9..41].to_vec().into(),
            dest_domain: u32::from_be_bytes(v[41..45].try_into().unwrap()),
            recipient: v[45..77].to_vec().into(),
            body: v[77..].to_vec().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::HexBinary;

    use super::Message;

    #[test]
    fn test_encode_decode() {
        let encode_expected = HexBinary::from_hex("00000021500000aef3000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f1600aa36a70000000000000000000000005d56b8a669f50193b54319442c6eee5edd66238148656c6c6f21").unwrap();

        let hex = |v: &str| -> HexBinary { HexBinary::from_hex(v).unwrap() };

        let decode_actual: Message = encode_expected.clone().into();
        let decode_expected = Message {
            version: 0,
            nonce: 8528,
            origin_domain: 44787,
            sender: hex("000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f16"),
            dest_domain: 11155111,
            recipient: hex("0000000000000000000000005d56b8a669f50193b54319442c6eee5edd662381"),
            body: hex("48656c6c6f21"),
        };
        let encode_actual: HexBinary = decode_expected.clone().into();

        assert_eq!(decode_expected, decode_actual);
        assert_eq!(encode_expected, encode_actual);
    }

    #[test]
    #[should_panic(expected = "range end index 77 out of range for slice of length 67")]
    fn test_overflow() {
        let no = HexBinary::from_hex("00000021500000aef3000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f1600aa36a70000000000000000000000005d56b8a669f50193b543").unwrap();

        let _msg: Message = no.into();
    }
}
