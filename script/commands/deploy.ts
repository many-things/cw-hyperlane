import { Command } from 'commander';

import { deployHook, deployIsm } from '../deploy';
import { Client, config, getNetwork } from '../shared/config';
import {
  Context,
  ContextDeployments,
  saveAgentConfig,
  saveContext,
} from '../shared/context';
import { deployContract, executeMultiMsg } from '../shared/contract';
import { CONTAINER, Dependencies } from '../shared/ioc';

export const deployCmd = new Command('deploy')
  .description('Deploy contracts')
  .configureHelp({ showGlobalOptions: true })
  .action(handleDeploy);

async function handleDeploy(_: any, cmd: any) {
  const opts = cmd.optsWithGlobals();
  const { ctx, client } = CONTAINER.get(Dependencies);

  ctx.deployments = ctx.deployments || {};
  ctx.deployments.core = await deployCore(opts, ctx, client);
  ctx.deployments.isms = await deployIsms(ctx, client);
  ctx.deployments.hooks = await deployHooks(opts, ctx, client);
  ctx.deployments.warp = { native: [], cw20: [] };
  ctx.deployments.test = await deployTest(opts, ctx, client);

  await executeMultiMsg(client, [
    {
      contract: ctx.deployments.core?.mailbox!,
      msg: {
        set_default_ism: {
          ism: ctx.deployments.isms?.address!,
        },
      },
    },
    {
      contract: ctx.deployments.core?.mailbox!,
      msg: {
        set_default_hook: {
          hook: ctx.deployments.hooks?.default!.address,
        },
      },
    },
    {
      contract: ctx.deployments.core?.mailbox!,
      msg: {
        set_required_hook: {
          hook: ctx.deployments.hooks?.required!.address,
        },
      },
    },
  ]);

  saveContext(opts.networkId, ctx);
  saveAgentConfig(getNetwork(opts.networkId), ctx);
}

const deployCore = async (
  { networkId }: { networkId: string },
  ctx: Context,
  client: Client,
): Promise<ContextDeployments['core']> => {
  const { hrp, domain } = getNetwork(networkId);

  const log = (v: string) => console.log('[core]'.green, v);
  const preload = ctx.deployments.core;
  const deployment = preload || {};

  deployment.mailbox =
    preload?.mailbox ||
    (await deployContract(ctx, client, 'hpl_mailbox', {
      hrp,
      domain,
      owner: client.signer,
    }));
  if (preload?.mailbox) log(`${deployment.mailbox.type} already deployed`);

  deployment.validator_announce =
    preload?.validator_announce ||
    (await deployContract(ctx, client, 'hpl_validator_announce', {
      hrp,
      mailbox: deployment.mailbox.address,
    }));
  if (preload?.validator_announce)
    log(`${deployment.validator_announce.type} already deployed`);

  return deployment;
};

const deployIsms = async (
  ctx: Context,
  client: Client,
): Promise<ContextDeployments['isms']> => {
  if (!config.deploy.ism) {
    throw new Error('ISM deployment config not found');
  }

  const log = (v: string) => console.log('[ism]'.green, v);
  const preload = ctx.deployments.isms;
  const deployment =
    preload ||
    (await deployIsm(
      ctx,
      client,
      config.deploy.ism instanceof Array
        ? {
            // default ism (multisig)
            type: 'multisig',
            owner: client.signer,
            validators: config.deploy.ism
              .map((domain) => ({
                domain,
                addrs: [client.signer_addr],
                threshold: 1,
              }))
              .reduce(
                (acc, v) => ({
                  [v.domain]: { addrs: v.addrs, threshold: v.threshold },
                  ...acc,
                }),
                {} as Record<string, any>,
              ),
          }
        : config.deploy.ism,
    ));
  if (preload) {
    log(`ism ${deployment.type} already deployed`);
  }

  return deployment;
};

const deployHooks = async (
  { networkId }: { networkId: string },
  ctx: Context,
  client: Client,
): Promise<ContextDeployments['hooks']> => {
  if (!config.deploy.hooks) {
    throw new Error('Hook deployment config not found');
  }

  const log = (v: string) => console.log('[hooks]'.green, v);
  const preload = ctx.deployments?.hooks;
  const deployment = preload || {};

  deployment.default =
    preload?.default ||
    (await deployHook(
      networkId,
      ctx,
      client,
      config.deploy.hooks.default || { type: 'mock' },
    ));
  if (preload?.default)
    log(`default hook ${deployment.default.type} already deployed`);

  deployment.required =
    preload?.required ||
    (await deployHook(
      networkId,
      ctx,
      client,
      config.deploy.hooks.required || { type: 'mock' },
    ));
  if (preload?.required)
    log(`required hook ${deployment.required.type} already deployed`);

  return deployment;
};

const deployTest = async (
  { networkId }: { networkId: string },
  ctx: Context,
  client: Client,
): Promise<ContextDeployments['test']> => {
  const { hrp } = getNetwork(networkId);

  const log = (v: string) => console.log('[test]'.green, v);
  const preload = ctx.deployments.test;
  const deployment = preload || {};

  deployment.msg_receiver =
    preload?.msg_receiver ||
    (await deployContract(ctx, client, 'hpl_test_mock_msg_receiver', {
      hrp,
    }));
  if (preload?.msg_receiver)
    log(`${deployment.msg_receiver.type} already deployed`);

  return deployment;
};
