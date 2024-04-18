import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { Secp256k1, keccak256 } from '@cosmjs/crypto';
import {
  DirectSecp256k1HdWallet,
  DirectSecp256k1Wallet,
} from '@cosmjs/proto-signing';
import { GasPrice, SigningStargateClient } from '@cosmjs/stargate';
import {
  Comet38Client,
  CometClient,
  Tendermint34Client,
  Tendermint37Client,
} from '@cosmjs/tendermint-rpc';
import { readFileSync } from 'fs';
import yaml from 'js-yaml';

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
  networks: {
    id: string;
    hrp: string;
    endpoint: {
      rpc: string;
      rest: string;
      grpc: string;
    };
    gas: {
      price: string;
      denom: string;
    };
    domain: number;
    tm_version?: '34' | '37' | '38';
  }[];

  evm_networks: {
    name: string;
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

  signer: string;

  deploy: {
    ism?: IsmType;
    hooks?: {
      default?: HookType;
      required?: HookType;
    };
  };
};

export class Client {
  wasm: SigningCosmWasmClient;
  stargate: SigningStargateClient;
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

export const getEvmNetwork = (networkName: string): Config['evm_networks'][number] => {
  const ret = config.evm_networks.find((v) => v.name === networkName);
  if (!ret)
    throw new Error(`EVM Network ${networkName} not found in the config file`);
  return ret;
}

export async function getSigningClient(
  networkId: string,
  { signer }: Config,
): Promise<Client> {
  const { tm_version, hrp, gas, endpoint } = getNetwork(networkId);

  const wallet =
    signer.split(' ').length > 1
      ? await DirectSecp256k1HdWallet.fromMnemonic(signer, { prefix: hrp })
      : await DirectSecp256k1Wallet.fromKey(Buffer.from(signer, 'hex'), hrp);

  const [account] = await wallet.getAccounts();
  const gasPrice = GasPrice.fromString(`${gas.price}${gas.denom}`);

  let clientBase: CometClient;

  switch (tm_version || '38') {
    case '34':
      clientBase = await Tendermint34Client.connect(endpoint.rpc);
      break;
    case '37':
      clientBase = await Tendermint37Client.connect(endpoint.rpc);
      break;
    case '38':
      clientBase = await Comet38Client.connect(endpoint.rpc);
      break;
  }

  const wasm = await SigningCosmWasmClient.createWithSigner(
    clientBase,
    wallet,
    { gasPrice },
  );
  const stargate = await SigningStargateClient.createWithSigner(
    clientBase,
    wallet,
    { gasPrice },
  );

  const pubkey = Secp256k1.uncompressPubkey(account.pubkey);
  const ethaddr = keccak256(pubkey.slice(1)).slice(-20);

  return {
    wasm,
    stargate,
    signer: account.address,
    signer_addr: Buffer.from(ethaddr).toString('hex'),
    signer_pubkey: Buffer.from(account.pubkey).toString('hex'),
  };
}
