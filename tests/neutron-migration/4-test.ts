import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { StargateClient } from '@cosmjs/stargate';

import { TIA_WHALE, endpoint } from './deps';
import { executeContract } from './shared';

async function main() {
  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
  };

  const resp = await executeContract(
    { address: TIA_WHALE, client },
    'neutron1jyyjd3x0jhgswgm6nnctxvzla8ypx50tew3ayxxwkrjfxhvje6kqzvzudq',
    {
      transfer_remote: {
        amount: `${1_000_000}`,
        dest_domain: 42161, // arbitrum
        recipient:
          '0000000000000000000000004264c2ebd0af02798259ffeb2691d19190dd1bcb',
      },
    },
    [
      {
        denom:
          'ibc/773B4D0A3CD667B2275D5A4A7A2F0909C0BA0F4059C0B9181E680DDF4965DCC7',
        amount: `${1_270_000}`,
      },
    ],
  );
  if (resp.code !== 0) throw new Error(`Tx failed: ${resp.rawLog}`);

  const queue = [
    'wasm-hpl_warp_native::transfer-remote',
    'wasm-mailbox_dispatch_id',
    'wasm-mailbox_dispatch',
    'wasm-igp-core-pay-for-gas',
    'wasm-igp-core-post-dispatch',
    'wasm-hpl_hook_merkle::post_dispatch',
    'wasm-hpl_hook_merkle::inserted_into_tree',
  ];
  resp.events.forEach((evt) => queue[0] === evt.type && queue.shift());

  if (queue.length > 0) {
    console.log('Expected events not flushed');
    console.log(`=> left: ${queue}`);
    throw Error('Transfer failed');
  }

  console.log('Transfer completed');
}

main().catch(console.error);
