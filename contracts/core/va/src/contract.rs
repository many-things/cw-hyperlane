#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, Addr, Deps, DepsMut, Empty, Env, Event, HexBinary, MessageInfo, Order, QueryResponse,
    Response, StdResult,
};

use hpl_interface::{
    core::{
        mailbox::{self, MailboxQueryMsg},
        va::{
            ExecuteMsg, GetAnnounceStorageLocationsResponse, GetAnnouncedValidatorsResponse,
            InstantiateMsg, QueryMsg,
        },
    },
    to_binary,
    types::{bech32_decode, bech32_encode, eth_hash, keccak256_hash, pub_to_addr},
};
use k256::{ecdsa::VerifyingKey, EncodedPoint};

use crate::{
    error::ContractError,
    state::{HRP, LOCAL_DOMAIN, MAILBOX, REPLAY_PROTECITONS, STORAGE_LOCATIONS, VALIDATORS},
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

    let local_domain = deps
        .querier
        .query_wasm_smart::<mailbox::LocalDomainResponse>(
            &mailbox,
            &mailbox::QueryMsg::Mailbox(MailboxQueryMsg::LocalDomain {}),
        )?
        .local_domain;

    HRP.save(deps.storage, &msg.hrp)?;
    MAILBOX.save(deps.storage, &mailbox)?;
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
    }
}

fn get_announce(
    deps: Deps,
    validators: Vec<HexBinary>,
) -> Result<GetAnnounceStorageLocationsResponse, ContractError> {
    let hrp = HRP.load(deps.storage)?;

    let storage_locations = validators
        .into_iter()
        .map(|v| {
            let raw_validator = bech32_encode(&hrp, &v)?;
            let validator = deps.api.addr_validate(raw_validator.as_str())?;

            let storage_locations = STORAGE_LOCATIONS
                .may_load(deps.storage, validator.clone())?
                .unwrap_or_default();
            Ok((validator.to_string(), storage_locations))
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(GetAnnounceStorageLocationsResponse { storage_locations })
}

fn get_validators(deps: Deps) -> Result<GetAnnouncedValidatorsResponse, ContractError> {
    let validators = VALIDATORS
        .keys(deps.storage, None, None, Order::Ascending)
        .map(|v| v.map(String::from))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(GetAnnouncedValidatorsResponse { validators })
}

fn replay_hash(validator: &Addr, storage_location: &str) -> StdResult<HexBinary> {
    Ok(keccak256_hash(
        [
            bech32_decode(validator.as_str())?,
            storage_location.as_bytes().to_vec(),
        ]
        .concat()
        .as_slice(),
    ))
}

fn domain_hash(local_domain: u32, mailbox: &str) -> StdResult<HexBinary> {
    let mut bz = vec![];
    bz.append(&mut local_domain.to_be_bytes().to_vec());
    bz.append(&mut bech32_decode(mailbox)?);
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
    let hrp = HRP.load(deps.storage)?;
    let raw_validator = bech32_encode(hrp.as_str(), &validator)?;
    let validator = deps.api.addr_validate(raw_validator.as_str())?;

    // check replay protection
    let replay_id = replay_hash(&validator, &storage_location)?;
    ensure!(
        !REPLAY_PROTECITONS.has(deps.storage, replay_id.to_vec()),
        ContractError::Unauthorized {}
    );
    REPLAY_PROTECITONS.save(deps.storage, replay_id.to_vec(), &Empty {})?;

    // make announcement digest
    let local_domain = LOCAL_DOMAIN.load(deps.storage)?;
    let mailbox = MAILBOX.load(deps.storage)?;

    // make digest
    let message_hash = eth_hash(announcement_hash(
        domain_hash(local_domain, mailbox.as_str())?.to_vec(),
        &storage_location,
    ))?;

    // recover pubkey from signature & verify
    let recovered = deps.api.secp256k1_recover_pubkey(
        &message_hash,
        &signature.as_slice()[..64],
        // We subs 27 according to this - https://eips.ethereum.org/EIPS/eip-155
        signature[64] - 27,
    )?;

    let pubkey = EncodedPoint::from_bytes(recovered).expect("failed to parse recovered pubkey");
    let pubkey = VerifyingKey::from_encoded_point(&pubkey).expect("invalid recovered public key");
    let pubkey_bin = pubkey.to_encoded_point(true).as_bytes().to_vec();

    let recovered_addr = bech32_encode(&hrp, &pub_to_addr(pubkey_bin.into())?)?;
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

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        ContractResult, QuerierResult, SystemResult, WasmQuery,
    };
    use ibcx_test_utils::{gen_addr, gen_bz};
    use k256::{
        ecdsa::{RecoveryId, Signature, SigningKey},
        elliptic_curve::{rand_core::OsRng, sec1::ToEncodedPoint},
        SecretKey,
    };
    use rstest::rstest;
    use serde::de::DeserializeOwned;

    use super::*;

    struct Announcement {
        validator: String,
        mailbox: String,
        domain: u32,
        location: String,
        signature: String,
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
                validator: validator.into(),
                mailbox: mailbox.into(),
                domain,
                location: location.into(),
                signature: signature.into(),
            }
        }

        fn preset1() -> Self {
            Self::new(
                "f9e25a6be80f6d48727e42381fc3c3b7834c0cb4",
                "62634b0c56b57fef1c27f25039cfb872875a9eeeb42d80a034f8d6b55ed20d09",
                26658,
                "file:///var/folders/3v/g38z040x54x8l6b160vv66b40000gn/T/.tmp7XoxND/checkpoint",
                "6c30e1072f0e23694d3a3a96dc41fc4d17636ce145e83adef3224a6f4732c2db715407b42478c581b6ac1b79e64807a7748935d398a33bf4b73d37924c293c941b",
            )
        }

        fn preset2() -> Self {
            Self::new(
                "f9e25a6be80f6d48727e42381fc3c3b7834c0cb4",
                "62634b0c56b57fef1c27f25039cfb872875a9eeeb42d80a034f8d6b55ed20d09",
                26657,
                "file:///var/folders/3v/g38z040x54x8l6b160vv66b40000gn/T/.tmpBJPK8C/checkpoint",
                "76c637d605f683734c672c0437f14ae48520e85fb68b0c0b9c28069f183e3bfc46f0de0655f06937c74b5a0a15f5b8fe37f1d1ad4dd8b64dc55307a2103fedad1c",
            )
        }

        fn rand() -> Self {
            // prepare test data
            let mailbox = gen_bz(20);
            let local_domain = 26657;
            let storage_location = "file://foo/bar";

            // generate keypair
            let secret_key = SecretKey::random(&mut OsRng);
            let pubkey = secret_key.public_key();

            let signing_key = SigningKey::from(secret_key);
            let pubkey_bin = pubkey.to_encoded_point(true).as_bytes().to_vec();

            let addr_bin = pub_to_addr(pubkey_bin.into()).unwrap();

            // make announcement data
            let verify_digest = eth_hash(announcement_hash(
                domain_hash(
                    local_domain,
                    bech32_encode("asdf", mailbox.as_slice()).unwrap().as_str(),
                )
                .unwrap()
                .to_vec(),
                storage_location,
            ))
            .unwrap();
            let signature = pack_signature(
                signing_key
                    .sign_prehash_recoverable(&verify_digest)
                    .unwrap(),
            );

            Self {
                validator: addr_bin.to_hex(),
                mailbox: mailbox.to_hex(),
                domain: local_domain,
                location: storage_location.to_string(),
                signature: signature.to_hex(),
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

    fn query<T: DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
        let res = super::query(deps, mock_env(), msg).unwrap();
        cosmwasm_std::from_binary(&res).unwrap()
    }

    #[rstest]
    fn test_init(#[values("osmo", "neutron")] hrp: &str) {
        let sender = gen_addr(hrp);
        let mailbox = gen_addr(hrp);

        let mut deps = mock_dependencies();

        deps.querier.update_wasm(|_: &WasmQuery| -> QuerierResult {
            SystemResult::Ok(ContractResult::Ok(
                cosmwasm_std::to_binary(&mailbox::LocalDomainResponse {
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

        assert_eq!(HRP.load(deps.as_ref().storage).unwrap(), hrp);
    }

    #[rstest]
    fn test_query(
        #[values("osmo", "neutron")] hrp: &str,
        #[values(0, 4)] validators_len: usize,
        #[values(0, 4)] locations_len: usize,
    ) {
        let mut deps = mock_dependencies();

        HRP.save(deps.as_mut().storage, &hrp.to_string()).unwrap();

        let validators = (0..validators_len)
            .map(|_| gen_addr(hrp))
            .collect::<Vec<_>>();
        for validator in validators {
            VALIDATORS
                .save(deps.as_mut().storage, validator.clone(), &Empty {})
                .unwrap();

            let locations = (0..locations_len)
                .map(|v| format!("file://foo/bar/{v}"))
                .collect::<Vec<_>>();

            STORAGE_LOCATIONS
                .save(deps.as_mut().storage, validator, &locations)
                .unwrap();
        }

        let GetAnnouncedValidatorsResponse { validators } =
            query(deps.as_ref(), QueryMsg::GetAnnouncedValidators {});
        assert_eq!(validators.len(), validators_len);

        let GetAnnounceStorageLocationsResponse { storage_locations } = query(
            deps.as_ref(),
            QueryMsg::GetAnnounceStorageLocations {
                validators: validators
                    .iter()
                    .map(|v| HexBinary::from(bech32_decode(v).unwrap()))
                    .collect::<Vec<_>>(),
            },
        );
        for (validator, locations) in storage_locations {
            assert!(validators.contains(&validator));
            assert_eq!(locations.len(), locations_len);
        }
    }

    #[rstest]
    #[case::rand(Announcement::rand(), false)]
    #[case::actual_data_1(Announcement::preset1(), false)]
    #[case::actual_data_2(Announcement::preset2(), false)]
    #[should_panic(expected = "unauthorized")]
    #[case::replay(Announcement::rand(), true)]
    #[should_panic(expected = "verify failed")]
    #[case::verify(Announcement::fail(), false)]
    fn test_announce(
        #[values("osmo", "neutron")] hrp: &str,
        #[case] announcement: Announcement,
        #[case] enable_duplication: bool,
    ) {
        let validator = HexBinary::from_hex(&announcement.validator).unwrap();
        let validator_addr = bech32_encode(hrp, validator.as_slice()).unwrap();

        let mailbox = HexBinary::from_hex(&announcement.mailbox).unwrap();
        let mailbox_addr = bech32_encode(hrp, mailbox.as_slice()).unwrap();

        let mut deps = mock_dependencies();

        HRP.save(deps.as_mut().storage, &hrp.to_string()).unwrap();
        LOCAL_DOMAIN
            .save(deps.as_mut().storage, &announcement.domain)
            .unwrap();
        MAILBOX.save(deps.as_mut().storage, &mailbox_addr).unwrap();

        let replay_id = replay_hash(&validator_addr, &announcement.location).unwrap();
        if enable_duplication {
            REPLAY_PROTECITONS
                .save(deps.as_mut().storage, replay_id.to_vec(), &Empty {})
                .unwrap();
        }

        announce(
            deps.as_mut(),
            mock_info(validator_addr.as_str(), &[]),
            validator,
            announcement.location.clone(),
            HexBinary::from_hex(&announcement.signature).unwrap(),
        )
        .map_err(|e| e.to_string())
        .unwrap();

        // check state
        assert!(REPLAY_PROTECITONS.has(deps.as_ref().storage, replay_id.to_vec()));
        assert!(VALIDATORS.has(deps.as_ref().storage, validator_addr.clone()));
        assert_eq!(
            STORAGE_LOCATIONS
                .load(deps.as_ref().storage, validator_addr)
                .unwrap(),
            vec![announcement.location]
        );
    }
}
