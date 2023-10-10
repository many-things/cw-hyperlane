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
    pub default_hook: u64,
    pub domain_routing_hook: u64,
    pub hub: u64,
    pub igp_core: u64,
    pub igp_gas_oracle: u64,
    pub ism_multisig: u64,
    pub ism_routing: u64,
    pub mailbox: u64,
    pub test_mock_hook: u64,
    pub test_mock_ism: u64,
    pub test_mock_msg_receiver: u64,
    pub multicall: u64,
    pub token_cw20: u64,
    pub token_native: u64,
    #[serde(rename = "validator_announce")]
    pub va: u64,
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
    pub hub: String,
    pub ism: String,
    pub hook: String,
    pub mailbox: String,
    pub msg_receiver: String,
}
