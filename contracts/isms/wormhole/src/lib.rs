mod error;

pub use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    ensure_eq, Addr, Binary, Deps, DepsMut, Empty, Env, Event, HexBinary, MessageInfo,
    QueryResponse, Response,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;
use hpl_interface::{
    ism::{
        wormhole::{ExecuteMsg, InstantiateMsg, QueryMsg, WormholeIsmQueryMsg},
        IsmQueryMsg, IsmType, ModuleTypeResponse, VerifyInfoResponse, VerifyResponse,
    },
    to_binary,
    types::{bech32_decode, Message},
};

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const WORMHOLE_CORE_KEY: &str = "wormhole_core";
const WORMHOLE_CORE: Item<Addr> = Item::new(WORMHOLE_CORE_KEY);

#[cw_serde]
enum WormholeQueryMsg {
    VerifyVAA { vaa: Binary, block_time: u64 },
}

#[cw_serde]
struct ParsedVAA {
    pub version: u8,
    pub guardian_set_index: u32,
    pub timestamp: u32,
    pub nonce: u32,
    pub len_signers: u8,

    pub emitter_chain: u16,
    pub emitter_address: Vec<u8>,
    pub sequence: u64,
    pub consistency_level: u8,
    pub payload: Vec<u8>,

    pub hash: Vec<u8>,
}

fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_ism_wormhole::{}", name))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    hpl_ownable::initialize(deps.storage, &owner)?;

    let wormhole_core = deps.api.addr_validate(&msg.wormhole_core)?;
    WORMHOLE_CORE.save(deps.storage, &wormhole_core)?;

    Ok(Response::new().add_event(
        new_event("instantiate")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("wormhole_core", wormhole_core),
    ))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::SetWormholeCore { wormhole_core } => {
            ensure_eq!(
                hpl_ownable::get_owner(deps.storage)?,
                info.sender,
                ContractError::Unauthorized
            );

            let wormhole_core = deps.api.addr_validate(&wormhole_core)?;
            WORMHOLE_CORE.save(deps.storage, &wormhole_core)?;

            Ok(Response::new().add_event(
                new_event("set_wormhole_core").add_attribute("wormhole_core", wormhole_core),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use IsmQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),

        QueryMsg::Ism(msg) => match msg {
            ModuleType {} => to_binary({
                Ok::<_, ContractError>(ModuleTypeResponse {
                    typ: IsmType::Wormhole,
                })
            }),
            Verify { metadata, message } => to_binary(verify(deps, metadata, message)),
            VerifyInfo { message } => to_binary(verify_info(deps, message)),
        },

        QueryMsg::WormholeIsm(msg) => match msg {
            WormholeIsmQueryMsg::WormholeCore {} => Ok(cosmwasm_std::to_binary(
                &WORMHOLE_CORE.load(deps.storage)?.into_string(),
            )?),
        },
    }
}

fn verify(
    deps: Deps,
    metadata: HexBinary,
    message: HexBinary,
) -> Result<VerifyResponse, ContractError> {
    let wormhole_core = WORMHOLE_CORE.load(deps.storage)?;
    let wormhole_query_msg = WormholeQueryMsg::VerifyVAA {
        vaa: Binary::from(metadata.as_slice()),
        block_time: 0,
    };

    let parsed_vaa: ParsedVAA = deps
        .querier
        .query_wasm_smart(wormhole_core, &wormhole_query_msg)?;

    let message: Message = message.into();
    let id = message.id();

    Ok(VerifyResponse {
        verified: HexBinary::from(parsed_vaa.payload) == id,
    })
}

fn verify_info(deps: Deps, _message: HexBinary) -> Result<VerifyInfoResponse, ContractError> {
    // this is not entirely correct, but I don't see a better way to do this
    // we cannot query validators from Wormhole Core contract
    Ok(VerifyInfoResponse {
        threshold: 1,
        validators: vec![bech32_decode(WORMHOLE_CORE.load(deps.storage)?.as_str())?.into()],
    })
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use crate::{verify, ParsedVAA, WormholeQueryMsg, WORMHOLE_CORE};
    use cosmwasm_std::{
        from_binary, from_slice,
        testing::{MockApi, MockStorage},
        to_binary, Addr, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError,
        SystemResult, WasmQuery,
    };
    use hpl_interface::ism::VerifyResponse;
    use ibcx_test_utils::hex;
    use std::marker::PhantomData;

    #[derive(Default)]
    struct CustomMockQuerier {}
    impl Querier for CustomMockQuerier {
        fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
            let request = match from_slice::<QueryRequest<Empty>>(bin_request).map_err(move |err| {
                QuerierResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", err),
                    request: bin_request.into(),
                })
            }) {
                Ok(v) => v,
                Err(e) => return e,
            };
            match request {
                QueryRequest::Wasm(request) => match request {
                    WasmQuery::Smart { contract_addr, msg } => {
                        const VAA: &str = "AQAAAAABAEvaVvB61VMTIBPWEgQstR04OEv9Stj+mZ2CkPwlPDRIfdL2MMXRViirkq0bHbUMtQM9gcAymhWj9NbT68PdER0BZV2uKAAAAAAAAgAAAAAAAAAAAAAAALlNEM3S65/plozvlT1fbhBX58ZAAAAAAAAAAADISIvbtb29zQ1v9P7OL15QdlONeG2fpn/7Ldsuf4RQ2ZA=";
                        assert_eq!(contract_addr, "wormhole_core");
                        let WormholeQueryMsg::VerifyVAA { vaa, block_time } =
                            from_binary(&msg).unwrap();
                        assert_eq!(block_time, 0);
                        assert_eq!(vaa.to_string(), VAA);
                        let vaa = ParsedVAA {
                            version: 1,
                            guardian_set_index: 0,
                            timestamp: 1700638248,
                            nonce: 0,
                            len_signers: 1,
                            emitter_chain: 2,
                            emitter_address: vec![
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 185, 77, 16, 205, 210, 235,
                                159, 233, 150, 140, 239, 149, 61, 95, 110, 16, 87, 231, 198, 64,
                            ],
                            sequence: 0,
                            consistency_level: 200,
                            payload: vec![
                                72, 139, 219, 181, 189, 189, 205, 13, 111, 244, 254, 206, 47, 94,
                                80, 118, 83, 141, 120, 109, 159, 166, 127, 251, 45, 219, 46, 127,
                                132, 80, 217, 144,
                            ],
                            hash: vec![
                                23, 195, 158, 108, 197, 235, 130, 102, 185, 255, 225, 41, 128, 71,
                                192, 121, 198, 19, 185, 49, 121, 235, 149, 124, 199, 132, 227, 245,
                                29, 120, 129, 140,
                            ],
                        };
                        SystemResult::Ok(cosmwasm_std::ContractResult::from(to_binary(&vaa)))
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
    }

    #[test]
    fn verification_test() {
        let mut deps = OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: CustomMockQuerier::default(),
            custom_query_type: PhantomData::<Empty>,
        };
        WORMHOLE_CORE
            .save(deps.as_mut().storage, &Addr::unchecked("wormhole_core"))
            .unwrap();

        let message = hex("03000000240001388100000000000000000000000004980c17e2ce26578c82f81207e706e4505fae3b0000a8690000000000000000000000000b1c1b54f45e02552331d3106e71f5e0b573d5d448656c6c6f21");
        let metadata = hex("010000000001004bda56f07ad553132013d612042cb51d38384bfd4ad8fe999d8290fc253c34487dd2f630c5d15628ab92ad1b1db50cb5033d81c0329a15a3f4d6d3ebc3dd111d01655dae28000000000002000000000000000000000000b94d10cdd2eb9fe9968cef953d5f6e1057e7c6400000000000000000c8488bdbb5bdbdcd0d6ff4fece2f5e5076538d786d9fa67ffb2ddb2e7f8450d990");
        let result = verify(deps.as_ref(), metadata, message).unwrap();
        assert_eq!(result, VerifyResponse { verified: true });
    }
}
