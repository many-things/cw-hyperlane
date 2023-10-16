use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub isms: Vec<String>,
    pub threshold: u8,
}
