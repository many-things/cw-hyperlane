use cosmwasm_std::{to_binary, Deps, Env, QueryResponse};
use hpl_interface::{
    hook::{OwnerResponse, PauseInfoResponse},
    igp_core::QuoteGasPaymentResponse,
    post_dispatch_hook::PostDispatchQueryMsg,
    types::message::Message,
};

use crate::{
    state::{HOOK_CONFIG, PAUSE},
    ContractError,
};

pub fn get_pause_info(deps: Deps) -> Result<QueryResponse, ContractError> {
    Ok(to_binary(&PauseInfoResponse {
        paused: PAUSE.load(deps.storage)?,
    })?)
}

pub fn get_owner_info(deps: Deps) -> Result<QueryResponse, ContractError> {
    Ok(to_binary(&OwnerResponse {
        owner: hpl_ownable::OWNER.load(deps.storage)?.to_string(),
    })?)
}

pub fn quote_dispatch(
    deps: Deps,
    _env: Env,
    msg: PostDispatchQueryMsg,
) -> Result<QueryResponse, ContractError> {
    match msg {
        PostDispatchQueryMsg::QuoteDispatch { metadata, message } => {
            let hpl_msg: Message = message.clone().into();
            let target_contract = HOOK_CONFIG
                .load(deps.storage, hpl_msg.dest_domain)
                .map_err(|_| ContractError::HookNotRegistered(hpl_msg.dest_domain))?;

            let result: QuoteGasPaymentResponse = deps.querier.query_wasm_smart(
                target_contract.hook,
                &PostDispatchQueryMsg::QuoteDispatch { metadata, message },
            )?;

            Ok(to_binary(&QuoteGasPaymentResponse {
                gas_needed: result.gas_needed,
            })?)
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        Addr, HexBinary, Uint256,
    };
    use hpl_interface::hook::HookConfig;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_get_pause_info() {
        let mut deps = mock_dependencies();

        // Pause
        PAUSE.save(deps.as_mut().storage, &true).unwrap();
        let paused_query = get_pause_info(deps.as_ref()).unwrap();

        assert_eq!(
            paused_query,
            to_binary(&PauseInfoResponse { paused: true }).unwrap()
        );

        // Unpause
        PAUSE.save(deps.as_mut().storage, &false).unwrap();
        let unpaused_query = get_pause_info(deps.as_ref()).unwrap();

        assert_eq!(
            unpaused_query,
            to_binary(&PauseInfoResponse { paused: false }).unwrap()
        )
    }

    #[test]
    fn test_get_owner_info() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked("owner");
        hpl_ownable::OWNER
            .save(deps.as_mut().storage, &owner)
            .unwrap();

        let owner_query = get_owner_info(deps.as_ref()).unwrap();
        assert_eq!(
            owner_query,
            to_binary(&OwnerResponse {
                owner: owner.to_string()
            })
            .unwrap()
        )
    }

    #[rstest]
    #[case(Addr::unchecked("osmo109ns4u04l44kqdkvp876hukd3hxz8zzm7809el"))]
    #[case(Addr::unchecked("neutron1d6a3j0kkpc8eac0j8h6ypyevfz8hd3qnsyg35p"))]
    fn test_quote_dispatch(#[case] hook: Addr) {
        let mut deps = mock_dependencies();

        deps.querier.update_wasm(
            |_| -> cosmwasm_std::SystemResult<cosmwasm_std::ContractResult<cosmwasm_std::Binary>> {
                let res = cosmwasm_std::ContractResult::Ok(
                    to_binary(&QuoteGasPaymentResponse {
                        gas_needed: Uint256::from_str("1000000").unwrap(),
                    })
                    .unwrap(),
                );

                cosmwasm_std::SystemResult::Ok(res)
            },
        );

        let hook_config = HookConfig {
            hook,
            destination: 11155111,
        };
        HOOK_CONFIG
            .save(deps.as_mut().storage, 11155111, &hook_config)
            .unwrap();

        let binary_message = HexBinary::from_hex("00000021500000aef3000000000000000000000000477d860f8f41bc69ddd32821f2bf2c2af0243f1600aa36a70000000000000000000000005d56b8a669f50193b54319442c6eee5edd66238148656c6c6f21").unwrap();
        let dummy_metadata = HexBinary::from_hex("deadbeefc0ffee").unwrap();

        let res = quote_dispatch(
            deps.as_ref(),
            mock_env(),
            PostDispatchQueryMsg::QuoteDispatch {
                metadata: dummy_metadata,
                message: binary_message,
            },
        )
        .unwrap();

        assert_eq!(
            res,
            to_binary(&QuoteGasPaymentResponse {
                gas_needed: Uint256::from_str("1000000").unwrap(),
            })
            .unwrap()
        );
    }
}
