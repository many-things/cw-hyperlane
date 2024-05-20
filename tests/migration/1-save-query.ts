import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { keccak256 } from '@cosmjs/crypto';
import { StargateClient } from '@cosmjs/stargate';
import { writeFileSync } from 'fs';

import { endpoint } from './deps';
import { migrationTargets } from './migration';
import { QUERIES } from './queries';

async function main() {
  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
  };

  const snapshot = [];

  for (const contract of migrationTargets) {
    const found = QUERIES.find((v) => v.contract === contract.name);
    if (!found) throw new Error(`No queries found for ${contract.name}`);

    console.log(`Processing ${contract.name}...`);

    for (const target of contract.address) {
      console.log(`=> CONTRACT: ${target}`);

      const results: {
        id: string;
        query: object;
        response?: object;
        error?: unknown;
      }[] = [];

      for (const query of found.queries) {
        console.log(`==> QUERING: ${JSON.stringify(query)}`);

        const queryIdRaw = keccak256(Buffer.from(JSON.stringify(query)));
        const queryId = Buffer.from(queryIdRaw).toString('hex');

        try {
          const resp = await client.wasm.queryContractSmart(target, query);
          console.log(JSON.stringify(resp));

          results.push({
            id: queryId,
            query,
            response: resp,
          });
        } catch (e) {
          console.error(e);
          results.push({ id: queryId, query, error: e });
        }
      }

      snapshot.push({
        contract: contract.name,
        address: target,
        results,
      });
    }
  }

  writeFileSync('snapshot-prev.json', JSON.stringify(snapshot, null, 2));
  console.log('Snapshot saved to snapshot-prev.json');
}

main().catch(console.error);
