import "reflect-metadata";

import { writeFileSync } from "fs";

import { loadContext } from "../src/load_context";
import {
  Client,
  DEFAULT_HOOK,
  DEFAULT_ISM,
  HookType,
  config,
  getSigningClient,
} from "../src/config";

import { ContractFetcher } from "./fetch";
import { Context } from "../src/types";
import { Contracts, deploy_ism, deploy_hook } from "../src/deploy";

const name = (c: any) => c.contractName;
const addr = (ctx: Context, c: any) => ctx.contracts[name(c)].address!;

async function main() {
  const client = await getSigningClient(config);

  let ctx = loadContext(config.network.id);

  const contracts = new ContractFetcher(ctx, client).getContracts();
  const {
    core: { mailbox },
    mocks,
  } = contracts;

  ctx = await deploy_core(ctx, client, contracts);
  ctx = await deploy_ism_and_hook(ctx, client, contracts);

  // init test mock msg receiver
  ctx.contracts[name(mocks.receiver)] = await mocks.receiver.instantiate({
    hrp: config.network.hrp,
  });

  // pre-setup
  await client.wasm.executeMultiple(
    client.signer,
    [
      {
        contractAddress: addr(ctx, mailbox),
        msg: {
          set_default_ism: {
            ism: ctx.contracts["hpl_default_ism"].address!,
          },
        },
      },
      {
        contractAddress: addr(ctx, mailbox),
        msg: {
          set_default_hook: {
            hook: ctx.contracts["hpl_default_hook"].address!,
          },
        },
      },
      {
        contractAddress: addr(ctx, mailbox),
        msg: {
          set_required_hook: {
            hook: ctx.contracts["hpl_required_hook"].address!,
          },
        },
      },
    ],
    "auto"
  );

  writeFileSync("./save.json", JSON.stringify(ctx, null, 2));
}

const deploy_core = async (
  ctx: Context,
  client: Client,
  { core: { mailbox, va } }: Contracts
): Promise<Context> => {
  // init mailbox
  ctx.contracts[name(mailbox)] = await mailbox.instantiate({
    hrp: config.network.hrp,
    owner: client.signer,
    domain: config.network.domain,
  });

  // init validator announce
  ctx.contracts[name(va)] = await va.instantiate({
    hrp: config.network.hrp,
    mailbox: addr(ctx, mailbox),
  });

  return ctx;
};

const deploy_ism_and_hook = async (
  ctx: Context,
  client: Client,
  contracts: Contracts
) => {
  // deploy default ism

  ctx.contracts["hpl_default_ism"] = {
    ...ctx.contracts[`hpl_ism_${config.deploy.ism?.type || "multisig"}`],
  };

  [ctx, ctx.contracts["hpl_default_ism"].address] = await deploy_ism(
    ctx,
    client,
    config.deploy.ism || DEFAULT_ISM(client.signer_addr),
    contracts
  );

  // deploy default hook

  ctx.contracts["hpl_default_hook"] = {
    ...ctx.contracts[
      config.deploy.hooks?.default?.type &&
      config.deploy.hooks?.default?.type !== "mock"
        ? `hpl_hook_${config.deploy.hooks.default.type}`
        : "hpl_test_mock_hook"
    ],
  };

  [ctx, ctx.contracts["hpl_default_hook"].address] = await deploy_hook(
    ctx,
    client,
    config.deploy.hooks?.default || DEFAULT_HOOK,
    contracts
  );

  // deploy required hook

  ctx.contracts["hpl_required_hook"] = {
    ...ctx.contracts[
      config.deploy.hooks?.required?.type &&
      config.deploy.hooks?.required?.type !== "mock"
        ? `hpl_hook_${config.deploy.hooks.required.type}`
        : "hpl_test_mock_hook"
    ],
  };

  [ctx, ctx.contracts["hpl_required_hook"].address] = await deploy_hook(
    ctx,
    client,
    config.deploy.hooks?.required || DEFAULT_HOOK,
    contracts
  );

  return ctx;
};

main().catch(console.error);
