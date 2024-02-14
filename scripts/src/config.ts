import yaml from "js-yaml";
import { readFileSync } from "fs";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import {
  Tendermint34Client,
  Tendermint37Client,
  TendermintClient,
} from "@cosmjs/tendermint-rpc";
import { DirectSecp256k1Wallet } from "@cosmjs/proto-signing";
import { GasPrice, SigningStargateClient } from "@cosmjs/stargate";
import { Secp256k1, keccak256 } from "@cosmjs/crypto";

export const DEFAULT_ISM = (signer: string): IsmType => ({
  type: "multisig",
  owner: "<signer>",
  validators: {
    5: {
      addrs: [signer],
      threshold: 1,
    },
  },
});

export const DEFAULT_HOOK = { type: "mock" } as HookType;

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
      isms: IsmType[];
    }
  | {
      type: "routing";
      owner: string;
      isms: { [domain: number]: IsmType };
    };

export type FeeHookType = {
  type: "fee";
  owner: string;
  fee: {
    denom: string;
    amount: bigint;
  };
};

export type IgpHookType = {
  type: "igp";
  token?: string;
  configs: {
    [domain: number]: {
      exchange_rate: number;
      gas_price: number;
    };
  };
};

export type RoutingHookType = {
  type: "routing";
  owner: string;
  hooks: { [domain: number]: HookType };
  custom_hooks?: {
    [domain: number]: { recipient: string; hook: string };
  };
  fallback_hook?: string;
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
  | RoutingHookType;

export type Config = {
  network: {
    id: string;
    hrp: string;
    url: string;
    gas: {
      price: string;
      denom: string;
    };
    domain: number;
    tm_version?: "34" | "37";
  };

  signer: string;

  deploy: {
    ism?: IsmType;
    hooks?: {
      default?: HookType;
      required?: HookType;
    };
  };
};

export type Client = {
  wasm: SigningCosmWasmClient;
  stargate: SigningStargateClient;
  signer: string;
  signer_addr: string;
  signer_pubkey: string;
};

const path = process.env.CONFIG || `${process.cwd()}/config.yaml`;

export const config = yaml.load(readFileSync(path, "utf-8")) as Config;

export async function getSigningClient({
  network,
  signer,
}: Config): Promise<Client> {
  const wallet = await DirectSecp256k1Wallet.fromKey(
    Buffer.from(signer, "hex"),
    network.hrp
  );

  const [account] = await wallet.getAccounts();

  let clientBase: TendermintClient;

  switch (network.tm_version || "37") {
    case "34":
      clientBase = await Tendermint34Client.connect(network.url);
      break;
    case "37":
      clientBase = await Tendermint37Client.connect(network.url);
      break;
  }

  const wasm = await SigningCosmWasmClient.createWithSigner(
    clientBase,
    wallet,
    {
      gasPrice: GasPrice.fromString(`${network.gas.price}${network.gas.denom}`),
    }
  );
  const stargate = await SigningStargateClient.createWithSigner(
    clientBase,
    wallet,
    {
      gasPrice: GasPrice.fromString(`${network.gas.price}${network.gas.denom}`),
    }
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
