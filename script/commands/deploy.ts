import { Command } from 'commander';

import { MsgExecuteContract } from '@injectivelabs/sdk-ts';
import { deployHook, deployIsm } from '../deploy';
import { saveAgentConfig } from '../shared/agent';
import { Client, config, getNetwork } from '../shared/config';
import { Context, ContextDeployments, saveContext } from '../shared/context';
import { deployContract, executeMultiMsg } from '../shared/contract';
import { CONTAINER, Dependencies } from '../shared/ioc';

export const deployCmd = new Command('deploy')
  .description('Deploy contracts')
  .configureHelp({ showGlobalOptions: true })
  .action(handleDeploy);

async function handleDeploy(_: object, cmd: Command) {
  const opts = cmd.optsWithGlobals() as { networkId: string };
  const { ctx, client } = CONTAINER.get(Dependencies);

  ctx.deployments = ctx.deployments || {};
  ctx.deployments.core = await deployCore(opts, ctx, client);
  ctx.deployments.isms = await deployIsms(ctx, client);
  // ctx.deployments.hooks = await deployHooks(opts, ctx, client);
  // ctx.deployments.warp = { native: [], cw20: [] };
  // ctx.deployments.test = await deployTest(opts, ctx, client);

  // const updateMsg = MsgUpdateAdmin.fromJSON({
  //   sender: client.injective_signer,
  //   newAdmin: 'inj1ac6qpt57vhtfzdecd2an052elwgenwtxcn9chl',
  //   contract: ctx.deployments.isms?.address!,
  // });

  const initMsg = MsgExecuteContract.fromJSON({
    contractAddress: ctx.deployments.isms?.address!,
    sender: client.injective_signer,
    msg: {
      ownable: {
        init_ownership_transfer: {
          next_owner: 'inj1ac6qpt57vhtfzdecd2an052elwgenwtxcn9chl'
        }
      }
    }
  })

  const resp = await client.injective.broadcast({
    msgs: initMsg,
  });

  console.log({resp});

  if (!ctx.deployments.core?.mailbox)
    throw new Error('deployed Mailbox contract not found on context');

  if (!ctx.deployments.isms)
    throw new Error('deployed ISM contract not found on context');

  if (!ctx.deployments.hooks?.default)
    throw new Error('deployed Default Hook contract not found on context');

  if (!ctx.deployments.hooks?.required)
    throw new Error('deployed Required Hook contract not found on context');

  await executeMultiMsg(client, [
    {
      contract: ctx.deployments.core.mailbox,
      msg: {
        set_default_ism: {
          ism: ctx.deployments.isms.address,
        },
      },
    },
    {
      contract: ctx.deployments.core.mailbox,
      msg: {
        set_default_hook: {
          hook: ctx.deployments.hooks.default.address,
        },
      },
    },
    {
      contract: ctx.deployments.core.mailbox,
      msg: {
        set_required_hook: {
          hook: ctx.deployments.hooks.required.address,
        },
      },
    },
  ]);

  saveContext(opts.networkId, ctx);
  await saveAgentConfig(getNetwork(opts.networkId), ctx);
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

  if (preload?.mailbox) {
    log(`${preload.mailbox.type} already deployed`);
    deployment.mailbox = preload.mailbox;
  } else {
    deployment.mailbox = await deployContract(ctx, client, 'hpl_mailbox', {
      hrp,
      domain,
      owner: client.signer,
    });
  }

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
  if (!config?.deploy?.ism) {
    throw new Error('ISM deployment config not found');
  }

  const log = (v: string) => console.log('[ism]'.green, v);
  const preload = ctx.deployments.isms;
  if (preload) {
    log(`ism ${preload.type} already deployed`);
    return preload;
  }

  return deployIsm(ctx, client, config.deploy.ism);
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

  if (preload?.default) {
    log(`default hook ${preload.default.type} already deployed`);
    deployment.default = preload.default;
  } else {
    if (!config.deploy.hooks.default)
      throw Error('Default hook deployment config not found');

    deployment.default = await deployHook(
      networkId,
      ctx,
      client,
      config.deploy.hooks.default,
    );
  }

  if (preload?.required) {
    log(`required hook ${preload.required.type} already deployed`);
    deployment.required = preload.required;
  } else {
    if (!config.deploy.hooks.required)
      throw Error('Required hook deployment config not found');

    deployment.required = await deployHook(
      networkId,
      ctx,
      client,
      config.deploy.hooks.required,
    );
  }

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
