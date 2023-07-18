#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, to_binary, Binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, Order,
    QueryResponse, Response, StdResult,
};

use hpl_interface::{
    types::{bech32_decode, keccak256_hash},
    va::{
        ExecuteMsg, GetAnnounceStorageLocationsResponse, GetAnnouncedValidatorsResponse,
        InstantiateMsg, MigrateMsg, QueryMsg,
    },
};

use crate::{
    error::ContractError,
    pub_to_addr,
    state::{
        ADDR_PREFIX, LOCAL_DOMAIN, MAILBOX, REPLAY_PROTECITONS, STORAGE_LOCATIONS, VALIDATORS,
    },
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADDR_PREFIX.save(deps.storage, &msg.addr_prefix)?;
    MAILBOX.save(deps.storage, &deps.api.addr_validate(&msg.mailbox)?)?;
    LOCAL_DOMAIN.save(deps.storage, &msg.local_domain)?;

    Ok(Response::new().add_event(
        Event::new("init-validator-announce")
            .add_attribute("creator", info.sender)
            .add_attribute("mailbox", msg.mailbox)
            .add_attribute("local-domain", msg.local_domain.to_string()),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Announce {
            validator,
            storage_location,
            signature,
        } => {
            let validator = deps.api.addr_validate(&validator)?;

            let replay_id = keccak256_hash(
                vec![
                    bech32_decode(validator.as_str())?,
                    storage_location.as_bytes().to_vec(),
                ]
                .concat()
                .as_slice(),
            );

            // check replay protection
            ensure!(
                REPLAY_PROTECITONS.has(deps.storage, replay_id.0.clone()),
                ContractError::Unauthorized {}
            );
            REPLAY_PROTECITONS.save(deps.storage, replay_id.0, &Empty {})?;

            // make announcement digest
            let domain_hash = keccak256_hash(
                vec![
                    LOCAL_DOMAIN.load(deps.storage)?.to_be_bytes().to_vec(),
                    bech32_decode(MAILBOX.load(deps.storage)?.as_str())?,
                    "HYPERLANE_ANNOUNCEMENT".as_bytes().to_vec(),
                ]
                .concat()
                .as_slice(),
            );

            let announcement_prehash = keccak256_hash(
                vec![domain_hash.0, storage_location.as_bytes().to_vec()]
                    .concat()
                    .as_slice(),
            );

            let announcement_digest = keccak256_hash(
                vec![
                    "\x19Ethereum Signed Message:\n".as_bytes().to_vec(),
                    announcement_prehash.0,
                ]
                .concat()
                .as_slice(),
            );

            // recover pubkey from signature & verify
            let recovered = deps.api.secp256k1_recover_pubkey(
                &announcement_digest,
                &signature.as_slice()[..64],
                signature[65],
            )?;

            let recovered_addr = pub_to_addr(Binary(recovered), &ADDR_PREFIX.load(deps.storage)?)?;
            ensure_eq!(recovered_addr, validator, ContractError::Unauthorized {});

            // save validator if not saved yet
            if !VALIDATORS.has(deps.storage, validator.clone()) {
                VALIDATORS.save(deps.storage, validator.clone(), &Empty {})?;
            }

            // append storage_locations
            let mut storage_locations = STORAGE_LOCATIONS
                .may_load(deps.storage, validator.clone())?
                .unwrap_or_default();
            storage_locations.push(storage_location.clone());
            STORAGE_LOCATIONS.save(deps.storage, validator.clone(), &storage_locations)?;

            Ok(Response::new().add_event(
                Event::new("validator-announcement")
                    .add_attribute("sender", info.sender)
                    .add_attribute("validator", validator)
                    .add_attribute("storage-location", storage_location),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::GetAnnounceStorageLocations { validators } => {
            let storage_locations = validators
                .into_iter()
                .map(|v| {
                    let validator = deps.api.addr_validate(&v)?;
                    let storage_locations = STORAGE_LOCATIONS
                        .may_load(deps.storage, validator.clone())?
                        .unwrap_or_default();
                    Ok((validator.to_string(), storage_locations))
                })
                .collect::<StdResult<Vec<_>>>()?;

            Ok(to_binary(&GetAnnounceStorageLocationsResponse {
                storage_locations,
            })?)
        }
        QueryMsg::GetAnnouncedValidators {} => {
            let validators = VALIDATORS
                .keys(deps.storage, None, None, Order::Ascending)
                .map(|v| v.map(String::from))
                .collect::<StdResult<Vec<_>>>()?;

            Ok(to_binary(&GetAnnouncedValidatorsResponse { validators })?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
