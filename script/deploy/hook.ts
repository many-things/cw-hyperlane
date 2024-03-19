import {
  Client,
  HookType,
  RoutingCustomHookType,
  RoutingFallbackHookType,
  RoutingHookType,
  getNetwork,
} from '../shared/config';
import { Context, ContextHook } from '../shared/context';
import {
  deployContract,
  executeContract,
  executeMultiMsg,
} from '../shared/contract';
import { deployIgp } from './igp';

const deployRoutingHook = async (
  networkId: string,
  ctx: Context,
  client: Client,
  hook: RoutingHookType,
): Promise<ContextHook> => {
  const routing = await deployContract(ctx, client, 'hpl_hook_routing', {
    owner: hook.owner === '<signer>' ? client.signer : hook.owner,
  });

  // if no hooks, return empty hooks
  if (Object.keys(hook.hooks).length === 0) return { ...routing, hooks: {} };

  const routes = [];
  for (const [domain, v] of Object.entries(hook.hooks)) {
    routes.push({
      domain: parseInt(domain),
      route: await deployHook(networkId, ctx, client, v),
    });
  }

  await executeContract(client, routing, {
    router: {
      set_routes: {
        set: routes.map((v) => ({
          domain: v.domain,
          route: v.route.address,
        })),
      },
    },
  });

  return {
    ...routing,
    hooks: routes.reduce((acc, v) => ({ [v.domain]: v.route, ...acc })),
  };
};

const deployCustomRoutingHook = async (
  networkId: string,
  ctx: Context,
  client: Client,
  hook: RoutingCustomHookType,
): Promise<ContextHook> => {
  const routing = await deployContract(ctx, client, 'hpl_hook_routing_custom', {
    owner: hook.owner === '<signer>' ? client.signer : hook.owner,
  });

  // if no hooks, return empty hooks
  if (Object.keys(hook.hooks).length === 0) return { ...routing, hooks: {} };

  const routes = [];
  for (const [domain, v] of Object.entries(hook.hooks)) {
    routes.push({
      domain: parseInt(domain),
      route: await deployHook(networkId, ctx, client, v),
    });
  }

  const customRoutes: {
    dest_domain: number;
    recipient: string;
    hook: ContextHook;
  }[] = [];
  for (const [domain, v] of Object.entries(hook.custom_hooks)) {
    customRoutes.push({
      dest_domain: parseInt(domain),
      recipient: v.recipient,
      hook: await deployHook(networkId, ctx, client, v.hook),
    });
  }

  await executeMultiMsg(client, [
    {
      contract: routing,
      msg: {
        router: {
          set_routes: {
            set: routes.map((v) => ({
              domain: v.domain,
              route: v.route.address,
            })),
          },
        },
      },
    },
    {
      contract: routing,
      msg: {
        register_custom_hooks: customRoutes.map((v) => ({
          ...v,
          hook: v.hook.address,
        })),
      },
    },
  ]);

  return {
    ...routing,
    hooks: routes.reduce((acc, v) => ({
      [v.domain]: {
        default: v.route,
        ...customRoutes
          .filter((r) => r.dest_domain === v.domain)
          .map(({ recipient, hook }) => ({ recipient, hook }))
          .reduce((acc, r) => ({ [r.recipient]: r.hook, ...acc })),
      },
      ...acc,
    })),
  };
};

const deployFallbackRoitingHook = async (
  networkId: string,
  ctx: Context,
  client: Client,
  hook: RoutingFallbackHookType,
): Promise<ContextHook> => {
  const routing = await deployContract(
    ctx,
    client,
    'hpl_hook_routing_fallback',
    {
      owner: hook.owner === '<signer>' ? client.signer : hook.owner,
    },
  );

  const fallback = await deployHook(networkId, ctx, client, hook.fallback_hook);

  // if no hooks, return empty hooks
  if (Object.keys(hook.hooks).length === 0)
    return { ...routing, hooks: { fallback } };

  const routes = await Promise.all(
    Object.entries(hook.hooks).map(async ([domain, v]) => ({
      domain: parseInt(domain),
      route: await deployHook(networkId, ctx, client, v),
    })),
  );

  await executeMultiMsg(client, [
    {
      contract: routing,
      msg: {
        router: {
          set_routes: {
            set: routes.map((v) => ({
              domain: v.domain,
              route: v.route.address,
            })),
          },
        },
      },
    },
    {
      contract: routing,
      msg: {
        set_fallback_hook: { hook: fallback.address },
      },
    },
  ]);

  return {
    ...routing,
    hooks: {
      fallback,
      ...routes.reduce((acc, v) => ({ [v.domain]: v.route, ...acc })),
    },
  };
};

export const deployHook = async (
  networkId: string,
  ctx: Context,
  client: Client,
  hook: HookType,
): Promise<ContextHook> => {
  switch (hook.type) {
    // deploy fee hook
    case 'fee': {
      const { gas } = getNetwork(networkId);

      return deployContract(ctx, client, 'hpl_hook_fee', {
        owner: hook.owner === '<signer>' ? client.signer : hook.owner,
        fee: {
          denom: hook.fee.denom || gas.denom,
          amount: hook.fee.amount.toString(),
        },
      });
    }

    // deploy merkle hook
    case 'merkle': {
      return deployContract(ctx, client, 'hpl_hook_merkle', {
        mailbox: ctx.deployments.core?.mailbox?.address,
      });
    }

    // deploy mock hook
    case 'mock': {
      return deployContract(ctx, client, 'hpl_test_mock_hook', {});
    }

    // deploy pausable hook
    case 'pausable': {
      return deployContract(ctx, client, 'hpl_hook_pausable', {
        owner: hook.owner === '<signer>' ? client.signer : hook.owner,
        paused: hook.paused || false,
      });
    }

    // deploy igp hook
    case 'igp': {
      return deployIgp(networkId, ctx, client, hook);
    }

    // deploy aggregate hook
    case 'aggregate': {
      const aggr = [];
      for (const v of hook.hooks) {
        aggr.push(await deployHook(networkId, ctx, client, v));
      }

      const aggregate = await deployContract(
        ctx,
        client,
        'hpl_hook_aggregate',
        {
          owner: hook.owner === '<signer>' ? client.signer : hook.owner,
          hooks: aggr.map((v) => v.address),
        },
      );

      return { ...aggregate, hooks: aggr };
    }

    // deploy routing hook
    case 'routing': {
      return deployRoutingHook(networkId, ctx, client, hook);
    }

    // deploy custom routing hook
    case 'routing-custom': {
      return deployCustomRoutingHook(networkId, ctx, client, hook);
    }

    // deploy fallback routing hook
    case 'routing-fallback': {
      return deployFallbackRoitingHook(networkId, ctx, client, hook);
    }

    default: {
      throw new Error(`invalid hook type ${hook}`);
    }
  }
};
