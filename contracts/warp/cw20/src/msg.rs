use cosmwasm_schema::cw_serde;
use hpl_interface::token::TokenMode;

#[cw_serde]
pub enum TokenOption {
    Create {
        code_id: u64,
        init_msg: Box<cw20_base::msg::InstantiateMsg>,
    },
    Reuse {
        contract: String,
    },
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token: Option<TokenOption>,
    pub mode: TokenMode,

    pub hrp: String,
    pub owner: String,
    pub mailbox: String,
}

#[cw_serde]
pub struct MigrateMsg {}
