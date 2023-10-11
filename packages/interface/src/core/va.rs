use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, HexBinary};

#[cw_serde]
pub struct InstantiateMsg {
    pub addr_prefix: String,
    pub mailbox: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Announce {
        validator: HexBinary,
        storage_location: String,
        signature: Binary,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAnnounceStorageLocationsResponse)]
    GetAnnounceStorageLocations { validators: Vec<HexBinary> },

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
