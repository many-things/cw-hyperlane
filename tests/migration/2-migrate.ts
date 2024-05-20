import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { StargateClient } from '@cosmjs/stargate';
import { existsSync, readFileSync, readdirSync, writeFileSync } from 'fs';

import {
  MULTISIG,
  MULTISIG_MEMBERS,
  MULTISIG_PRE_PROP,
  MULTISIG_PROP_MODULE,
  RUNNER,
  endpoint,
} from './deps';
import { migrationTargets } from './migration';
import { Multisig } from './multisig';
import { makeMember, resultPath, uploadContract } from './shared';

function loadArtifacts(): string[] {
  return readdirSync(`${process.cwd()}/artifacts`).filter((v) =>
    v.endsWith('.wasm'),
  );
}

async function main() {
  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
  };

  const runner = await makeMember(client, RUNNER);

  const multisig = await Multisig.create({
    address: {
      hub: MULTISIG,
      prop: MULTISIG_PROP_MODULE,
      preProp: MULTISIG_PRE_PROP,
    },
    members: MULTISIG_MEMBERS,
    client,
  });

  // upload new contract codes

  let codes: { name: string; codeId: number }[] = [];

  if (existsSync(resultPath('uploaded.json'))) {
    codes = JSON.parse(readFileSync(resultPath('uploaded.json'), 'utf-8'));
    console.log(
      `loaded uploaded contracts from ${resultPath('uploaded.json')}`,
    );
  } else {
    for (const artifact of loadArtifacts()) {
      const resp = await uploadContract(
        runner,
        `${process.cwd()}/artifacts/${artifact}`,
      );

      const codeId = Number(
        resp.events
          .filter((v) => v.type === 'store_code')[0]
          .attributes.find((v) => v.key === 'code_id')!.value,
      );

      console.log(`code uploaded. ${resp.hash} ${codeId} ${artifact}`);
      codes.push({ name: artifact.replace('.wasm', ''), codeId });
    }

    writeFileSync(resultPath('uploaded.json'), JSON.stringify(codes, null, 2));
    console.log(
      `uploaded all contracts => exported to ${resultPath('uploaded.json')}`,
    );
  }

  // run migration

  await multisig.run(
    migrationTargets.flatMap((info) =>
      info.address.map((addr) => ({
        wasm: {
          migrate: {
            contract_addr: addr,
            new_code_id: codes.find((c) => c.name === info.name)?.codeId,
            msg: Buffer.from(JSON.stringify({})).toString('base64'),
          },
        },
      })),
    ),
  );
}

main().catch(console.error);
