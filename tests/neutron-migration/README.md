# Neutron Contract Migration Tests

This directory contains tests for the Neutron contract migration.

> Migration target contracts must be after [this commit](https://github.com/many-things/cw-hyperlane/commit/00f64a9e8df5b384c2648d3f0f38baa549f204b3) which enables semver based migration and applies to the migration logic. + Some of the contracts does not have the migration logic yet on the v0.0.6 release.

## Before running tests

1. Need to create Cradle account to make forknet of the Neutron Mainnet. (<https://app.newmetric.xyz/>)
2. Need to build this project and make sure the whole contracts are stored in the 'artifacts' directory by running `make optimize` in the root directory.
   - Or you can download the artifacts from the [release page](https://github.com/many-things/cw-hyperlane/releases/tag/v0.0.6)

## Running tests

1. Add the Neutron forknet info to `config.yaml` file.

   ```yaml
   - id: neutron-1-fork
   hrp: neutron
   is_cradle: true
   cradle_session_id: { { cradle_session_id } } # replace me
   gas:
       price: 0.025
       denom: untrn
   domain: 1853125230
   ```

2. Remove the pre-existing results of the migration test in the `results` directory.

3. Run the scripts one by one.

   ```bash
   yarn tsx ./tests/migration/0-init.ts

   yarn tsx ./tests/migration/1-save-query.ts

   yarn tsx ./tests/migration/2-migrate.ts

   yarn tsx ./tests/migration/3-compare-query.ts

   yarn tsx ./tests/migration/4-test.ts
   ```

   1. Step 1. Initialize the test environment.
      - This script will sends tokens that are required for the migration test to the test account.
   2. Step 2. Query as much as possible and save the results.

   3. Step 3. Migrate the Neutron contracts.

      - This script mimics the Neutron multisig contract account.

   4. Step 4. Compare the query results before and after the migration.

## After Running tests

- Check the results in the `results` directory. Specifically, check the `compare-results.diff.json` file to see if there are any differences between the query results before and after the migration.
  - It should be contains only the diff about the `hpl_merkle_hook` contract which is the `ownable` logic removed.
