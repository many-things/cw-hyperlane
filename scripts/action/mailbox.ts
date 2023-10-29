import { Command } from "commander";

import { version } from "../package.json";
import { config, getSigningClient } from "../src/config";
import HplMailbox from "../src/contracts/hpl_mailbox";
import { addPad } from "../src/conv";
import { loadContext } from "../src/load_context";
import { ContractFetcher } from "./fetch";
import { ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import HplIsmAggregate from "../src/contracts/hpl_ism_aggregate";
import HplIgp from "../src/contracts/hpl_igp";
import HplIgpGasOracle from "../src/contracts/hpl_igp_oracle";
import HplHookMerkle from "../src/contracts/hpl_hook_merkle";
import { toBech32 } from "@cosmjs/encoding";

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

program.command("quoteGasPayment")
  .argument("<dest_domain>", 'destination domain, e.g. "5"')  
  .argument("<gas_amount>", 'gas amount')
  .action(makeHandler("quoteGasPayment"))

program.command("recipientIsm")
  .argument("<recipient>")
  .action(makeHandler("recipientIsm"))

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
  action: "dispatch" | "process" | "quoteGasPayment" | "recipientIsm"
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
    case "recipientIsm":
      return async (recipient: string) => {
        const { mailbox, client } = await loadDeps();
        recipient = toBech32('dual', Buffer.from(recipient, 'hex'))
        console.log(recipient)
        console.log(await client.wasm.queryContractSmart(recipient, {
          ism_specifier: {
            interchain_security_module: {}
          }
        }))
        const res = await mailbox.query({ mailbox: {
          recipient_ism: {
            recipient_addr: toBech32('dual', Buffer.from(recipient, 'hex'))
          }}
        })
        console.log(res)

        console.log('defaut ism')
        console.log(await mailbox.query({ mailbox: { default_ism: {}} }))
      }
    case "quoteGasPayment":
      return async (
        dest_domain: string,
        gas_amount: number
      ) => {
        const { igp } = await loadDeps();
        // let res = await igp.core.query({ 
        //   igp: {
        //     quote_gas_payment: {
        //       dest_domain: Number(dest_domain),
        //       gas_amount: gas_amount.toString()
        //     }
        //   }
        // })
        let res
        console.log(res)
        await igp.core.execute({
          router: {
            set_route: {
              set: {
                domain: Number(dest_domain),
                route: igp.oracle.address!
              }
            }
          }
        })
        // await igp.oracle.execute({
        //   set_remote_gas_data: {
        //     config: {
        //       remote_domain: Number(dest_domain),
        //       token_exchange_rate: "1000000",
        //       gas_price: "1"
        //     }
        //   }
        // })
        res = await igp.core.query({ 
          igp: {
            quote_gas_payment: {
              dest_domain: Number(dest_domain),
              gas_amount: gas_amount.toString()
            }
          }
        })
        console.log(res)
      }
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
          [{ denom: "untrn", amount: "2" }]
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
