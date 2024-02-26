import yaml from "js-yaml";
import { readFileSync } from "fs";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import {
  Comet38Client,
  Tendermint34Client,
  Tendermint37Client,
  CometClient,
} from "@cosmjs/tendermint-rpc";
import {
  DirectSecp256k1HdWallet,
  DirectSecp256k1Wallet,
} from "@cosmjs/proto-signing";
import { GasPrice, SigningStargateClient } from "@cosmjs/stargate";
import { Secp256k1, keccak256 } from "@cosmjs/crypto";

export type IsmType =
  | {
      type: "multisig";
      owner: string;
      validators: {
        [domain: number]: { addrs: string[]; threshold: number };
      };
    }
  | {
      type: "mock";
    }
  | {
      type: "aggregate";
      owner: string;
      isms: Exclude<IsmType, number[]>[];
    }
  | {
      type: "routing";
      owner: string;
      isms: { [domain: number]: Exclude<IsmType, number[]> };
    }
  | number[];

export type FeeHookType = {
  type: "fee";
  owner: string;
  fee: {
    denom?: string;
    amount: bigint;
  };
};

export type IgpHookType = {
  type: "igp";
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
  type: "routing";
  owner: string;
  hooks: { [domain: number]: HookType };
};

export type RoutingCustomHookType = {
  type: "routing-custom";
  owner: string;
  hooks: { [domain: number]: HookType };
  custom_hooks: {
    [domain: number]: { recipient: string; hook: HookType };
  };
};

export type RoutingFallbackHookType = {
  type: "routing-fallback";
  owner: string;
  hooks: { [domain: number]: HookType };
  fallback_hook: HookType;
};

export type HookType =
  | FeeHookType
  | {
      type: "merkle";
    }
  | {
      type: "mock";
    }
  | {
      type: "pausable";
      owner: string;
      paused: boolean;
    }
  | IgpHookType
  | { type: "aggregate"; owner: string; hooks: HookType[] }
  | RoutingHookType
  | RoutingCustomHookType
  | RoutingFallbackHookType;

export type Config = {
  networks: {
    id: string;
    hrp: string;
    url: string;
    gas: {
      price: string;
      denom: string;
    };
    domain: number;
    tm_version?: "34" | "37" | "38";
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

export const getNetwork = (networkId: string): Config["networks"][number] => {
  const ret = config.networks.find((v) => v.id === networkId);
  if (!ret)
    throw new Error(`Network ${networkId} not found in the config file`);
  return ret;
};

export const config = yaml.load(readFileSync(path, "utf-8")) as Config;

export async function getSigningClient(
  networkId: string,
  { signer }: Config
): Promise<Client> {
  const { tm_version, hrp, gas, url } = getNetwork(networkId);

  const wallet =
    signer.split(" ").length > 0
      ? await DirectSecp256k1HdWallet.fromMnemonic(signer, { prefix: hrp })
      : await DirectSecp256k1Wallet.fromKey(Buffer.from(signer, "hex"), hrp);

  const [account] = await wallet.getAccounts();
  const gasPrice = GasPrice.fromString(`${gas.price}${gas.denom}`);

  let clientBase: CometClient;

  switch (tm_version || "38") {
    case "34":
      clientBase = await Tendermint34Client.connect(url);
      break;
    case "37":
      clientBase = await Tendermint37Client.connect(url);
      break;
    case "38":
      clientBase = await Comet38Client.connect(url);
      break;
  }

  const wasm = await SigningCosmWasmClient.createWithSigner(
    clientBase,
    wallet,
    { gasPrice }
  );
  const stargate = await SigningStargateClient.createWithSigner(
    clientBase,
    wallet,
    { gasPrice }
  );

  const pubkey = Secp256k1.uncompressPubkey(account.pubkey);
  const ethaddr = keccak256(pubkey.slice(1)).slice(-20);

  return {
    wasm,
    stargate,
    signer: account.address,
    signer_addr: Buffer.from(ethaddr).toString("hex"),
    signer_pubkey: Buffer.from(account.pubkey).toString("hex"),
  };
}
