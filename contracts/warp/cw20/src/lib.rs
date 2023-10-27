use cosmwasm_std::{Addr, Event};
use cw_storage_plus::Item;
use hpl_interface::warp::TokenMode;

pub mod contract;
mod conv;
pub mod error;

// reply message
pub const REPLY_ID_CREATE_DENOM: u64 = 0;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// storage definition for token denomination
const TOKEN_KEY: &str = "token";
const TOKEN: Item<Addr> = Item::new(TOKEN_KEY);

// storage definition for token mode
const MODE_KEY: &str = "mode";
const MODE: Item<TokenMode> = Item::new(MODE_KEY);

// storage definition for token hrp
const HRP_KEY: &str = "hrp";
const HRP: Item<String> = Item::new(HRP_KEY);

// storage definition for mailbox
const MAILBOX_KEY: &str = "mailbox";
const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);

const ISM_KEY: &str = "ism";
const ISM: Item<Addr> = Item::new(ISM_KEY);

const HOOK_KEY: &str = "hook";
const HOOK: Item<Addr> = Item::new(HOOK_KEY);

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_warp_cw20::{name}"))
}
