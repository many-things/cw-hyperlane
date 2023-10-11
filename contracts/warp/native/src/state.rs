use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use hpl_interface::token::TokenMode;

pub const TOKEN_KEY: &str = "token";
pub const TOKEN: Item<String> = Item::new(TOKEN_KEY);

pub const MODE_KEY: &str = "mode";
pub const MODE: Item<TokenMode> = Item::new(MODE_KEY);

pub const HRP_KEY: &str = "hrp";
pub const HRP: Item<String> = Item::new(HRP_KEY);

pub const OWNER_KEY: &str = "owner";
pub const OWNER: Item<Addr> = Item::new(OWNER_KEY);

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);
