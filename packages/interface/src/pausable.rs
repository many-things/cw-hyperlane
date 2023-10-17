use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum PausableMsg {
    Pause {},
    Release {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum PausableQueryMsg {
    #[returns(PauseInfoResponse)]
    PauseInfo {},
}

#[cw_serde]
pub struct PauseInfoResponse {
    pub paused: bool,
}
