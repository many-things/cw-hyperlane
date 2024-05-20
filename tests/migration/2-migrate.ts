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
  TIA_WHALE,
  denom,
  endpoint,
} from './deps';
import { Multisig } from './multisig';
import { makeMember, sendTx, uploadContract } from './shared';

const migrationTargets = [
  {
    name: 'hpl_mailbox',

    address: [
      'neutron1sjzzd4gwkggy6hrrs8kxxatexzcuz3jecsxm3wqgregkulzj8r7qlnuef4',
    ],
  },
  {
    name: 'hpl_validator_announce',

    address: [
      'neutron17w4q6efzym3p4c6umyp4cjf2ustjtmwfqdhd7rt2fpcpk9fmjzsq0kj0f8',
    ],
  },
  {
    name: 'hpl_ism_multisig',

    address: [
      'neutron1q75ky8reksqzh0lkhk9k3csvjwv74jjquahrj233xc7dvzz5fv4qtvw0qg',
    ],
  },
  {
    name: 'hpl_hook_merkle',

    address: [
      'neutron1e5c2qqquc86rd3q77aj2wyht40z6z3q5pclaq040ue9f5f8yuf7qnpvkzk',
    ],
  },
  {
    name: 'hpl_igp',

    address: [
      'neutron1ww9yg48qvmpmedyvkrcrmjsudxeu840l5n6ywqykpqlsdq9pdxkqg2zq7e',
    ],
  },
  {
    name: 'hpl_igp_oracle',

    address: [
      'neutron1sjxus3rynpwq0ncnm0m0dfun9x3flwmalsmveh4kuml0650wsq4q8n4mus',
    ],
  },
  {
    name: 'hpl_warp_native',

    address: [
      'neutron1jyyjd3x0jhgswgm6nnctxvzla8ypx50tew3ayxxwkrjfxhvje6kqzvzudq',
      'neutron1ch7x3xgpnj62weyes8vfada35zff6z59kt2psqhnx9gjnt2ttqdqtva3pa',
    ],
  },
  {
    name: 'hpl_test_mock_msg_receiver',

    address: [
      'neutron1v0t8nztzhdsan2cv23x66xjrak9844zz9hq7gkz0w4j6xl4lmpzq89kt5g',
    ],
  },
];

async function main() {
  const initialAccounts = [RUNNER, TIA_WHALE, ...MULTISIG_MEMBERS];

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

  const initFundingResp = await sendTx({ address: NEUTRON_WHALE, client }, [
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
  console.log('initial funding', initFundingResp.transactionHash);

  // upload new contract codes

  const codes: { name: string; codeId: number }[] = [];

  for (const artifact of artifacts) {
    const resp = await uploadContract(runner, `./artifacts/${artifact}`);

    const codeId = Number(
      resp.events
        .filter((v) => v.type === 'store_code')[0]
        .attributes.find((v) => v.key === 'code_id')!.value,
    );

    console.log(`code uploaded. ${resp.hash} ${codeId} ${artifact}`);
    codes.push({ name: artifact.replace('.wasm', ''), codeId });
  }
  writeFileSync('./uploaded.json', JSON.stringify(codes, null, 2));
  console.log(`uploaded all contracts => exported to ./uploaded.json`);

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
