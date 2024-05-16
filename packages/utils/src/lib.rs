use cosmwasm_std::{ensure_eq, StdError, Storage};
use semver::Version;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MigrationError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

impl From<semver::Error> for MigrationError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}

pub fn migrate(
    storage: &mut dyn Storage,
    contract_name: &str,
    contract_version: &str,
) -> Result<(), MigrationError> {
    let stored = cw2::get_contract_version(storage)?;

    ensure_eq!(
        stored.contract,
        contract_name,
        StdError::generic_err("contract name mismatch")
    );

    let version: Version = contract_version.parse()?;
    let stored_version: Version = stored.version.parse()?;

    if stored_version < version {
        Ok(cw2::set_contract_version(
            storage,
            contract_name,
            contract_version,
        )?)
    } else {
        Err(StdError::generic_err("invalid version").into())
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::mock_dependencies, StdError, Storage};

    use crate::migrate;

    fn assert_version(storage: &dyn Storage, contract: &str, version: &str) {
        let stored = cw2::get_contract_version(storage).unwrap();

        assert_eq!(stored.contract, contract);
        assert_eq!(stored.version, version);
    }

    #[test]
    fn test_migrate() {
        let mut deps = mock_dependencies();
        cw2::set_contract_version(&mut deps.storage, "hello", "0.0.6-rc8").unwrap();

        migrate(&mut deps.storage, "nono", "0.0.6-rc8")
            .expect_err(&StdError::generic_err("contract name mismatch").to_string());

        migrate(&mut deps.storage, "hello", "0.0.5")
            .expect_err(&StdError::generic_err("invalid version").to_string());

        migrate(&mut deps.storage, "hello", "0.0.6").unwrap();
        assert_version(&deps.storage, "hello", "0.0.6");
    }
}
