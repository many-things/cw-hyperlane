import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { StargateClient } from '@cosmjs/stargate';
import { readFileSync } from 'fs';
import _ from 'lodash';

import { endpoint } from './deps';
import { Snapshot, makeSnapshot } from './shared';

function loadSnapshot(): Snapshot {
  return JSON.parse(readFileSync('snapshot-prev.json', 'utf-8'));
}

async function main() {
  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
  };

  const prevSnapshot = loadSnapshot();
  const newSnapshot = await makeSnapshot(client);

  console.log('Comparing snapshots...');

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

      const isEqual = _.isEqual(result, prevResult);
      if (isEqual) {
        console.log(`[${contract}] ${address} ${result.id} is equal`);
      } else {
        console.log(`[${contract}] ${address} ${result.id} is different`);
        console.log('Prev:', prevResult);
        console.log('New:', result);
      }
    }
  }
}

main().catch(console.error);
