// prevents "TypeError: Reflect.hasOwnMetadata is not a function"
import "reflect-metadata";

import { version } from "../package.json";
import { loadContext } from "../src/load_context";
import { config, getSigningClient } from "../src/config";
import { ContractFetcher } from "./fetch";
import { addPad } from "../src/conv";
import { ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Command } from "commander";

const program = new Command();

program.name("Warp CLI").version(version);

program
  .command("new")
  .argument("<denom>", 'token denom, e.g. "untrn"')
  .option(
    "--token-mode <token_mode>",
    'token mode, e.g. "collateral" or "bridged"',
    "collateral"
  )
  .action(create);

program
  .command("set-ism")
  .argument("<address>", "address of internal warp route")
  .argument("<ismAddress>", "address of ISM")
  .action(setIsm);

program
  .command("link")
  .argument("<address>", "address of internal warp route")
  .argument("<domain>", "domain of external chain, e.g. 5 (goerli)")
  .argument("<external_route>", "address of external route")
  .action(link);

program
  .command("transfer")
  .argument("<address>", "address of internal warp route")
  .argument("<domain>", "domain of external chain, e.g. 5 (goerli)")
  .argument("<recipient>", "recipient address")
  .argument("<amount>")
  .action(transfer);

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

async function create(
  denom: string,
  { tokenMode }: { tokenMode: "collateral" | "bridged" }
) {
  const client = await getSigningClient(config);
  const ctx = loadContext(config.network.id);

  const fetcher = new ContractFetcher(ctx, client);
  const {
    core: { mailbox },
    warp,
  } = fetcher.getContracts();

  switch (tokenMode) {
    case "collateral":
      const ibc_route = await warp.native.instantiate({
        token: {
          collateral: {
            denom,
          },
        },
        hrp: config.network.hrp,
        owner: client.signer,
        mailbox: mailbox.address!,
      });

      console.log("ibc_route", ibc_route);
      return;
    case "bridged":
      throw Error("not implemented");
  }
}

async function setIsm(address: string, ism: string) {
  const client = await getSigningClient(config);
  const resp = await client.wasm.execute(
    client.signer,
    address,
    {
      connection: {
        set_ism: {
          ism,
        },
      },
    },
    "auto"
  );
  console.log(parseWasmEventLog(resp));
  console.log(resp.transactionHash);
}
async function link(address: string, domain: string, external_route: string) {
  const client = await getSigningClient(config);
  const resp = await client.wasm.execute(
    client.signer,
    address,
    {
      router: {
        set_route: {
          set: {
            domain: Number(domain),
            route: addPad(external_route),
          },
        },
      },
    },
    "auto"
  );
  console.log(parseWasmEventLog(resp));
  console.log(resp.transactionHash);
}

async function transfer(
  address: string,
  domain: string,
  recipient: string,
  amount: string
) {
  const client = await getSigningClient(config);

  const {
    type: {
      native: {
        fungible: { denom },
      },
    },
  }: {
    type: {
      native: {
        fungible: {
          denom: string;
        };
      };
    };
  } = await client.wasm.queryContractSmart(address, {
    token_default: {
      token_type: {},
    },
  });

  const resp = await client.wasm.execute(
    client.signer,
    address,
    {
      transfer_remote: {
        dest_domain: Number(domain),
        recipient: addPad(recipient),
        amount,
      },
    },
    "auto",
    undefined,
    [
      { amount, denom },
      { amount: "100", denom: "untrn" },
    ]
  );
  console.log(parseWasmEventLog(resp));
  console.log(resp.transactionHash);
}
