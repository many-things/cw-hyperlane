// prevents "TypeError: Reflect.hasOwnMetadata is not a function"
import "reflect-metadata";

import { writeFileSync } from "fs";

import { Client, HookType, config, getSigningClient } from "../src/config";
import { loadContext } from "../src/load_context";

import { Contracts, deploy_ism } from "../src/deploy";
import { Context } from "../src/types";
import { ContractFetcher } from "./fetch";

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
  ctx = await deploy_igp(ctx, client, contracts);
  ctx = await deploy_ism_hook(ctx, client, contracts);

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

const deploy_igp = async (
  ctx: Context,
  client: Client,
  { igp }: Contracts
): Promise<Context> => {
  // init igp
  ctx.contracts[name(igp.core)] = await igp.core.instantiate({
    hrp: config.network.hrp,
    owner: client.signer,
    gas_token: config.deploy.igp.token || config.network.gas.denom,
    beneficiary: client.signer,
    default_gas_usage: "250000" // must be string
  });

  // init igp oracle
  ctx.contracts[name(igp.oracle)] = await igp.oracle.instantiate({
    owner: client.signer,
  });

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

  return ctx;
};

const deploy_ism_hook = async (
  ctx: Context,
  client: Client,
  contracts: Contracts
) => {
  ctx.contracts["hpl_default_ism"] = {
    ...ctx.contracts[`hpl_ism_${config.deploy.ism?.type || "multisig"}`],

    address: await deploy_ism(
      client,
      config.deploy.ism || {
        type: "multisig",
        owner: "<signer>",
        validators: {
          123: {
            addrs: [client.signer_addr],
            threshold: 1,
          },
        },
      },
      contracts
    ),
  };

  ctx.contracts["hpl_default_hook"] = {
    ...ctx.contracts[
      config.deploy.hooks?.default?.type &&
      config.deploy.hooks?.default?.type !== "mock"
        ? `hpl_hook_${config.deploy.hooks.default.type}`
        : "hpl_test_mock_hook"
    ],

    address: await deploy_hook(
      ctx,
      client,
      config.deploy.hooks?.default || { type: "mock" },
      contracts
    ),
  };

  ctx.contracts["hpl_required_hook"] = {
    ...ctx.contracts[
      config.deploy.hooks?.required?.type &&
      config.deploy.hooks?.required?.type !== "mock"
        ? `hpl_hook_${config.deploy.hooks.required.type}`
        : "hpl_test_mock_hook"
    ],

    address: await deploy_hook(
      ctx,
      client,
      config.deploy.hooks?.required || { type: "mock" },
      contracts
    ),
  };

  return ctx;
};

const deploy_hook = async (
  ctx: Context,
  client: Client,
  hook: HookType,
  contracts: Contracts
): Promise<string> => {
  const {
    core: { mailbox },
    hooks,
    igp,
    mocks,
  } = contracts;

  switch (hook.type) {
    case "aggregate":
      const aggregate_hook_res = await hooks.aggregate.instantiate({
        owner: hook.owner === "<signer>" ? client.signer : hook.owner,
        hooks: await Promise.all(
          hook.hooks.map((v) => deploy_hook(ctx, client, v, contracts))
        ),
      });

      return aggregate_hook_res.address!;

    case "merkle":
      const merkle_hook_res = await hooks.merkle.instantiate({
        mailbox: addr(ctx, mailbox),
      });

      return merkle_hook_res.address!;

    case "mock":
      const mock_hook_res = await mocks.hook.instantiate({});

      return mock_hook_res.address!;

    case "pausable":
      const pausable_hook_res = await hooks.pausable.instantiate({
        owner: hook.owner === "<signer>" ? client.signer : hook.owner,
      });

      return pausable_hook_res.address!;

    case "igp":
      return ctx.contracts[name(igp.core)].address!;

    case "routing":
      const routing_hook_res = await hooks.routing.instantiate({
        owner: hook.owner === "<signer>" ? client.signer : hook.owner,
      });

      await client.wasm.execute(
        client.signer,
        routing_hook_res.address!,
        {
          router: {
            set_routes: {
              set: await Promise.all(
                Object.entries(hook.hooks).map(async ([domain, v]) => {
                  const route = await deploy_hook(ctx, client, v, contracts);
                  return { domain, route };
                })
              ),
            },
          },
        },
        "auto"
      );
    default:
      throw new Error("invalid hook type");
  }
};

main().catch(console.error);
