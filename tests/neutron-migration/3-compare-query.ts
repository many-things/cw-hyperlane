import { CosmWasmClient, setupWasmExtension } from '@cosmjs/cosmwasm-stargate';
import { QueryClient, StargateClient } from '@cosmjs/stargate';
import { readFileSync, writeFileSync } from 'fs';
import _ from 'lodash';

import { endpoint } from './deps';
import { Snapshot, makeSnapshot, resultPath } from './shared';
import { connectComet } from '@cosmjs/tendermint-rpc';

function loadPrevSnapshot(): Snapshot {
  return JSON.parse(readFileSync(resultPath('snapshot-prev.json'), 'utf-8'));
}

function loadNewSnapshot(): Snapshot {
  return JSON.parse(readFileSync(resultPath('snapshot-new.json'), 'utf-8'));
}

async function main() {
  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
    stateClient: QueryClient.withExtensions(await connectComet(endpoint.rpc), setupWasmExtension),
  };

  writeFileSync(
    resultPath('snapshot-new.json'),
    JSON.stringify(await makeSnapshot(client), null, 2),
  );
  console.log(`Snapshot saved to ${resultPath('snapshot-new.json')}`);

  const prevSnapshot = loadPrevSnapshot();
  const newSnapshot = loadNewSnapshot();

  console.log('Comparing snapshots...');

  const compareResults = [];
  const compareState   = [];

  for (const { contract, address, results, state } of newSnapshot) {
    const compareTarget = prevSnapshot.find(
      (v) => v.contract === contract && v.address === address,
    );
    if (!compareTarget)
      throw Error(`No previous snapshot found for ${contract} at ${address}`);

    for (const result of results) {
      const prevResult = compareTarget.results.find((v) => v.id === result.id);
      if (!prevResult)
        throw Error(`No previous prevResult found for ${result.id}`);

      compareResults.push({
        id: result.id,
        contract,
        address,
        query: JSON.stringify(result.query),
        diff: !_.isEqual(result, prevResult)
          ? {
              prev: prevResult,
              new: result,
            }
          : undefined,
      });
    }

    for (const state_kv of state) {
      const prevStateKv = compareTarget.state.find((kv) => kv.key === state_kv.key);
      if (!prevStateKv)
        throw Error(`No previous prevStateKv found for ${state_kv.key}`);

      compareState.push({
        key: prevStateKv.key,
        contract,
        address,
        state_diff: !_.isEqual(state_kv, prevStateKv)
        ? {
            prev: prevStateKv,
            new: state_kv,
          }
        : undefined,
      })
    }
  }

  writeFileSync(
    resultPath('compare-results.json'),
    JSON.stringify(compareResults, null, 2),
  );
  console.log(`Compare results saved to ${resultPath('compare-results.json')}`);

  writeFileSync(
    resultPath('compare-results.diff.json'),
    JSON.stringify(
      compareResults.filter((v) => v.diff !== undefined),
      null,
      2,
    ),
  );
  console.log(
    `Diff results saved to ${resultPath('compare-results.diff.json')}`,
  );

  writeFileSync(
    resultPath('compare-state.json'),
    JSON.stringify(compareState, null, 2),
  );
  console.log(`Compare state saved to ${resultPath('compare-state.json')}`);

  writeFileSync(
    resultPath('compare-state.diff.json'),
    JSON.stringify(
      compareState.filter((v) => v.state_diff !== undefined),
      null,
      2,
    ),
  );
  console.log(
    `Diff state saved to ${resultPath('compare-state.diff.json')}`,
  );
}

main().catch(console.error);
