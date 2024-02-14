import { Client, HookType, IgpHookType, IsmType, config } from "../src/config";
import {
  HplMailbox,
  HplValidatorAnnounce,
  HplHookAggregate,
  HplHookMerkle,
  HplHookPausable,
  HplHookRouting,
  HplHookRoutingCustom,
  HplIgp,
  HplIsmAggregate,
  HplIsmMultisig,
  HplIsmRouting,
  HplTestMockHook,
  HplTestMockMsgReceiver,
  HplWarpCw20,
  HplWarpNative,
  HplIgpOracle,
  HplHookFee,
  HplTestMockIsm,
} from "./contracts";
import { Context } from "./types";

export type Contracts = {
  core: {
    mailbox: HplMailbox;
    va: HplValidatorAnnounce;
  };
  hooks: {
    aggregate: HplHookAggregate;
    fee: HplHookFee;
    merkle: HplHookMerkle;
    pausable: HplHookPausable;
    routing: HplHookRouting;
    routing_custom: HplHookRoutingCustom;
    routing_fallback: HplHookRoutingCustom;
  };
  igp: {
    core: HplIgp;
    oracle: HplIgpOracle;
  };
  isms: {
    aggregate: HplIsmAggregate;
    multisig: HplIsmMultisig;
    routing: HplIsmRouting;
  };
  mocks: {
    hook: HplTestMockHook;
    ism: HplTestMockIsm;
    receiver: HplTestMockMsgReceiver;
  };
  warp: {
    cw20: HplWarpCw20;
    native: HplWarpNative;
  };
};

const name = (c: any) => c.contractName;
const addr = (ctx: Context, c: any) => ctx.contracts[name(c)].address!;

export const deploy_ism = async (
  ctx: Context,
  client: Client,
  ism: IsmType,
  contracts: Contracts
): Promise<[Context, string]> => {
  const { isms } = contracts;

  switch (ism.type) {
    // deploy multisig ism
    case "multisig":
      const multisig_ism_res = await isms.multisig.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
      });
      ctx.contracts[name(isms.multisig)] = multisig_ism_res;

      for (const [domain, { addrs, threshold }] of Object.entries(
        ism.validators
      )) {
        await client.wasm.execute(
          client.signer,
          multisig_ism_res.address!,
          {
            set_validators: {
              domain: Number(domain),
              threshold,
              validators: addrs,
            },
          },
          "auto"
        );
      }

      return [ctx, multisig_ism_res.address!];

    // deploy aggregate ism
    case "aggregate":
      const aggregate_ism_res = await isms.aggregate.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
        isms: await Promise.all(
          ism.isms.map(async (v) => {
            const [newCtx, aggr] = await deploy_ism(ctx, client, v, contracts);

            ctx = newCtx;

            return aggr;
          })
        ),
      });
      ctx.contracts[name(isms.aggregate)] = aggregate_ism_res;

      return [ctx, aggregate_ism_res.address!];

    // deploy routing ism
    case "routing":
      const routing_ism_res = await isms.routing.instantiate({
        owner: ism.owner === "<signer>" ? client.signer : ism.owner,
      });
      ctx.contracts[name(isms.routing)] = routing_ism_res;

      await client.wasm.execute(
        client.signer,
        routing_ism_res.address!,
        {
          router: {
            set_routes: {
              set: await Promise.all(
                Object.entries(ism.isms).map(async ([domain, v]) => {
                  const [newCtx, route] = await deploy_ism(
                    ctx,
                    client,
                    v,
                    contracts
                  );

                  ctx = newCtx;

                  return { domain, route };
                })
              ),
            },
          },
        },
        "auto"
      );

      return [ctx, routing_ism_res.address!];

    default:
      throw new Error("invalid ism type");
  }
};

export const deploy_hook = async (
  ctx: Context,
  client: Client,
  hook: HookType,
  contracts: Contracts
): Promise<[Context, string]> => {
  const {
    core: { mailbox },
    hooks,
    mocks,
  } = contracts;

  switch (hook.type) {
    // deploy fee hook
    case "fee":
      const fee_hook_res = await hooks.fee.instantiate({
        owner: hook.owner === "<signer>" ? client.signer : hook.owner,
        fee: { ...hook.fee, amount: hook.fee.amount.toString() },
      });

      ctx.contracts[name(hooks.fee)] = fee_hook_res;

      return [ctx, fee_hook_res.address!];

    // deploy merkle hook
    case "merkle":
      const merkle_hook_res = await hooks.merkle.instantiate({
        mailbox: addr(ctx, mailbox),
      });

      ctx.contracts[name(hooks.merkle)] = merkle_hook_res;

      return [ctx, merkle_hook_res.address!];

    // deploy mock hook
    case "mock":
      const mock_hook_res = await mocks.hook.instantiate({});

      ctx.contracts[name(mocks.hook)] = mock_hook_res;

      return [ctx, mock_hook_res.address!];

    // deploy pausable hook
    case "pausable":
      const pausable_hook_res = await hooks.pausable.instantiate({
        owner: hook.owner === "<signer>" ? client.signer : hook.owner,
        paused: hook.paused || false,
      });

      ctx.contracts[name(hooks.pausable)] = pausable_hook_res;

      return [ctx, pausable_hook_res.address!];

    // deploy igp hook
    case "igp":
      ctx = await deploy_igp(ctx, client, hook, contracts);

      return [ctx, addr(ctx, contracts.igp.core)];

    // deploy aggregate hook
    case "aggregate":
      const aggregate_set = [];

      for (const v of hook.hooks) {
        const [newCtx, aggr] = await deploy_hook(ctx, client, v, contracts);

        ctx = newCtx;

        aggregate_set.push(aggr);
      }

      const aggregate_hook_res = await hooks.aggregate.instantiate({
        owner: hook.owner === "<signer>" ? client.signer : hook.owner,
        hooks: aggregate_set,
      });

      ctx.contracts[name(hooks.aggregate)] = aggregate_hook_res;

      return [ctx, aggregate_hook_res.address!];

    // deploy routing hook
    case "routing":
      const routing_hook_res = await hooks.routing.instantiate({
        owner: hook.owner === "<signer>" ? client.signer : hook.owner,
      });

      ctx.contracts[name(hooks.routing)] = routing_hook_res;

      const routing_set = [];

      for (const [domain, v] of Object.entries(hook.hooks)) {
        const [newCtx, route] = await deploy_hook(ctx, client, v, contracts);

        ctx = newCtx;

        routing_set.push({ domain: Number(domain), route });
      }

      await client.wasm.execute(
        client.signer,
        routing_hook_res.address!,
        {
          router: {
            set_routes: {
              set: routing_set,
            },
          },
        },
        "auto"
      );

      return [ctx, routing_hook_res.address!];
    default:
      throw new Error("invalid hook type");
  }
};

export const deploy_igp = async (
  ctx: Context,
  client: Client,
  settings: IgpHookType,
  { igp }: Contracts
): Promise<Context> => {
  // init igp
  ctx.contracts[name(igp.core)] = await igp.core.instantiate({
    hrp: config.network.hrp,
    owner: client.signer,
    gas_token: settings.token || config.network.gas.denom,
    beneficiary: client.signer,
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
        configs: Object.entries(settings.configs).map(([domain, v]) => ({
          remote_domain: Number(domain),
          token_exchange_rate: v.exchange_rate.toString(),
          gas_price: v.gas_price.toString(),
        })),
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
          set: Object.keys(settings.configs).map((domain) => ({
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
