import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";

import { loadContext } from "./src/load_context";
import HplMailbox from "./src/contracts/hpl_mailbox";
import { Context } from "./src/types";
import HplWarpNative from "./src/contracts/hpl_warp_native";

const NETWORK_ID = process.env.NETWORK_ID || "osmo-test-5";
const NETWORK_HRP = process.env.NETWORK_HRP || "osmo";
const NETWORK_URL =
  process.env.NETWORK_URL || "https://rpc.osmotest5.osmosis.zone";
const NETWORK_GAS = process.env.NETWORK_GAS || "0.025uosmo";

async function getSigningClient(): Promise<{
  client: SigningCosmWasmClient;
  address: string;
}> {
  const mnemonic = process.env["SIGNING_MNEMONIC"] as string;
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: NETWORK_HRP,
  });
  const [{ address }] = await wallet.getAccounts();

  const client = await SigningCosmWasmClient.connectWithSigner(
    NETWORK_URL,
    wallet,
    {
      gasPrice: GasPrice.fromString(NETWORK_GAS),
    }
  );
  return { client, address };
}

type Const<T> = new (
  address: string | undefined,
  codeId: number | undefined,
  digest: string,
  signer: string,
  client: SigningCosmWasmClient
) => T;

class ContractFetcher {
  constructor(
    private ctx: Context,
    private owner: string,
    private client: SigningCosmWasmClient
  ) {}

  public get<T>(f: Const<T>, name: string): T {
    return new f(
      this.ctx.contracts[name].address,
      this.ctx.contracts[name].codeId,
      this.ctx.contracts[name].digest,
      this.owner,
      this.client
    );
  }
}

async function main() {
  const { client, address: owner } = await getSigningClient();

  const ctx = loadContext(NETWORK_ID);

  const fetcher = new ContractFetcher(ctx, owner, client);

  const mailbox = fetcher.get(HplMailbox, "hpl_mailbox");

  const warp_native = fetcher.get(HplWarpNative, "hpl_warp_native");

  const target_denom =
    "ibc/B5CB286F69D48B2C4F6F8D8CF59011C40590DCF8A91617A5FBA9FF0A7B21307F";

  const ibc_route = await warp_native.instantiate({
    token: {
      collateral: {
        denom: target_denom,
      },
    },
    hrp: "dual",
    owner,
    mailbox: mailbox.address!,
  });

  console.log("ibc_route", ibc_route);
}

main().catch(console.error);
