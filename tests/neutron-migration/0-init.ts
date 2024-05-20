import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { MsgSendEncodeObject, StargateClient } from '@cosmjs/stargate';

import {
  MULTISIG_MEMBERS,
  NEUTRON_WHALE,
  RUNNER,
  TIA_WHALE,
  denom,
  endpoint,
} from './deps';
import { sendTx } from './shared';

async function main() {
  const initialAccounts = [RUNNER, TIA_WHALE, ...MULTISIG_MEMBERS];

  const client = {
    wasm: await CosmWasmClient.connect(endpoint.rpc),
    stargate: await StargateClient.connect(endpoint.rpc),
  };

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
}

main().catch(console.error);
