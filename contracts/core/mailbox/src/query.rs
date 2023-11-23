use cosmwasm_std::{Coins, Deps, HexBinary, StdResult};
use hpl_interface::{
    core::mailbox::{
        DefaultHookResponse, DefaultIsmResponse, DispatchMsg, HrpResponse,
        LatestDispatchedIdResponse, LocalDomainResponse, MessageDeliveredResponse, NonceResponse,
        RecipientIsmResponse, RequiredHookResponse,
    },
    hook::{self, QuoteDispatchResponse},
    ism,
};

use crate::{
    state::{CONFIG, DELIVERIES, LATEST_DISPATCHED_ID, NONCE},
    ContractError, MAILBOX_VERSION,
};

pub fn get_hrp(deps: Deps) -> Result<HrpResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(HrpResponse { hrp: config.hrp })
}

pub fn get_local_domain(deps: Deps) -> Result<LocalDomainResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(LocalDomainResponse {
        local_domain: config.local_domain,
    })
}

pub fn get_default_ism(deps: Deps) -> Result<DefaultIsmResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(DefaultIsmResponse {
        default_ism: config.get_default_ism().into(),
    })
}

pub fn get_default_hook(deps: Deps) -> Result<DefaultHookResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(DefaultHookResponse {
        default_hook: config.get_default_hook().into(),
    })
}

pub fn get_required_hook(deps: Deps) -> Result<RequiredHookResponse, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    Ok(RequiredHookResponse {
        required_hook: config.get_required_hook().into(),
    })
}

pub fn get_delivered(deps: Deps, id: HexBinary) -> Result<MessageDeliveredResponse, ContractError> {
    let delivered = DELIVERIES.has(deps.storage, id.to_vec());

    Ok(MessageDeliveredResponse { delivered })
}

pub fn get_nonce(deps: Deps) -> Result<NonceResponse, ContractError> {
    let nonce = NONCE.load(deps.storage)?;

    Ok(NonceResponse { nonce })
}

pub fn get_recipient_ism(
    deps: Deps,
    recipient: String,
) -> Result<RecipientIsmResponse, ContractError> {
    let default_ism = CONFIG.load(deps.storage)?.get_default_ism();

    let recipient = deps.api.addr_validate(&recipient)?;

    let ism = ism::recipient(&deps.querier, recipient)?.unwrap_or(default_ism);

    Ok(RecipientIsmResponse { ism: ism.into() })
}

pub fn get_latest_dispatch_id(deps: Deps) -> Result<LatestDispatchedIdResponse, ContractError> {
    let latest_dispatched_id = LATEST_DISPATCHED_ID.load(deps.storage)?.into();

    Ok(LatestDispatchedIdResponse {
        message_id: latest_dispatched_id,
    })
}

pub fn quote_dispatch(
    deps: Deps,
    sender: String,
    dispatch_msg: DispatchMsg,
) -> Result<QuoteDispatchResponse, ContractError> {
    let sender = deps.api.addr_validate(&sender)?;

    let config = CONFIG.load(deps.storage)?;
    let nonce = NONCE.load(deps.storage)?;

    let msg = dispatch_msg
        .clone()
        .to_msg(MAILBOX_VERSION, nonce, config.local_domain, sender)?;

    let default_hook = config.get_default_hook();
    let required_hook = config.get_required_hook();

    let base_fee = hook::quote_dispatch(
        &deps.querier,
        dispatch_msg.get_hook_addr(deps.api, default_hook)?,
        dispatch_msg.metadata.clone().unwrap_or_default(),
        msg.clone(),
    )?
    .fees;

    let required_fee = hook::quote_dispatch(
        &deps.querier,
        required_hook,
        dispatch_msg.metadata.unwrap_or_default(),
        msg,
    )?
    .fees;

    let total_fee =
        required_fee
            .into_iter()
            .try_fold(Coins::try_from(base_fee)?, |mut acc, fee| {
                acc.add(fee)?;
                StdResult::Ok(acc)
            })?;

    Ok(QuoteDispatchResponse {
        fees: total_fee.to_vec(),
    })
}

#[cfg(test)]
mod test {

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env},
        Addr,
    };
    use hpl_interface::core::mailbox::MailboxQueryMsg;
    use ibcx_test_utils::{gen_addr, gen_bz, hex};
    use rstest::rstest;
    use serde::de::DeserializeOwned;

    use crate::{
        contract,
        state::{Config, Delivery},
    };

    use super::*;

    fn query<T: DeserializeOwned>(deps: Deps, req: MailboxQueryMsg) -> T {
        from_binary(&contract::query(deps, mock_env(), req.wrap()).unwrap()).unwrap()
    }

    fn query_hrp(deps: Deps) -> HrpResponse {
        query(deps, MailboxQueryMsg::Hrp {})
    }

    fn query_local_domain(deps: Deps) -> LocalDomainResponse {
        query(deps, MailboxQueryMsg::LocalDomain {})
    }

    fn query_default_hook(deps: Deps) -> DefaultHookResponse {
        query(deps, MailboxQueryMsg::DefaultHook {})
    }

    fn query_default_ism(deps: Deps) -> DefaultIsmResponse {
        query(deps, MailboxQueryMsg::DefaultIsm {})
    }

    fn query_required_hook(deps: Deps) -> RequiredHookResponse {
        query(deps, MailboxQueryMsg::RequiredHook {})
    }

    fn query_delivered(deps: Deps, id: HexBinary) -> MessageDeliveredResponse {
        query(deps, MailboxQueryMsg::MessageDelivered { id })
    }

    fn query_nonce(deps: Deps) -> NonceResponse {
        query(deps, MailboxQueryMsg::Nonce {})
    }

    #[rstest]
    #[case(
        Some(gen_addr("osmo")),
        Some(gen_addr("neutron")),
        Some(gen_addr("cosmos"))
    )]
    #[should_panic(expected = "default_ism not set")]
    #[case(None, Some(gen_addr("neutron")), Some(gen_addr("cosmos")))]
    #[should_panic(expected = "default_hook not set")]
    #[case(Some(gen_addr("osmo")), None, Some(gen_addr("cosmos")))]
    #[should_panic(expected = "required_hook not set")]
    #[case(Some(gen_addr("osmo")), Some(gen_addr("neutron")), None)]
    fn test_query_config(
        #[case] default_ism: Option<Addr>,
        #[case] default_hook: Option<Addr>,
        #[case] required_hook: Option<Addr>,
    ) {
        let mut deps = mock_dependencies();

        CONFIG
            .save(
                deps.as_mut().storage,
                &Config {
                    default_hook: default_hook.clone(),
                    default_ism: default_ism.clone(),
                    required_hook: required_hook.clone(),
                    ..Config::new("hrp", 123)
                },
            )
            .unwrap();

        NONCE.save(deps.as_mut().storage, &7u32).unwrap();

        let hrp_res = query_hrp(deps.as_ref()).hrp;

        let local_domain_res = query_local_domain(deps.as_ref()).local_domain;

        let default_hook_res = query_default_hook(deps.as_ref()).default_hook;

        let default_ism_res = query_default_ism(deps.as_ref()).default_ism;

        let required_hook_res = query_required_hook(deps.as_ref()).required_hook;

        let nonce_res = query_nonce(deps.as_ref()).nonce;

        assert_eq!(hrp_res, "hrp");
        assert_eq!(local_domain_res, 123);
        assert_eq!(default_hook_res, default_hook.unwrap());
        assert_eq!(default_ism_res, default_ism.unwrap());
        assert_eq!(required_hook_res, required_hook.unwrap());
        assert_eq!(nonce_res, 7);
    }

    #[rstest]
    #[case(None, false)]
    #[case(Some(hex("beef")), true)]
    #[case(Some(hex("deadbeef")), false)]
    fn test_query_delivered(#[case] message_id: Option<HexBinary>, #[case] delivered: bool) {
        let mut deps = mock_dependencies();

        if let Some(id) = message_id {
            DELIVERIES
                .save(
                    deps.as_mut().storage,
                    id.to_vec(),
                    &Delivery {
                        sender: Addr::unchecked("sender"),
                        block_number: 123,
                    },
                )
                .unwrap();
        }

        assert_eq!(
            query_delivered(deps.as_ref(), HexBinary::from_hex("beef").unwrap()).delivered,
            delivered
        );
    }

    #[rstest]
    fn test_query_latest_dispatched_id() {
        let mut deps = mock_dependencies();

        let rand_id = gen_bz(32).to_vec();

        LATEST_DISPATCHED_ID
            .save(deps.as_mut().storage, &rand_id)
            .unwrap();

        let res: LatestDispatchedIdResponse =
            query(deps.as_ref(), MailboxQueryMsg::LatestDispatchId {});
        assert_eq!(res.message_id, rand_id);
    }
}
