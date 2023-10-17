use std::collections::BTreeMap;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct CodesMap(pub BTreeMap<String, u64>);

impl FromIterator<(String, u64)> for CodesMap {
    fn from_iter<T: IntoIterator<Item = (String, u64)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cw_serde]
pub struct Codes {
    pub mailbox: u64,
    #[serde(rename = "validator_announce")]
    pub va: u64,

    pub hook_merkle: u64,
    pub hook_pausable: u64,
    pub hook_routing: u64,
    pub hook_routing_custom: u64,
    pub hook_routing_fallback: u64,

    pub igp: u64,
    pub igp_oracle: u64,

    pub ism_multisig: u64,
    pub ism_routing: u64,

    pub test_mock_hook: u64,
    pub test_mock_ism: u64,
    pub test_mock_msg_receiver: u64,

    pub warp_cw20: u64,
    pub warp_native: u64,
}

impl TryFrom<CodesMap> for Codes {
    type Error = eyre::Error;

    fn try_from(v: CodesMap) -> Result<Self, Self::Error> {
        let bin = serde_json::to_vec(&v)?;

        let ret = serde_json::from_slice(&bin)?;

        Ok(ret)
    }
}

#[cw_serde]
pub struct CoreDeployments {
    pub mailbox: String,
    pub igp: String,
    pub igp_oracle: String,
    pub default_ism: String,
    pub default_hook: String,
    pub required_hook: String,
    pub msg_receiver: String,
}
