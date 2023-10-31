import { Command } from "commander";
import { ExecuteResult } from "@cosmjs/cosmwasm-stargate";

import { version } from "../package.json";
import { config, getSigningClient } from "../src/config";
import { loadContext } from "../src/load_context";
import { ContractFetcher } from "./fetch";
import {
  HplMailbox,
  HplIgp,
  HplIgpGasOracle,
  HplHookMerkle,
  HplIsmAggregate,
} from "../src/contracts";

const program = new Command();

program.name("Mailbox CLI").version(version);

program
  .command("get-ism")
  .argument("<recipient_addr>", "recipient address in bech32")
  .action(makeHandler("get-ism"));

program
  .command("show")
  .argument("<ism_addr>", "ism address in bech32")
  .argument("<originDomain>", "origin domain to be used when multisig")
  .action(makeHandler("show-ism"));

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
  action: "get-ism" | "show-ism"
): (...args: any[]) => void | Promise<void> {
  const ctx = loadContext(config.network.id);

  const loadDeps = async () => {
    const client = await getSigningClient(config);
    const fetcher = new ContractFetcher(ctx, client);
    const mailbox = fetcher.get(HplMailbox, "hpl_mailbox");
    const igp = fetcher.get(HplIgp, "hpl_igp");
    const igp_oracle = fetcher.get(HplIgpGasOracle, "hpl_igp_oracle");
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
    case "get-ism":
      return async (recipient_addr: string) => {
        const { mailbox } = await loadDeps();

        const ism = await mailbox.query({ mailbox: { default_ism: {} } });
        console.log("Default ISM on mailbox is", ism);

        const recipientIsm = await mailbox.query({
          mailbox: { recipient_ism: { recipient_addr } },
        });

        console.log("Recipient ISM is ", recipientIsm);
      };
    case "show-ism":
      return async (ism_addr: string, originDomain?: string) => {
        // Generic info
        const { client } = await loadDeps();
        const ism = await client.wasm.queryContractSmart(ism_addr, {
          ism: {
            module_type: {},
          },
        });
        switch (ism.type) {
          case "message_id_multisig":
            const msig = await client.wasm.queryContractSmart(ism_addr, {
              multisig_ism: {
                enrolled_validators: {
                  domain: Number(originDomain),
                },
              },
            });
            const owner = await client.wasm.queryContractSmart(ism_addr, {
              ownable: { get_owner: {} },
            });
            console.log(msig, owner);
            break;

          default:
            break;
        }
      };
  }
}
