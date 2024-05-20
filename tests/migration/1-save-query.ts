import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { StargateClient } from '@cosmjs/stargate';
import { writeFileSync } from 'fs';

import { endpoint } from './deps';
import { makeSnapshot } from './shared';

async function main() {
  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
  };

  const snapshot = await makeSnapshot(client);

  writeFileSync('snapshot-prev.json', JSON.stringify(snapshot, null, 2));
  console.log('Snapshot saved to snapshot-prev.json');
}

main().catch(console.error);
