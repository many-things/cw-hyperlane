#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use serde_json_wasm::to_string;
use cosmwasm_std::{Event, MessageInfo, ensure_eq, QueryResponse, Response, DepsMut, Deps, Env, StdResult, StdError, Addr};
use cw_storage_plus::Item;
use hpl_interface::{
    core::mailbox::{LatestDispatchedIdResponse, MailboxQueryMsg},
    hook::{
        axelar::{ExecuteMsg, InstantiateMsg, QueryMsg, AxelarInfoResponse, AxelarQueryMsg, AxelarFee, AxelarGeneralMessage, QuoteDispatchMetadata, RegisterDestinationContractMsg},
        HookQueryMsg, MailboxResponse, QuoteDispatchResponse, PostDispatchMsg, QuoteDispatchMsg,
    },
    to_binary,
    types::Message,
};
use ethabi::{Address, encode, Token};
use ethabi::ethereum_types::H160;
use osmosis_std::types::ibc::applications::transfer::{v1::MsgTransfer};
use hpl_ownable::get_owner;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const AXELAR_GATEWAY: &str = "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5";

// TODO: move these to a single struct
const DESTINATION_CHAIN_KEY: &str = "destination_chain";
const DESTINATION_CHAIN: Item<String> = Item::new(DESTINATION_CHAIN_KEY);

const DESTINATION_CONTRACT_KEY: &str = "destination_contract";
const DESTINATION_CONTRACT: Item<String> = Item::new(DESTINATION_CONTRACT_KEY);

const DESTINATION_ISM_KEY: &str = "destination_ism";
const DESTINATION_ISM: Item<String> = Item::new(DESTINATION_ISM_KEY);

const AXELAR_GATEWAY_CHANNEL_KEY: &str = "axelar_gateway_channel";
const AXELAR_GATEWAY_CHANNEL: Item<String> = Item::new(AXELAR_GATEWAY_CHANNEL_KEY);

pub const MAILBOX_KEY: &str = "mailbox";
pub const MAILBOX: Item<Addr> = Item::new(MAILBOX_KEY);



fn new_event(name: &str) -> Event {
    Event::new(format!("hpl_hook_pausable::{}", name))
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("hook paused")]
    Paused {},

    #[error("invalid recipient address")]
    InvalidRecipientAddress {address: String},



}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    hpl_ownable::initialize(deps.storage, &owner)?;
    
    let destination_chain = &msg.destination_chain;
    let destination_contract = &msg.destination_contract;
    let destination_ism = &msg.destination_ism;
    let axelar_gateway_channel = &msg.axelar_gateway_channel;
    let mailbox: Addr = deps.api.addr_validate(&msg.mailbox)?;
        
    DESTINATION_CHAIN.save(deps.storage, destination_chain)?;
    DESTINATION_CONTRACT.save(deps.storage, destination_contract)?;
    DESTINATION_ISM.save(deps.storage, destination_ism)?;
    AXELAR_GATEWAY_CHANNEL.save(deps.storage, axelar_gateway_channel)?;
    MAILBOX.save(deps.storage, &mailbox)?;

    Ok(Response::new().add_event(
        new_event("initialize")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("destination_chain", destination_chain)
            .add_attribute("destination_contract", destination_contract)
            .add_attribute("destination_ism", destination_ism)
            .add_attribute("destination_ism", axelar_gateway_channel)


    ))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // TODO: maybe add SetWomrholeCore Msg
        ExecuteMsg::Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),
        ExecuteMsg::PostDispatch(msg) => post_dispatch(deps, env, info, msg),
        ExecuteMsg::RegisterDestinationContract(msg) => register_desination_contract(deps, info, msg),
    }
}

fn register_desination_contract(
    deps: DepsMut,
    info: MessageInfo,
    msg: RegisterDestinationContractMsg,
) -> Result<Response, ContractError> {
    ensure_eq!(
        get_owner(deps.storage)?,
        info.sender,
        ContractError::Unauthorized {}
    );

    let destination_contract = &msg.destination_contract;
    DESTINATION_CONTRACT.save(deps.storage, destination_contract)?;

    Ok(Response::new().add_event(
        new_event("register_destination_contract")
            .add_attribute("destination_contract", destination_contract)


    ))   
}

fn post_dispatch(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    req: PostDispatchMsg,
) -> Result<Response, ContractError> {
  
    // Ensure message_id matches latest dispatch from mailbox
    let mailbox = MAILBOX.load(deps.storage)?;
    let latest_dispatch_id = deps
        .querier
        .query_wasm_smart::<LatestDispatchedIdResponse>(
            &mailbox,
            &MailboxQueryMsg::LatestDispatchId {}.wrap(),
        )?
        .message_id;

    let decoded_msg: Message = req.message.clone().into();

    ensure_eq!(
        latest_dispatch_id,
        decoded_msg.id(),
        ContractError::Unauthorized {}
    );

    //send message to axelar gateway
    let desination_chain = DESTINATION_CHAIN.load(deps.storage)?;
    let desination_contract = DESTINATION_CONTRACT.load(deps.storage)?;
    let desination_ism = DESTINATION_ISM.load(deps.storage)?;
    let axelar_gateway_channel = AXELAR_GATEWAY_CHANNEL.load(deps.storage)?;
    let recipients = vec![desination_ism];

    // TODO: do we need to pass a fee?
    multi_send_to_evm(deps, env, info, axelar_gateway_channel, desination_chain, desination_contract, recipients, None)

}

pub fn multi_send_to_evm(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    gateway_channel: String,
    destination_chain: String,
    destination_contract: String,
    recipients: Vec<String>,
    fee: Option<AxelarFee>
) -> Result<Response, ContractError> {
    let addresses = recipients
    .into_iter()
    .map(|s| {
        match s.parse::<H160>() {
            Ok(address) => Ok(Token::Address(Address::from(address))),
            Err(_) => Err(ContractError::InvalidRecipientAddress { address: s }),
        }
    })
    .collect::<Result<Vec<Token>, ContractError>>()?;
    let payload = encode(&[Token::Array(addresses)]);

    let msg = AxelarGeneralMessage {
        destination_chain,
        destination_address: destination_contract.clone(),
        payload,
        type_: 2,
        fee
    };

    let coin = cw_utils::one_coin(&info).unwrap();
    let ibc_transfer = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: gateway_channel,
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: AXELAR_GATEWAY.to_string(),
        timeout_height: None,
        timeout_timestamp: env.block.time.plus_seconds(604_800u64).nanos(),
        memo: to_string(&msg).unwrap(),
    };

    // Base response
    let response = Response::new()
        .add_attribute("status", "ibc_message_created")
        .add_attribute("ibc_message", format!("{:?}", ibc_transfer));

    return Ok(response.add_message(ibc_transfer));
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::Axelar(msg) => Ok(handle_query(deps, env, msg)?),
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Hook(msg) => match msg {
            HookQueryMsg::Mailbox {} => to_binary(get_mailbox(deps)),
            HookQueryMsg::QuoteDispatch(_) => to_binary(quote_dispatch(msg)),
        },
    }
}

pub fn handle_query(
    deps: Deps,
    _env: Env,
    _msg: AxelarQueryMsg,
) -> StdResult<QueryResponse> {
    cosmwasm_std::to_binary(&AxelarInfoResponse {
            destination_chain: DESTINATION_CHAIN.load(deps.storage)?,
            destination_contract: DESTINATION_CONTRACT.load(deps.storage)?,
            destination_ism: DESTINATION_ISM.load(deps.storage)?,
            axelar_gateway_channel: AXELAR_GATEWAY_CHANNEL.load(deps.storage)?,
        })
}

fn get_mailbox(_deps: Deps) -> Result<MailboxResponse, ContractError> {
    Ok(MailboxResponse {
        mailbox: "unrestricted".to_string(),
    })
}

fn quote_dispatch(msg: QuoteDispatchMsg) -> Result<QuoteDispatchResponse, ContractError> {
    let decoded_metadata: QuoteDispatchMetadata = req.message.clone().into();
    Ok(QuoteDispatchResponse { gas_amount: decoded_metadata.coin})
}