#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, ensure_eq, Deps, DepsMut, Empty, Env, Event, HexBinary, MessageInfo, Order,
    QueryResponse, Response, StdResult,
};

use hpl_interface::{
    core::{
        mailbox::{self, MailboxQueryMsg},
        va::{
            ExecuteMsg, GetAnnounceStorageLocationsResponse, GetAnnouncedValidatorsResponse,
            InstantiateMsg, LocalDomainResponse, MailboxResponse, QueryMsg,
        },
    },
    to_binary,
    types::{bech32_decode, eth_addr, eth_hash, keccak256_hash},
};

use crate::{
    error::ContractError,
    state::{LOCAL_DOMAIN, MAILBOX, REPLAY_PROTECITONS, STORAGE_LOCATIONS, VALIDATORS},
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

    let mailbox = deps.api.addr_validate(&msg.mailbox)?;
    let mailbox_addr = bech32_decode(mailbox.as_str())?;

    let local_domain = deps
        .querier
        .query_wasm_smart::<mailbox::LocalDomainResponse>(
            &mailbox,
            &mailbox::QueryMsg::Mailbox(MailboxQueryMsg::LocalDomain {}),
        )?
        .local_domain;

    MAILBOX.save(deps.storage, &mailbox_addr)?;
    LOCAL_DOMAIN.save(deps.storage, &local_domain)?;

    Ok(Response::new().add_event(
        Event::new("init-validator-announce")
            .add_attribute("creator", info.sender)
            .add_attribute("mailbox", msg.mailbox)
            .add_attribute("local-domain", local_domain.to_string()),
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
        } => announce(deps, info, validator, storage_location, signature),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::GetAnnounceStorageLocations { validators } => {
            to_binary(get_announce(deps, validators))
        }
        QueryMsg::GetAnnouncedValidators {} => to_binary(get_validators(deps)),
        QueryMsg::Mailbox {} => to_binary(get_mailbox(deps)),
        QueryMsg::LocalDomain {} => to_binary(get_local_domain(deps)),
    }
}

fn get_announce(
    deps: Deps,
    validators: Vec<HexBinary>,
) -> Result<GetAnnounceStorageLocationsResponse, ContractError> {
    let storage_locations = validators
        .into_iter()
        .map(|v| {
            let storage_locations = STORAGE_LOCATIONS
                .may_load(deps.storage, v.to_vec())?
                .unwrap_or_default();
            Ok((v.to_hex(), storage_locations))
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(GetAnnounceStorageLocationsResponse { storage_locations })
}

fn get_validators(deps: Deps) -> Result<GetAnnouncedValidatorsResponse, ContractError> {
    let validators = VALIDATORS
        .keys(deps.storage, None, None, Order::Ascending)
        .map(|v| v.map(HexBinary::from).map(|v| v.to_hex()))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(GetAnnouncedValidatorsResponse { validators })
}

fn get_mailbox(deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: HexBinary::from(MAILBOX.load(deps.storage)?).to_hex(),
    })
}

fn get_local_domain(deps: Deps) -> Result<LocalDomainResponse, ContractError> {
    Ok(LocalDomainResponse {
        local_domain: LOCAL_DOMAIN.load(deps.storage)?,
    })
}

fn replay_hash(validator: &HexBinary, storage_location: &str) -> StdResult<HexBinary> {
    Ok(keccak256_hash(
        [validator.to_vec(), storage_location.as_bytes().to_vec()]
            .concat()
            .as_slice(),
    ))
}

fn domain_hash(local_domain: u32, mailbox: HexBinary) -> StdResult<HexBinary> {
    let mut bz = vec![];
    bz.append(&mut local_domain.to_be_bytes().to_vec());
    // left pad with zeroes
    let mut addr = [0u8; 32];
    addr[32 - mailbox.len()..].copy_from_slice(&mailbox);
    bz.append(&mut addr.to_vec());
    bz.append(&mut "HYPERLANE_ANNOUNCEMENT".as_bytes().to_vec());

    let hash = keccak256_hash(&bz);

    Ok(hash)
}

fn announcement_hash(mut domain_hash: Vec<u8>, storage_location: &str) -> HexBinary {
    let mut bz = vec![];
    bz.append(&mut domain_hash);
    bz.append(&mut storage_location.as_bytes().to_vec());

    keccak256_hash(&bz)
}

fn announce(
    deps: DepsMut,
    info: MessageInfo,
    validator: HexBinary,
    storage_location: String,
    signature: HexBinary,
) -> Result<Response, ContractError> {
    ensure_eq!(
        validator.len(),
        20,
        ContractError::invalid_addr("length should be 20")
    );

    // check replay protection
    let replay_id = replay_hash(&validator, &storage_location)?;
    ensure!(
        !REPLAY_PROTECITONS.has(deps.storage, replay_id.to_vec()),
        ContractError::unauthorized("replay protection triggered")
    );
    REPLAY_PROTECITONS.save(deps.storage, replay_id.to_vec(), &Empty {})?;

    // make announcement digest
    let local_domain = LOCAL_DOMAIN.load(deps.storage)?;
    let mailbox_addr = MAILBOX.load(deps.storage)?;

    // make digest
    let message_hash = eth_hash(announcement_hash(
        domain_hash(local_domain, mailbox_addr.into())?.to_vec(),
        &storage_location,
    ))?;

    // recover pubkey from signature & verify
    let pubkey = deps.api.secp256k1_recover_pubkey(
        &message_hash,
        &signature.as_slice()[..64],
        // We subs 27 according to this - https://eips.ethereum.org/EIPS/eip-155
        signature[64] - 27,
    )?;

    ensure_eq!(
        eth_addr(pubkey.into())?,
        validator,
        ContractError::VerifyFailed {}
    );

    // save validator if not saved yet
    if !VALIDATORS.has(deps.storage, validator.to_vec()) {
        VALIDATORS.save(deps.storage, validator.to_vec(), &Empty {})?;
    }

    // append storage_locations
    let mut storage_locations = STORAGE_LOCATIONS
        .may_load(deps.storage, validator.to_vec())?
        .unwrap_or_default();
    storage_locations.push(storage_location.clone());
    STORAGE_LOCATIONS.save(deps.storage, validator.to_vec(), &storage_locations)?;

    Ok(Response::new().add_event(
        Event::new("validator-announcement")
            .add_attribute("sender", info.sender)
            .add_attribute("validator", validator.to_string())
            .add_attribute("storage-location", storage_location),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        to_json_binary, ContractResult, QuerierResult, SystemResult, WasmQuery,
    };

    use hpl_interface::build_test_querier;
    use ibcx_test_utils::{gen_addr, gen_bz, hex};
    use k256::{
        ecdsa::{RecoveryId, Signature, SigningKey},
        elliptic_curve::{rand_core::OsRng, sec1::ToEncodedPoint},
        SecretKey,
    };
    use rstest::rstest;

    use super::*;

    build_test_querier!(crate::contract::query);

    struct Announcement {
        validator: HexBinary,
        mailbox: String,
        domain: u32,
        location: String,
        signature: HexBinary,
    }

    impl Announcement {
        fn new(
            validator: &str,
            mailbox: &str,
            domain: u32,
            location: &str,
            signature: &str,
        ) -> Self {
            Self {
                validator: hex(validator),
                mailbox: mailbox.into(),
                domain,
                location: location.into(),
                signature: hex(signature),
            }
        }

        fn preset() -> Self {
            Self::new(
                "05a9b5efe9f61f9142453d8e9f61565f333c6768",
                "00000000000000000000000049cfd6ef774acab14814d699e3f7ee36fdfba932",
                5,
                "s3://hyperlane-testnet4-goerli-validator-0/us-east-1",
                "dc47d48744fdb42b983f0244ed397feac08ee556eb48416582b5b638ada7b5322c8822e56a9020de7fe663ad43070f04b341514faf430ebf880bb1932434027d1c",
            )
        }

        fn preset_20_byte_address() -> Self {
            let mut p = Announcement::preset();
            p.mailbox = "49cfd6ef774acab14814d699e3f7ee36fdfba932".into();
            return p;
        }

        fn rand() -> Self {
            // prepare test data
            let mailbox = gen_bz(32);
            let local_domain = 26657;
            let storage_location = "file://foo/bar";

            // generate keypair
            let secret_key = SecretKey::random(&mut OsRng);
            let pubkey = secret_key.public_key();

            let signing_key = SigningKey::from(secret_key);
            let pubkey_bin = pubkey.to_encoded_point(false).as_bytes().to_vec();
            let addr_bin = eth_addr(pubkey_bin.into()).unwrap();

            // make announcement data
            let verify_digest = eth_hash(announcement_hash(
                domain_hash(local_domain, mailbox.clone()).unwrap().to_vec(),
                storage_location,
            ))
            .unwrap();
            let signature = pack_signature(
                signing_key
                    .sign_prehash_recoverable(&verify_digest)
                    .unwrap(),
            );

            Self {
                validator: addr_bin,
                mailbox: mailbox.to_hex(),
                domain: local_domain,
                location: storage_location.to_string(),
                signature,
            }
        }

        fn fail() -> Self {
            let mut announcement = Self::rand();

            announcement.domain = 26658;

            announcement
        }
    }

    fn pack_signature((rs, v): (Signature, RecoveryId)) -> HexBinary {
        let mut bz = rs.to_bytes().to_vec();
        bz.push(v.to_byte() + 27u8);
        bz.into()
    }

    #[rstest]
    fn test_init(#[values("osmo", "neutron")] hrp: &str) {
        let sender = gen_addr(hrp);
        let mailbox = gen_addr(hrp);

        let mut deps = mock_dependencies();

        deps.querier.update_wasm(|_: &WasmQuery| -> QuerierResult {
            SystemResult::Ok(ContractResult::Ok(
                to_json_binary(&mailbox::LocalDomainResponse {
                    local_domain: 26657,
                })
                .unwrap(),
            ))
        });

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info(sender.as_str(), &[]),
            InstantiateMsg {
                hrp: hrp.to_string(),
                mailbox: mailbox.to_string(),
            },
        )
        .unwrap();
    }

    #[rstest]
    fn test_queries(#[values(0, 4)] validators_len: usize, #[values(0, 4)] locations_len: usize) {
        let mut deps = mock_dependencies();

        let validators = (0..validators_len).map(|_| gen_bz(20)).collect::<Vec<_>>();
        for validator in validators {
            VALIDATORS
                .save(deps.as_mut().storage, validator.to_vec(), &Empty {})
                .unwrap();

            let locations = (0..locations_len)
                .map(|v| format!("file://foo/bar/{v}"))
                .collect::<Vec<_>>();

            STORAGE_LOCATIONS
                .save(deps.as_mut().storage, validator.to_vec(), &locations)
                .unwrap();
        }

        let GetAnnouncedValidatorsResponse { validators } =
            test_query(deps.as_ref(), QueryMsg::GetAnnouncedValidators {});
        assert_eq!(validators.len(), validators_len);

        let GetAnnounceStorageLocationsResponse { storage_locations } = test_query(
            deps.as_ref(),
            QueryMsg::GetAnnounceStorageLocations {
                validators: validators.iter().map(|v| hex(v)).collect(),
            },
        );
        for (validator, locations) in storage_locations {
            assert!(validators.contains(&validator));
            assert_eq!(locations.len(), locations_len);
        }
    }

    #[rstest]
    #[case::rand(Announcement::rand(), false)]
    #[case::actual_data(Announcement::preset(), false)]
    #[case::actual_data_20_bytes(Announcement::preset_20_byte_address(), false)]
    #[should_panic(expected = "unauthorized")]
    #[case::replay(Announcement::rand(), true)]
    #[should_panic(expected = "verify failed")]
    #[case::verify(Announcement::fail(), false)]
    fn test_announce(#[case] announcement: Announcement, #[case] enable_duplication: bool) {
        let validator = announcement.validator;
        let mailbox = HexBinary::from_hex(&announcement.mailbox).unwrap();

        let mut deps = mock_dependencies();

        LOCAL_DOMAIN
            .save(deps.as_mut().storage, &announcement.domain)
            .unwrap();
        MAILBOX
            .save(deps.as_mut().storage, &mailbox.to_vec())
            .unwrap();

        let replay_id = replay_hash(&validator, &announcement.location).unwrap();
        if enable_duplication {
            REPLAY_PROTECITONS
                .save(deps.as_mut().storage, replay_id.to_vec(), &Empty {})
                .unwrap();
        }

        announce(
            deps.as_mut(),
            mock_info("someone", &[]),
            validator.clone(),
            announcement.location.clone(),
            announcement.signature,
        )
        .map_err(|e| e.to_string())
        .unwrap();

        // check state
        assert!(REPLAY_PROTECITONS.has(deps.as_ref().storage, replay_id.to_vec()));
        assert!(VALIDATORS.has(deps.as_ref().storage, validator.to_vec()));
        assert_eq!(
            STORAGE_LOCATIONS
                .load(deps.as_ref().storage, validator.to_vec())
                .unwrap(),
            vec![announcement.location]
        );
    }
}
