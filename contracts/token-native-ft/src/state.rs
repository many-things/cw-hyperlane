use cw_storage_plus::Item;
use hpl_interface::token::TokenMode;

pub const TOKEN_KEY: &str = "token";
pub const TOKEN: Item<String> = Item::new(TOKEN_KEY);

pub const MODE_KEY: &str = "mode";
pub const MODE: Item<TokenMode> = Item::new(MODE_KEY);
