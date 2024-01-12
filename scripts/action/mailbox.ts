// prevents "TypeError: Reflect.hasOwnMetadata is not a function"
import "reflect-metadata";

import { ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Command } from "commander";

import { version } from "../package.json";
import { config, getSigningClient } from "../src/config";
import {
  HplHookMerkle,
  HplIgp,
  HplIgpOracle,
  HplIsmAggregate,
  HplMailbox,
} from "../src/contracts";
import { addPad } from "../src/conv";
import { loadContext } from "../src/load_context";
import { ContractFetcher } from "./fetch";

const program = new Command();

program.name("Mailbox CLI").version(version);

program
  .command("dispatch")
  .argument("<dest_domain>", 'destination domain, e.g. "5"')
  .argument("<recipient_addr>", "recipient address in hex")
  .argument("<msg_body>", "message body in utf-8")
  .action(makeHandler("dispatch"));

program
  .command("process")
  .argument("<metadata>", "metadata in hex")
  .argument("<msg_body>", "message body in hex")
  .action(makeHandler("process"));

program.parseAsync(process.argv).catch(console.error);

const parseWasmEventLog = (res: ExecuteResult) => {
  return (
    res.events
      // .filter((v) => v.type.startsWith("wasm"))
      .map((v) => ({
        "@type": v.type.slice(5),
        ...Object.fromEntries(v.attributes.map((x) => [x.key, x.value])),
      }))
  );
};

function makeHandler(
  action: "dispatch" | "process"
): (...args: any[]) => void | Promise<void> {
  const ctx = loadContext(config.network.id);

  const loadDeps = async () => {
    const client = await getSigningClient(config);
    const fetcher = new ContractFetcher(ctx, client);
    const mailbox = fetcher.get(HplMailbox, "hpl_mailbox");
    const igp = fetcher.get(HplIgp, "hpl_igp");
    const igp_oracle = fetcher.get(HplIgpOracle, "hpl_igp_oracle");
    const hook_merkle = fetcher.get(HplHookMerkle, "hpl_hook_merkle");
    const hook_aggregate = fetcher.get(HplIsmAggregate, "hpl_hook_aggregate");

    return {
      client,
      mailbox,
      igp: { core: igp, oracle: igp_oracle },
      hook: { merkle: hook_merkle, aggregate: hook_aggregate },
    };
  };

  switch (action) {
    case "dispatch":
      return async (
        dest_domain: string,
        recipient_addr: string,
        msg_body: string
      ) => {
        const { mailbox } = await loadDeps();

        const res = await mailbox.execute(
          {
            dispatch: {
              dest_domain: Number(dest_domain),
              recipient_addr: addPad(recipient_addr),
              msg_body: Buffer.from(msg_body, "utf-8").toString("hex"),
            },
          },
          [{ denom: "inj", amount: "2500" }]
        );
        console.log(parseWasmEventLog(res));
      };
    case "process":
      return async (metadata: string, msg_body: string) => {
        const { mailbox } = await loadDeps();

        const res = await mailbox.execute({
          process: {
            metadata,
            msg_body,
          },
        });
        console.log(parseWasmEventLog(res));
      };
  }
}
