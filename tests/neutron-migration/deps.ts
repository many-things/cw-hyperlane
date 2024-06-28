import { wasmTypes } from '@cosmjs/cosmwasm-stargate';
import { Registry } from '@cosmjs/proto-signing';
import { bankTypes } from '@cosmjs/stargate/build/modules';

import { getNetwork } from '../../script/shared/config';
import {
  DEFAULT_CRADLE_GRPC_BASE_URL,
  DEFAULT_CRADLE_RPC_BASE_URL,
} from '../../script/shared/constants';
import { loadContext } from '../../script/shared/context';

export const RUNNER = 'neutron1rdrfmxtu7jes96r9ta2ykf922ejwd9cw0wazhw';

export const MULTISIG =
  'neutron1fqf5mprg3f5hytvzp3t7spmsum6rjrw80mq8zgkc0h6rxga0dtzqws3uu7';
export const MULTISIG_PRE_PROP =
  'neutron16vvgmn9734g806a48uw9qawjk6sy709jlp7xlxtpelpvffjxl2lqykqklw';
export const MULTISIG_PROP_MODULE =
  'neutron1r5zzzm4wmwqk897rff6ppv4d7snfmyzjnzmzydlfdff5xl9qczzqy9cdrt';
export const MULTISIG_MEMBERS = [
  'neutron1plh8l930u0x298cndmjfhxr20wg8xrxj0x70qr',
  'neutron1tkavhfqt8358vl74z7r5kdkdy05s98yka0gl0t',
  'neutron1rdrfmxtu7jes96r9ta2ykf922ejwd9cw0wazhw',
  'neutron16q4u9swh27fs0y7w733qnaxk9qr7zrkgdf696n',
];

export const TIA_WHALE = 'neutron1r2xujqn6ud8g29pppz02e0jpk500rctxxfa622';
export const NEUTRON_WHALE = 'neutron1p3ucd3ptpw902fluyjzhq3ffgq4ntddau6elth';

export const networkId = 'neutron-1-fork';
export const denom = 'untrn';
export const hrp = 'neutron';

export const ctx = loadContext(networkId);
export const reg = new Registry([...bankTypes, ...wasmTypes]);

export const networkConfig = getNetwork(networkId);

export const endpoint = (() =>
  networkConfig.is_cradle
    ? {
        rpc: (
          networkConfig.cradle_rpc_base_url || DEFAULT_CRADLE_RPC_BASE_URL
        ).replaceAll('{session_id}', networkConfig.cradle_session_id),

        grpc: (
          networkConfig.cradle_grpc_base_url || DEFAULT_CRADLE_GRPC_BASE_URL
        ).replaceAll('{session_id}', networkConfig.cradle_session_id),
      }
    : networkConfig.endpoint)();
