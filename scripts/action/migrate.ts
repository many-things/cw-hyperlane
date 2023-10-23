import { ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { config, getSigningClient } from "../src/config";
import HplHookAggregate from "../src/contracts/hpl_hook_aggregate";
import HplIgp from "../src/contracts/hpl_igp";
import HplMailbox from "../src/contracts/hpl_mailbox";
import HplTestMockHook from "../src/contracts/hpl_test_mock_hook";
import { loadContext, saveContext } from "../src/load_context";
import { ContractFetcher } from "./fetch";

const parseWasmEventLog = (res: ExecuteResult) => {
  return res.events
    .filter((v) => v.type.startsWith("wasm"))
    .map((v) => ({
      "@type": v.type.slice(5),
      ...Object.fromEntries(v.attributes.map((x) => [x.key, x.value])),
    }));
};

async function main() {
  const client = await getSigningClient(config);

  const ctx = loadContext(config.network.id);

  const fetcher = new ContractFetcher(ctx, client);

  const igp = fetcher.get(HplIgp, "hpl_igp");
  const mailbox = fetcher.get(HplMailbox, "hpl_mailbox");
  const hook_test = fetcher.get(HplTestMockHook, "hpl_test_mock_hook");
  const hook_merkle = fetcher.get(HplHookAggregate, "hpl_hook_merkle");
  const hook_aggregate = fetcher.get(HplHookAggregate, "hpl_hook_aggregate");

  ctx.contracts["hpl_hook_merkle"] = await hook_merkle.instantiate({
    owner: client.signer,
    mailbox: mailbox.address!,
  });
  console.log(ctx.contracts["hpl_hook_merkle"].address!);

  ctx.contracts["hpl_hook_aggregate"] = await hook_aggregate.instantiate({
    owner: client.signer,
    hooks: [igp.address!, ctx.contracts["hpl_hook_merkle"].address!],
  });

  const res = await client.wasm.executeMultiple(
    client.signer,
    [
      {
        contractAddress: mailbox.address!,
        msg: {
          set_default_hook: {
            hook: hook_test.address!,
          },
        },
      },
      {
        contractAddress: mailbox.address!,
        msg: {
          set_required_hook: {
            hook: hook_aggregate.address!,
          },
        },
      },
    ],
    "auto"
  );
  console.log(parseWasmEventLog(res));

  saveContext(config.network.id, ctx);
}

main().catch(console.error);
