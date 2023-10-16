use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
#[derive(Default)]
pub struct Config {
    pub hrp: String,
    pub local_domain: u32,
    pub default_ism: Option<Addr>,
    pub default_hook: Option<Addr>,
    pub required_hook: Option<Addr>,
}

#[allow(dead_code)]
impl Config {
    pub fn new(hrp: impl Into<String>, local_domain: u32) -> Self {
        Self {
            hrp: hrp.into(),
            local_domain,
            ..Default::default()
        }
    }

    pub fn with_ism(mut self, default_ism: Addr) -> Self {
        self.default_ism = Some(default_ism);
        self
    }

    pub fn with_hook(mut self, default_hook: Addr, required_hook: Addr) -> Self {
        self.default_hook = Some(default_hook);
        self.required_hook = Some(required_hook);
        self
    }

    pub fn get_default_ism(&self) -> Addr {
        self.default_ism.clone().expect("default_ism not set")
    }

    pub fn get_default_hook(&self) -> Addr {
        self.default_hook.clone().expect("default_hook not set")
    }

    pub fn get_required_hook(&self) -> Addr {
        self.required_hook.clone().expect("required_hook not set")
    }
}

#[cw_serde]
pub struct Delivery {
    pub sender: Addr,
    pub block_number: u64,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const NONCE_KEY: &str = "nonce";
pub const NONCE: Item<u32> = Item::new(NONCE_KEY);

pub const LATEST_DISPATCHED_ID_KEY: &str = "latest_dispatched_id";
pub const LATEST_DISPATCHED_ID: Item<Vec<u8>> = Item::new(LATEST_DISPATCHED_ID_KEY);

pub const DELIVERIES_PREFIX: &str = "deliveries";
pub const DELIVERIES: Map<Vec<u8>, Delivery> = Map::new(DELIVERIES_PREFIX);
