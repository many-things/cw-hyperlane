import "reflect-metadata";
import { Command } from "commander";

import { version } from "../package.json";
import { config, getSigningClient } from "../src/config";
import { addPad } from "../src/conv";
import { loadContext } from "../src/load_context";
import { ContractFetcher } from "./fetch";
import { ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { toBech32 } from "@cosmjs/encoding";
import { Context } from "../src/types";
import { HplHookMerkle, HplIgp, HplIsmAggregate, HplMailbox, HplIgpOracle } from "../src/contracts";

const program = new Command();

program.name("hyperlane CLI").version(version);

const ismCommand = program.command("ism");
ismCommand
  .command("getIsm")
  .argument("<recipient_addr>", "recipient address in bech32")
  .action(makeHandler("getIsm"));
ismCommand
  .command("show")
  .argument("<ism_addr>", "ism address in bech32")
  .argument("<originDomain>", "origin domain to be used when multisig")
  .action(makeHandler("showIsm"));
ismCommand
  .command("deploy")
  .action(makeHandler("deploy"));

const mailboxCommand = program.command("mailbox");
mailboxCommand.command("show").action(makeMailboxHandler("show"));
mailboxCommand.command("dispatch").action(makeMailboxHandler("dispatch"))

const igpCommand = program.command("igp");
igpCommand.command("deploy").action(makeIgpHandler("deploy"))

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

async function loadDeps(ctx: Context) {
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

const name = (c: any) => c.contractName;
const addr = (ctx: Context, c: any) => ctx.contracts[name(c)].address!;


function makeIgpHandler(
  action: "deploy"
): (...args: any[]) => void | Promise<void> {
  const ctx = loadContext(config.network.id);
  
  switch (action) {
    case "deploy":
      return async () => {
        const client = await getSigningClient(config);
        const fetcher = new ContractFetcher(ctx, client);
        const {
          core: { mailbox },
          igp,
          hooks,
          warp,
        } = fetcher.getContracts();

        console.log('Deploy IGP Core')
        ctx.contracts[name(igp.core)] = await igp.core.instantiate({
          hrp: config.network.hrp,
          owner: client.signer,
          gas_token: config.deploy.igp.token || config.network.gas.denom,
          beneficiary: client.signer,
          default_gas_usage: '250000'
        });
        console.log(`Deploy IGP Oracle for core`)

        ctx.contracts[name(igp.oracle)] = await igp.oracle.instantiate({
          owner: client.signer,
        });


        console.log(`set oracle configs`, Object.entries(config.deploy.igp.configs).map(
          ([domain, v]) => ({
            remote_domain: Number(domain),
            token_exchange_rate: v.exchange_rate.toString(),
            gas_price: v.gas_price.toString(),
          })
        ))
        await client.wasm.execute(
          client.signer,
          addr(ctx, igp.oracle),
          {
            set_remote_gas_data_configs: {
              configs: Object.entries(config.deploy.igp.configs).map(
                ([domain, v]) => ({
                  remote_domain: Number(domain),
                  token_exchange_rate: v.exchange_rate.toString(),
                  gas_price: v.gas_price.toString(),
                })
              ),
            },
          },
          "auto"
        );

        console.log(`set Oracle`, Object.keys(config.deploy.igp.configs).map((domain) => ({
          domain: Number(domain),
          route: addr(ctx, igp.oracle),
        })))
        await client.wasm.execute(
          client.signer,
          addr(ctx, igp.core),
          {
            router: {
              set_routes: {
                set: Object.keys(config.deploy.igp.configs).map((domain) => ({
                  domain: Number(domain),
                  route: addr(ctx, igp.oracle),
                })),
              },
            },
          },
          "auto"
        );

        console.log('Deploy Aggregate hook of merkle and IGP', [hooks.merkle.address, igp.core.address])
        const aggregate_hook_res = await hooks.aggregate.instantiate({
          owner: client.signer,
          hooks: [hooks.merkle.address, igp.core.address],
        });
        console.log(`Deployed Aggregate hook`, {
          agg_hook: aggregate_hook_res.address,
          igp_core: igp.core.address,
          igp_oracle: igp.oracle
        })
      };
  }
}

function makeMailboxHandler(
  action: "show" | "dispatch"
): (...args: any[]) => void | Promise<void> {
  const ctx = loadContext(config.network.id);
  switch (action) {
    case "dispatch": 
      return async () => {
        const { mailbox } = await loadDeps(ctx);

        const res = await mailbox.execute(
          {
            dispatch: {
              dest_domain: Number(169),
              recipient_addr: addPad("0xdeadbeef"),
              msg_body: Buffer.from("Hello from Neutron Mainnet to Manta Pacific oct 29, 12:55 am", "utf-8").toString("hex"),
            },
          },
          [{ denom: "ibc/773B4D0A3CD667B2275D5A4A7A2F0909C0BA0F4059C0B9181E680DDF4965DCC7", amount: "540000" }]
        );
        console.log(res)

      }
    case "show":
      return async () => {
        const { mailbox } = await loadDeps(ctx);
        console.log(`Mailbox address is ${mailbox.address}`)

        const defaultHook = await mailbox.query({ mailbox: {
          default_hook: {},
        } })
        const requiredHook = await mailbox.query({ mailbox: {
          required_hook: {},
        } })
        const ism = await mailbox.query({
          mailbox: { default_ism: {}}
        })
        

        console.log(`Default hook`, defaultHook)
        console.log(`Required Hook`, requiredHook)
      };
  }
}

function makeHandler(
  action: "getIsm" | "showIsm" | "deploy"
): (...args: any[]) => void | Promise<void> {
  const ctx = loadContext(config.network.id);

  switch (action) {
    case "deploy":
      return async () => {
        
      }
    case "getIsm":
      return async (recipient_addr: string) => {
        const { mailbox } = await loadDeps(ctx);

        const ism = await mailbox.query({ mailbox: { default_ism: {} } });
        console.log("Default ISM on mailbox is", ism);

        const recipientIsm = await mailbox.query({
          mailbox: { recipient_ism: { recipient_addr } },
        });

        console.log("Recipient ISM is ", recipientIsm);
      };
    case "showIsm":
      return async (ism_addr: string, originDomain?: string) => {
        // Generic info
        const { client } = await loadDeps(ctx);
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
