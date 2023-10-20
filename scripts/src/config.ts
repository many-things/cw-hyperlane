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

export type Config = {
  network: {
    id: string;
    hrp: string;
    url: string;
    gas: string;
    domain: number;
    tm_version?: "34" | "37";
  };

  signer: string;
};

export type Client = {
  wasm: SigningCosmWasmClient;
  stargate: SigningStargateClient;
  signer: string;
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
      gasPrice: GasPrice.fromString(network.gas),
    }
  );
  const stargate = await SigningStargateClient.createWithSigner(
    clientBase,
    wallet,
    {
      gasPrice: GasPrice.fromString(network.gas),
    }
  );

  return {
    wasm,
    stargate,
    signer: account.address,
    signer_pubkey: Buffer.from(account.pubkey).toString("hex"),
  };
}
