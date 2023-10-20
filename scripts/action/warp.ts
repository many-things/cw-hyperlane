import { loadContext } from "../src/load_context";
import HplMailbox from "../src/contracts/hpl_mailbox";
import HplWarpNative from "../src/contracts/hpl_warp_native";
import { config, getSigningClient } from "../src/config";
import { ContractFetcher } from "./fetch";

async function main() {
  const client = await getSigningClient(config);

  const ctx = loadContext(config.network.id);

  const fetcher = new ContractFetcher(ctx, client);

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
    owner: client.signer,
    mailbox: mailbox.address!,
  });

  console.log("ibc_route", ibc_route);
}

main().catch(console.error);
