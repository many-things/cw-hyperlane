import { CosmWasmClient, setupWasmExtension } from '@cosmjs/cosmwasm-stargate';
import { QueryClient, StargateClient } from '@cosmjs/stargate';
import { connectComet } from '@cosmjs/tendermint-rpc';
import { writeFileSync } from 'fs';

import { endpoint } from './deps';
import { makeSnapshot, resultPath } from './shared';

async function main() {
  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
    stateClient: QueryClient.withExtensions(
      await connectComet(endpoint.rpc),
      setupWasmExtension,
    ),
  };

  const snapshot = await makeSnapshot(client);

  writeFileSync(
    resultPath('snapshot-prev.json'),
    JSON.stringify(snapshot, null, 2),
  );
  console.log(`Snapshot saved to ${resultPath('snapshot-prev.json')}`);
}

main().catch(console.error);
