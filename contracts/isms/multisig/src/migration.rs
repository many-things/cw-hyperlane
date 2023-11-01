use cosmwasm_std::{DepsMut, HexBinary, Order, Response, StdResult};
use cw_storage_plus::Map;

use crate::{new_event, ContractError, VALIDATORS};

const LEGACY_VALIDATORS_PREFIX: &str = "validators";
const LEGACY_VALIDATORS: Map<u32, Vec<HexBinary>> = Map::new(LEGACY_VALIDATORS_PREFIX);

const LEGACY_THRESHOLD_PREFIX: &str = "threshold";
const LEGACY_THRESHOLD: Map<u32, u8> = Map::new(LEGACY_THRESHOLD_PREFIX);

pub fn migrate(deps: DepsMut) -> Result<Response, ContractError> {
    let legacy_validators = LEGACY_VALIDATORS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|x| {
            let (k, v) = x?;

            let t = LEGACY_THRESHOLD.load(deps.storage, k)?;

            Ok((k, v, t))
        })
        .collect::<StdResult<Vec<_>>>()?;

    let mut events = vec![];

    for (domain, validators, threshold) in legacy_validators {
        LEGACY_VALIDATORS.remove(deps.storage, domain);
        LEGACY_THRESHOLD.remove(deps.storage, domain);

        VALIDATORS.save(
            deps.storage,
            domain,
            &crate::ValidatorSet {
                validators: validators.clone(),
                threshold,
            },
        )?;

        events.push(
            new_event("migration::validator_set")
                .add_attribute("domain", domain.to_string())
                .add_attribute("validators", validators.len().to_string())
                .add_attribute("threshold", threshold.to_string()),
        )
    }

    Ok(Response::new().add_event(new_event("migration")))
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::mock_dependencies;
    use ibcx_test_utils::gen_bz;

    use super::*;

    #[test]
    fn test_migration() {
        let mut deps = mock_dependencies();

        let set = (1u32..25u32)
            .map(|v| {
                (
                    v,
                    (0..v).map(|_| gen_bz(20)).collect::<Vec<_>>(),
                    (v / 2) as u8,
                )
            })
            .collect::<Vec<_>>();

        for (domain, validators, threshold) in set.clone() {
            LEGACY_VALIDATORS
                .save(deps.as_mut().storage, domain, &validators)
                .unwrap();

            LEGACY_THRESHOLD
                .save(deps.as_mut().storage, domain, &threshold)
                .unwrap();
        }

        super::migrate(deps.as_mut()).unwrap();

        for (domain, validators, threshold) in set {
            let new_valset = VALIDATORS.load(deps.as_ref().storage, domain).unwrap();

            assert_eq!(new_valset.validators, validators);
            assert_eq!(new_valset.threshold, threshold);
        }
    }
}
