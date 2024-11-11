import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { Secp256k1, keccak256 } from '@cosmjs/crypto';
import {
  DirectSecp256k1HdWallet,
  DirectSecp256k1Wallet,
} from '@cosmjs/proto-signing';
import { GasPrice, SigningStargateClient } from '@cosmjs/stargate';
import { readFileSync } from 'fs';
import yaml from 'js-yaml';

import {
  DEFAULT_CRADLE_GRPC_BASE_URL,
  DEFAULT_CRADLE_RPC_BASE_URL,
} from './constants';

export type IsmType =
  | {
      type: 'multisig';
      owner: string;
      validators: {
        [domain: number]: { addrs: string[]; threshold: number };
      };
    }
  | {
      type: 'mock';
    }
  | {
      type: 'aggregate';
      owner: string;
      isms: IsmType[];
    }
  | {
      type: 'routing';
      owner: string;
      isms: { [domain: number]: IsmType };
    };

export type FeeHookType = {
  type: 'fee';
  owner: string;
  fee: {
    denom?: string;
    amount: bigint;
  };
};

export type IgpHookType = {
  type: 'igp';
  owner: string;
  token?: string;
  configs: {
    [domain: number]: {
      exchange_rate: number;
      gas_price: number;
    };
  };
  default_gas_usage: number;
};

export type RoutingHookType = {
  type: 'routing';
  owner: string;
  hooks: { [domain: number]: HookType };
};

export type RoutingCustomHookType = {
  type: 'routing-custom';
  owner: string;
  hooks: { [domain: number]: HookType };
  custom_hooks: {
    [domain: number]: { recipient: string; hook: HookType };
  };
};

export type RoutingFallbackHookType = {
  type: 'routing-fallback';
  owner: string;
  hooks: { [domain: number]: HookType };
  fallback_hook: HookType;
};

export type HookType =
  | FeeHookType
  | {
      type: 'merkle';
    }
  | {
      type: 'mock';
    }
  | {
      type: 'pausable';
      owner: string;
      paused: boolean;
    }
  | IgpHookType
  | { type: 'aggregate'; owner: string; hooks: HookType[] }
  | RoutingHookType
  | RoutingCustomHookType
  | RoutingFallbackHookType;

export type Config = {
  networks: ({
    id: string;
    signer: string;
    hrp: string;
    gas: {
      price: string;
      denom: string;
    };
    domain: number;
  } & (
    | {
        is_cradle: undefined | false;
        endpoint: {
          rpc: string;
          rest: string;
          grpc: string;
        };
      }
    | {
        is_cradle: true;
        cradle_rpc_base_url?: string;
        cradle_rest_base_url?: string;
        cradle_grpc_base_url?: string;
        cradle_session_id: string;
      }
  ))[];

  evm_networks: {
    name: string;
    signer: string;
    chain_id: number;
    rpc_endpoint: string;
    network: string;
    native_currency: {
      name: string;
      symbol: string;
      decimals: number;
    };
    mailbox_address: `0x${string}`;
    multisig_ism_factory_address: `0x${string}`;
  }[];

  deploy: {
    ism?: IsmType;
    hooks?: {
      default?: HookType;
      required?: HookType;
    };
  };
};

import { Network } from "@injectivelabs/networks";
import { MsgBroadcasterWithPk } from '@injectivelabs/sdk-ts';

export class Client {
  wasm: SigningCosmWasmClient;
  stargate: SigningStargateClient;
  injective: MsgBroadcasterWithPk;
  injective_signer: string;
  signer: string;
  signer_addr: string;
  signer_pubkey: string;
}

const path = process.env.CONFIG || `${process.cwd()}/config.yaml`;

export const getNetwork = (networkId: string): Config['networks'][number] => {
  const ret = config.networks.find((v) => v.id === networkId);
  if (!ret)
    throw new Error(`Network ${networkId} not found in the config file`);
  return ret;
};

export const config = yaml.load(readFileSync(path, 'utf-8')) as Config;

export const getEvmNetwork = (
  networkName: string,
): Config['evm_networks'][number] => {
  const ret = config.evm_networks.find((v) => v.name === networkName);
  if (!ret)
    throw new Error(`EVM Network ${networkName} not found in the config file`);
  return ret;
};

export async function getSigningClient(networkId: string): Promise<Client> {
  const networkConfig = getNetwork(networkId);

  const { signer, hrp, gas } = networkConfig;

  const endpoint = (() =>
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

  const wallet =
    signer.split(' ').length > 1
      ? await DirectSecp256k1HdWallet.fromMnemonic(signer, { prefix: hrp })
      : await DirectSecp256k1Wallet.fromKey(Buffer.from(signer, 'hex'), hrp);

  const [account] = await wallet.getAccounts();

  const gasPrice = GasPrice.fromString(`${gas.price}${gas.denom}`);

  const injective = new MsgBroadcasterWithPk({
    privateKey: signer,
    network: Network.Mainnet
  });

  // TODO: figure out how to derive this
  const injective_signer = "inj1xe7h6urta6mftjl8ncud2fv42wgdtdad8ackjm";

  const wasm = await SigningCosmWasmClient.connectWithSigner(
    endpoint.rpc,
    wallet,
    { gasPrice },
  );
  const stargate = await SigningStargateClient.connectWithSigner(
    endpoint.rpc,
    wallet,
    { gasPrice },
  );

  const pubkey = Secp256k1.uncompressPubkey(account.pubkey);
  const ethaddr = keccak256(pubkey.slice(1)).slice(-20);

  return {
    wasm,
    stargate,
    injective,
    injective_signer,
    signer: account.address,
    signer_addr: Buffer.from(ethaddr).toString('hex'),
    signer_pubkey: Buffer.from(account.pubkey).toString('hex'),
  };
}
