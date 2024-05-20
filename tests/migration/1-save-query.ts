import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { StargateClient } from '@cosmjs/stargate';

import { endpoint } from './deps';
import { QUERIES } from './queries';

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
  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
  };

  for (const target of migrationTargets) {
    const found = QUERIES.find((v) => v.contract === target.name);
    if (!found) throw new Error(`No queries found for ${target.name}`);

    console.log(`Processing ${target.name}...`);

    for (const oneof of target.address) {
      console.log(`=> CONTRACT: ${oneof}`);

      for (const query of found.queries) {
        console.log(`==> QUERING: ${JSON.stringify(query)}`);

        const resp = await client.wasm.queryContractSmart(oneof, query);
        console.log(resp);
      }
    }
  }
}

main().catch(console.error);
