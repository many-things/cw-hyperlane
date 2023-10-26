import { Event } from "@cosmjs/cosmwasm-stargate";
import { config, getSigningClient } from "../src/config";
import { loadContext } from "../src/load_context";
import { ContractFetcher } from "./fetch";

const parseEventLog = (events: readonly Event[]) => {
  return events.map((v) => ({
    "@type": v.type.slice(5),
    ...Object.fromEntries(v.attributes.map((x) => [x.key, x.value])),
  }));
};

async function main() {
  const client = await getSigningClient(config);

  const ctx = loadContext(config.network.id);

  const contracts = new ContractFetcher(ctx, client).getContracts();

  const migrate_resp = await client.wasm.migrate(
    client.signer,
    "dual1nzkcccxw00u9egqfuuq2ue23hjj6kxmfvmc5y0r7wchk5e6nypns6768kk",
    contracts.warp.native.codeId!,
    {},
    "auto"
  );
  console.log(parseEventLog(migrate_resp.events));
}

main().catch(console.error);
