#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, to_binary, Binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, Order, QueryResponse,
    Response, StdResult,
};

use hpl_interface::{
    types::{bech32_decode, bech32_encode, keccak256_hash},
    va::{
        ExecuteMsg, GetAnnounceStorageLocationsResponse, GetAnnouncedValidatorsResponse,
        InstantiateMsg, MigrateMsg, QueryMsg,
    },
};
use k256::{ecdsa::VerifyingKey, EncodedPoint};

use crate::{
    error::ContractError,
    eth_hash, pub_to_addr,
    state::{
        ADDR_PREFIX, LOCAL_DOMAIN, MAILBOX, REPLAY_PROTECITONS, STORAGE_LOCATIONS, VALIDATORS,
    },
    CONTRACT_NAME, CONTRACT_VERSION,
};

pub fn domain_hash(local_domain: u32, mailbox: &str) -> StdResult<Binary> {
    let mut bz = vec![];
    bz.append(&mut local_domain.to_be_bytes().to_vec());
    bz.append(&mut bech32_decode(mailbox)?);
    bz.append(&mut "HYPERLANE_ANNOUNCEMENT".as_bytes().to_vec());

    let hash = keccak256_hash(&bz);

    Ok(hash)
}

pub fn announcement_hash(mut domain_hash: Vec<u8>, storage_location: &str) -> Binary {
    let mut bz = vec![];
    bz.append(&mut domain_hash);
    bz.append(&mut storage_location.as_bytes().to_vec());

    keccak256_hash(&bz)
}

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
            let addr_prefix = ADDR_PREFIX.load(deps.storage)?;
            let raw_validator = bech32_encode(addr_prefix.as_str(), &validator)?;
            let validator = deps.api.addr_validate(raw_validator.as_str())?;

            let replay_id = keccak256_hash(
                [
                    bech32_decode(validator.as_str())?,
                    storage_location.as_bytes().to_vec(),
                ]
                .concat()
                .as_slice(),
            );

            // check replay protection
            ensure!(
                !REPLAY_PROTECITONS.has(deps.storage, replay_id.0.clone()),
                ContractError::Unauthorized {}
            );
            REPLAY_PROTECITONS.save(deps.storage, replay_id.0, &Empty {})?;

            // make announcement digest
            let local_domain = LOCAL_DOMAIN.load(deps.storage)?;
            let mailbox = MAILBOX.load(deps.storage)?;

            // make digest
            let message_hash = eth_hash(announcement_hash(
                domain_hash(local_domain, mailbox.as_str())?.0,
                &storage_location,
            ))?;

            // recover pubkey from signature & verify
            let recovered = deps.api.secp256k1_recover_pubkey(
                &message_hash,
                &signature.as_slice()[..64],
                // We subs 27 according to this - https://eips.ethereum.org/EIPS/eip-155
                signature[64] - 27,
            )?;

            let public_key =
                VerifyingKey::from_encoded_point(&EncodedPoint::from_bytes(recovered).unwrap())
                    .expect("invalid recovered public key");
            let public_key_compressed = public_key.to_encoded_point(true).as_bytes().to_vec();

            let recovered_addr = pub_to_addr(
                Binary(public_key_compressed),
                &ADDR_PREFIX.load(deps.storage)?,
            )?;
            ensure!(recovered_addr == validator, ContractError::VerifyFailed {});

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
                    let addr_prefix = ADDR_PREFIX.load(deps.storage)?;
                    let raw_validator = bech32_encode(addr_prefix.as_str(), &v)?;
                    let validator = deps.api.addr_validate(raw_validator.as_str())?;

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
