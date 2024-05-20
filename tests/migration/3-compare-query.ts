import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { StargateClient } from '@cosmjs/stargate';
import { readFileSync, writeFileSync } from 'fs';
import _ from 'lodash';

import { endpoint } from './deps';
import { Snapshot, makeSnapshot, resultPath } from './shared';

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

  for (const { contract, address, results } of newSnapshot) {
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
}

main().catch(console.error);
