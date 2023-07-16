use cosmwasm_std::{DepsMut, Event, MessageInfo, Response, StdResult};
use hpl_interface::ism::multisig::ThresholdSet;

use crate::{
    event::emit_set_threshold,
    state::{assert_owned, THRESHOLD},
    ContractError,
};

pub fn set_threshold(
    deps: DepsMut,
    info: MessageInfo,
    threshold: ThresholdSet,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender)?;
    THRESHOLD.save(deps.storage, threshold.domain, &threshold.threshold)?;

    Ok(Response::new().add_event(emit_set_threshold(threshold.domain, threshold.threshold)))
}

pub fn set_thresholds(
    deps: DepsMut,
    info: MessageInfo,
    thresholds: Vec<ThresholdSet>,
) -> Result<Response, ContractError> {
    assert_owned(deps.storage, info.sender)?;

    let events: Vec<Event> = thresholds
        .into_iter()
        .map(|v| {
            THRESHOLD.save(deps.storage, v.domain, &v.threshold)?;
            Ok(emit_set_threshold(v.domain, v.threshold))
        })
        .collect::<StdResult<_>>()?;

    Ok(Response::new().add_events(events.into_iter()))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr, Storage,
    };

    use crate::state::{Config, CONFIG};

    use super::*;
    const ADDR1_VAULE: &str = "addr1";
    const ADDR2_VAULE: &str = "addr2";

    fn mock_owner(storage: &mut dyn Storage, owner: Addr) {
        let config = Config {
            owner,
            addr_prefix: String::new(),
        };

        CONFIG.save(storage, &config).unwrap();
    }

    #[test]
    fn test_set_threshold() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked(ADDR1_VAULE);
        mock_owner(deps.as_mut().storage, owner.clone());

        let threshold = ThresholdSet {
            domain: 1u32,
            threshold: 8u8,
        };

        // set_threshold failure test
        let info = mock_info(ADDR2_VAULE, &[]);
        let fail_result = set_threshold(deps.as_mut(), info, threshold.clone()).unwrap_err();

        assert!(matches!(fail_result, ContractError::Unauthorized {}));

        // set_threshold success test
        let info = mock_info(owner.as_str(), &[]);
        let result = set_threshold(deps.as_mut(), info, threshold.clone()).unwrap();

        assert_eq!(
            result.events,
            vec![emit_set_threshold(threshold.domain, threshold.threshold)]
        );

        // check it actually saved
        let saved_threshold = THRESHOLD.load(&deps.storage, threshold.domain).unwrap();
        assert_eq!(saved_threshold, threshold.threshold);
    }

    #[test]
    fn test_set_thresholds() {
        let mut deps = mock_dependencies();
        let owner = Addr::unchecked(ADDR1_VAULE);
        mock_owner(deps.as_mut().storage, owner.clone());

        let thresholds: Vec<ThresholdSet> = vec![
            ThresholdSet {
                domain: 1u32,
                threshold: 8u8,
            },
            ThresholdSet {
                domain: 2u32,
                threshold: 7u8,
            },
            ThresholdSet {
                domain: 3u32,
                threshold: 6u8,
            },
        ];

        // set_threshold failure test
        let info = mock_info(ADDR2_VAULE, &[]);
        let fail_result = set_thresholds(deps.as_mut(), info, thresholds.clone()).unwrap_err();

        assert!(matches!(fail_result, ContractError::Unauthorized {}));

        // set_threshold success test
        let info = mock_info(owner.as_str(), &[]);
        let result = set_thresholds(deps.as_mut(), info, thresholds.clone()).unwrap();

        assert_eq!(
            result.events,
            vec![
                emit_set_threshold(1u32, 8u8),
                emit_set_threshold(2u32, 7u8),
                emit_set_threshold(3u32, 6u8),
            ]
        );

        // check it actually saved
        for threshold in thresholds {
            let saved_threshold = THRESHOLD.load(&deps.storage, threshold.domain).unwrap();
            assert_eq!(saved_threshold, threshold.threshold);
        }
    }
}
