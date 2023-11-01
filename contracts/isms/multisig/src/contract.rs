#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure_eq, Deps, DepsMut, Empty, Env, MessageInfo, QueryResponse, Response, StdResult,
};
use cw2::set_contract_version;
use hpl_interface::{
    ism::{
        multisig::{
            EnrolledValidatorsResponse, ExecuteMsg, InstantiateMsg, ListEnrolledValidatorsResponse,
            MultisigIsmQueryMsg, QueryMsg, ValidatorSet,
        },
        IsmQueryMsg,
    },
    to_binary,
};
use hpl_ownable::get_owner;

use crate::{error::ContractError, new_event, CONTRACT_NAME, CONTRACT_VERSION, VALIDATORS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;

    hpl_ownable::initialize(deps.storage, &owner)?;

    Ok(Response::new().add_event(
        new_event("instantiate")
            .add_attribute("sender", info.sender)
            .add_attribute("owner", owner),
    ))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Ownable(msg) => Ok(hpl_ownable::handle(deps, env, info, msg)?),

        UpdateValidatorSet { set } => update_validator_set(deps, info, set),
        UnsetDomain { domain } => unset_domain(deps, info, domain),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    use crate::query;
    use IsmQueryMsg::*;

    match msg {
        QueryMsg::Ownable(msg) => Ok(hpl_ownable::handle_query(deps, env, msg)?),
        QueryMsg::Ism(msg) => match msg {
            ModuleType {} => to_binary(query::get_module_type()),
            Verify {
                metadata: raw_metadata,
                message: raw_message,
            } => to_binary(query::verify_message(deps, raw_metadata, raw_message)),
            VerifyInfo {
                message: raw_message,
            } => to_binary(query::get_verify_info(deps, raw_message)),
        },
        QueryMsg::MultisigIsm(msg) => match msg {
            MultisigIsmQueryMsg::EnrolledValidators { domain } => Ok(cosmwasm_std::to_binary(
                &to_resp(VALIDATORS.load(deps.storage, domain)?),
            )?),
            MultisigIsmQueryMsg::ListEnrolledValidators {
                offset,
                limit,
                order,
            } => {
                let ((min, max), limit, order) = hpl_interface::range_option(offset, limit, order)?;

                let resp = ListEnrolledValidatorsResponse {
                    validators: VALIDATORS
                        .range(deps.storage, min, max, order.into())
                        .take(limit)
                        .map(|x| x.map(|(k, v)| (k, to_resp(v))))
                        .collect::<StdResult<_>>()?,
                };

                Ok(cosmwasm_std::to_binary(&resp)?)
            }
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    crate::migration::migrate(deps)
}

fn to_resp(v: crate::ValidatorSet) -> EnrolledValidatorsResponse {
    EnrolledValidatorsResponse {
        validators: v.validators,
        threshold: v.threshold,
    }
}

pub fn update_validator_set(
    deps: DepsMut,
    info: MessageInfo,
    set: ValidatorSet,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
        ContractError::Unauthorized
    );

    set.validate()?;

    VALIDATORS.save(
        deps.storage,
        set.domain,
        &crate::ValidatorSet {
            validators: set.validators.clone(),
            threshold: set.threshold,
        },
    )?;

    Ok(Response::new().add_event(
        new_event("update_validator_set")
            .add_attribute("sender", info.sender)
            .add_attribute("validators", set.validators.len().to_string())
            .add_attribute("threshold", set.threshold.to_string()),
    ))
}

pub fn unset_domain(
    deps: DepsMut,
    info: MessageInfo,
    domain: u32,
) -> Result<Response, ContractError> {
    ensure_eq!(
        info.sender,
        get_owner(deps.storage)?,
        ContractError::Unauthorized
    );

    VALIDATORS.remove(deps.storage, domain);

    Ok(Response::new().add_event(
        new_event("unset_domain")
            .add_attribute("sender", info.sender)
            .add_attribute("domain", domain.to_string()),
    ))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::mock_dependencies;
    use hpl_interface::{build_test_executor, ism::multisig::ValidatorSet};
    use ibcx_test_utils::{addr, gen_bz};
    use rstest::rstest;

    build_test_executor!(super::execute);

    #[rstest]
    #[case("owner", "owner")]
    #[should_panic(expected = "unauthorized")]
    #[case("user", "owner")]
    #[should_panic(expected = "unauthorized")]
    #[case("owner", "user")]
    #[should_panic(expected = "unauthorized")]
    #[case("user", "user")]
    fn test_update_validator_set(#[case] setter: &str, #[case] remover: &str) {
        let mut deps = mock_dependencies();

        hpl_ownable::initialize(deps.as_mut().storage, &addr("owner")).unwrap();

        let set = ValidatorSet {
            validators: vec![gen_bz(20), gen_bz(20), gen_bz(20)],
            threshold: 2,
            domain: 1,
        };

        test_execute(
            deps.as_mut(),
            &addr(setter),
            super::ExecuteMsg::UpdateValidatorSet { set: set.clone() },
            vec![],
        );

        let updated_set = crate::VALIDATORS.load(deps.as_ref().storage, 1).unwrap();

        assert_eq!(updated_set.validators, set.validators);
        assert_eq!(updated_set.threshold, set.threshold);

        test_execute(
            deps.as_mut(),
            &addr(remover),
            super::ExecuteMsg::UnsetDomain { domain: 1 },
            vec![],
        );

        assert!(!crate::VALIDATORS.has(deps.as_ref().storage, 1));
    }
}
