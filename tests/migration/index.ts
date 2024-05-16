import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { MsgSendEncodeObject, StargateClient } from '@cosmjs/stargate';
import { readdirSync, writeFileSync } from 'fs';

import {
  MULTISIG,
  MULTISIG_MEMBERS,
  MULTISIG_PRE_PROP,
  MULTISIG_PROP_MODULE,
  NEUTRON_WHALE,
  RUNNER,
  denom,
  endpoint,
} from './deps';
import { Multisig } from './multisig';
import { makeMember, sendTx, uploadContract } from './shared';

const migrationTargets = [
  {
    name: 'hpl_mailbox',
    codeId: 416,
    address: [
      'neutron1sjzzd4gwkggy6hrrs8kxxatexzcuz3jecsxm3wqgregkulzj8r7qlnuef4',
    ],
  },
  {
    name: 'hpl_validator_announce',
    codeId: 432,
    address: [
      'neutron17w4q6efzym3p4c6umyp4cjf2ustjtmwfqdhd7rt2fpcpk9fmjzsq0kj0f8',
    ],
  },
  {
    name: 'hpl_ism_multisig',
    codeId: 431,
    address: [
      'neutron1q75ky8reksqzh0lkhk9k3csvjwv74jjquahrj233xc7dvzz5fv4qtvw0qg',
    ],
  },
  {
    name: 'hpl_hook_merkle',
    codeId: 406,
    address: [
      'neutron1e5c2qqquc86rd3q77aj2wyht40z6z3q5pclaq040ue9f5f8yuf7qnpvkzk',
    ],
  },
  {
    name: 'hpl_igp',
    codeId: 433,
    address: [
      'neutron1ww9yg48qvmpmedyvkrcrmjsudxeu840l5n6ywqykpqlsdq9pdxkqg2zq7e',
    ],
  },
  {
    name: 'hpl_igp_oracle',
    codeId: 412,
    address: [
      'neutron1sjxus3rynpwq0ncnm0m0dfun9x3flwmalsmveh4kuml0650wsq4q8n4mus',
    ],
  },
  {
    name: 'hpl_warp_native',
    codeId: 421,
    address: [
      'neutron1jyyjd3x0jhgswgm6nnctxvzla8ypx50tew3ayxxwkrjfxhvje6kqzvzudq',
      'neutron1ch7x3xgpnj62weyes8vfada35zff6z59kt2psqhnx9gjnt2ttqdqtva3pa',
    ],
  },
  {
    name: 'hpl_test_mock_msg_receiver',
    codeId: 418,
    address: [
      'neutron1v0t8nztzhdsan2cv23x66xjrak9844zz9hq7gkz0w4j6xl4lmpzq89kt5g',
    ],
  },
];

async function main() {
  const initialAccounts = [RUNNER, ...MULTISIG_MEMBERS];

  const artifacts = readdirSync('./artifacts').filter((v) =>
    v.endsWith('.wasm'),
  );

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

  // initial funding

  await sendTx({ address: NEUTRON_WHALE, client }, [
    ...initialAccounts.map(
      (toAddress) =>
        ({
          typeUrl: '/cosmos.bank.v1beta1.MsgSend',
          value: {
            fromAddress: NEUTRON_WHALE,
            toAddress,
            amount: [{ denom, amount: `${100_000_000}` }],
          },
        }) as MsgSendEncodeObject,
    ),
  ]);

  // upload new contract codes

  const codes: { name: string; codeId: number }[] = [];

  for (const artifact of artifacts) {
    const resp = await uploadContract(runner, `./artifacts/${artifact}`);
    console.log(`uploaded ${artifact}: ${resp.hash}`);

    const codeId = Number(
      resp.events
        .filter((v) => v.type === 'store_code')[0]
        .attributes.find((v) => v.key === 'code_id')!.value,
    );

    codes.push({ name: artifact.replace('.wasm', ''), codeId });
  }
  writeFileSync('./uploaded.json', JSON.stringify(codes, null, 2));
  console.log(`uploaded all contracts => exported to ./uploaded.json`);

  // run migration

  await multisig.run(
    migrationTargets.map((info) =>
      info.address.map((addr) => ({
        wasm: {
          contract_addr: addr,
          new_code_id: codes.find((c) => c.name === info.name),
          msg: Buffer.from(JSON.stringify({}), 'base64'),
        },
      })),
    ),
  );

  // dispatch random message

  // check validator
}

main().catch(console.error);
