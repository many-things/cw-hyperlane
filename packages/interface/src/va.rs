use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {
    pub hrp: String,
    pub mailbox: String,
    pub local_domain: u32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Announce {
        validator: String,
        storage_location: String,
        signature: Binary,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAnnounceStorageLocationsResponse)]
    GetAnnounceStorageLocations { validators: Vec<String> },

    #[returns(GetAnnouncedValidatorsResponse)]
    GetAnnouncedValidators {},
}

#[cw_serde]
pub struct GetAnnounceStorageLocationsResponse {
    pub storage_locations: Vec<(String, Vec<String>)>,
}

#[cw_serde]
pub struct GetAnnouncedValidatorsResponse {
    pub validators: Vec<String>,
}

#[cw_serde]
pub struct MigrateMsg {}
