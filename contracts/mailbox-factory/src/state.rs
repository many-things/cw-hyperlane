use cw_storage_plus::Item;

pub const ORIGIN_DOMAIN_KEY: &str = "origin_domain";
pub const ORIGIN_DOMAIN: Item<u32> = Item::new(ORIGIN_DOMAIN_KEY);

pub const MAILBOX_CODE_KEY: &str = "mailbox_code";
pub const MAILBOX_CODE: Item<u64> = Item::new(MAILBOX_CODE_KEY);
