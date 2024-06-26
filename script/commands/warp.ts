import { Command, Option } from 'commander';
import { readFileSync } from 'fs';

import {
  WarpTokenConfig,
  deployCw20TokenWarp,
  deployNativeTokenWarp,
} from '../deploy';
import { saveContext } from '../shared/context';
import { executeContract } from '../shared/contract';
import { CONTAINER, Dependencies } from '../shared/ioc';
import { addPad } from '../shared/utils';

const warpCmd = new Command('warp')
  .description('Hyperlane warp route commands')
  .configureHelp({ showGlobalOptions: true });

warpCmd
  .command('create')
  .description('Create a new warp route')
  .argument('<config-file>', 'path to the warp route config file')
  .addOption(
    new Option(
      '--ism <ism-address>',
       'ISM to set on warp route (in bech32 format)',
      )
  )
  .action(handleCreate);

warpCmd
  .command('link')
  .description('Link a warp route with external chain')
  .addOption(
    new Option(
      '--asset-type <asset-type>',
      'type of asset, it can be native or cw20',
    )
      .makeOptionMandatory()
      .choices(['native', 'cw20']),
  )
  .addOption(
    new Option(
      '--asset-id <asset-id>',
      'asset id to link with warp route',
    ).makeOptionMandatory(),
  )
  .addOption(
    new Option(
      '--target-domain <target-domain>',
      'target domain id to link',
    ).makeOptionMandatory(),
  )
  .addOption(
    new Option(
      '--warp-address <warp-address>',
      'warp contract address to link with',
    ).makeOptionMandatory(),
  )
  .action(handleLink);

warpCmd
  .command('transfer')
  .description('Transfer a warp route to external chain')
  .addOption(
    new Option(
      '--asset-type <asset-type>',
      'type of asset, it can be native or cw20',
    )
      .makeOptionMandatory()
      .choices(['native', 'cw20']),
  )
  .addOption(
    new Option(
      '--asset-id <asset-id>',
      'asset id to link with warp route',
    ).makeOptionMandatory(),
  )
  .addOption(
    new Option(
      '--target-domain <target-domain>',
      'target domain id to link',
    ).makeOptionMandatory(),
  )
  .addOption(
    new Option(
      '--amount <amount>',
      'amount to send',
    )
  )
  .addOption(
    new Option(
      '--bridged-denom <bridged-denom>',
      'denom to transfer'
    )
  )
  .addOption(
    new Option(
      '--fee-denom <fee-denom>',
      'fee denom'
    )
  )
  .action(handleTransfer);

export { warpCmd };

function checkConfigType<
  TokenType extends 'native' | 'cw20',
  TokenMode extends 'bridged' | 'collateral',
>(
  config: WarpTokenConfig,
  tokenType: TokenType,
  tokenMode: TokenMode,
): config is WarpTokenConfig<typeof tokenType, typeof tokenMode> {
  return config.type === tokenType && config.mode === tokenMode;
}

async function handleCreate(configFile: string, _: object, cmd: Command) {
  type Option = {
    ismAddress?: `0x{string}`;
  };

  const opts: Option = cmd.optsWithGlobals();
  const deps = CONTAINER.get(Dependencies);

  const warpConfigFile = readFileSync(configFile, 'utf-8');
  const warpConfig: WarpTokenConfig = JSON.parse(warpConfigFile);

  const { type: warpType, mode } = warpConfig;

  const mailbox = deps.ctx.deployments?.core?.mailbox?.address;
  if (!mailbox) {
    console.error(
      '[error]'.red,
      'mailbox contract not yet deployed.',
      'how about run `deploy` command first?',
    );
    return;
  }

  deps.ctx.deployments.warp = deps.ctx.deployments.warp || {
    native: [],
    cw20: [],
  };

  type WarpContract = {
    type: string,
    address: string,
    hexed: string
  }
  let newWarp: WarpContract;

  switch (warpType) {
    case 'native': {
      if (!checkConfigType(warpConfig, 'native', mode))
        throw Error('Invalid wrap config type. This cannot be happended');

      const nativeWarp = await deployNativeTokenWarp(deps, mailbox, warpConfig);
      if (!nativeWarp) {
        console.log('[error]'.red, 'failed to deploy native warp contract');
        return;
      }

      deps.ctx.deployments.warp?.native?.push({
        id: warpConfig.id,
        ...nativeWarp,
      });
      newWarp = nativeWarp;
      break;
    }
    case 'cw20': {
      if (!checkConfigType(warpConfig, 'cw20', warpConfig.mode))
        throw Error('Invalid wrap config type. This cannot be happended');

      const cw20Warp = await deployCw20TokenWarp(deps, mailbox, warpConfig);
      if (!cw20Warp) {
        console.log('[error]'.red, 'failed to deploy cw20 warp contract');
        return;
      }

      deps.ctx.deployments.warp?.cw20?.push({
        id: warpConfig.id,
        ...cw20Warp,
      });
      newWarp = cw20Warp;
      break;
    }
  }

  if (opts.ismAddress) {
    console.log(`Setting ISM address to ${opts.ismAddress}`)
    const response = await executeContract(
      deps.client,
      newWarp,
      {
        connection: {
          set_ism: {
            ism: opts.ismAddress
          }
        }
      }
    );
    console.log(`Code: ${response.code}, Hash: ${response.hash}`);
  }

  saveContext(deps.network.id, deps.ctx);
}

async function handleLink(_: object, cmd: Command) {
  type Option = {
    assetType: 'native' | 'cw20';
    assetId: string;
    targetDomain: string;
    warpAddress: string;
  };

  const opts: Option = cmd.optsWithGlobals();
  const deps = CONTAINER.get(Dependencies);

  const warp = deps.ctx.deployments.warp;
  if (!warp)
    throw new Error(
      [
        '[error]'.red,
        'warp contract is not deployed.',
        'Run `warp create` first.',
      ].join(' '),
    );

  deps.ctx.deployments.warp = {
    ...warp,
    [opts.assetType]: warp[opts.assetType as 'native' | 'cw20'] || [],
  };

  const routes = deps.ctx.deployments.warp[opts.assetType] || [];
  const route = routes.find((v) => v.id === opts.assetId);
  if (!route) {
    console.error(
      '[error]'.red,
      `warp route with id ${opts.assetId} not found.`,
    );
    return;
  }

  const linkResp = await executeContract(deps.client, route, {
    router: {
      set_route: {
        set: {
          domain: parseInt(opts.targetDomain),
          route: addPad(opts.warpAddress),
        },
      },
    },
  });

  console.log(linkResp.hash);
}

async function handleTransfer(_: object, cmd: Command) {
  type Option = {
    assetType: 'native' | 'cw20';
    assetId: string;
    targetDomain: string;
    amount?: number;
    bridgedDenom?: string;
    feeDenom?: string;
  };

  const opts: Option = cmd.optsWithGlobals();
  const deps = CONTAINER.get(Dependencies);

  const warp = deps.ctx.deployments.warp;
  if (!warp)
    throw new Error(
      [
        '[error]'.red,
        'warp contract is not deployed.',
        'Run `warp create` first.',
      ].join(' '),
    );

  deps.ctx.deployments.warp = {
    ...warp,
    [opts.assetType]: warp[opts.assetType as 'native' | 'cw20'] || [],
  };

  const routes = deps.ctx.deployments.warp[opts.assetType] || [];
  const route = routes.find((v) => v.id === opts.assetId);
  if (!route) {
    console.error(
      '[error]'.red,
      `warp route with id ${opts.assetId} not found.`,
    );
    return;
  }

  await executeContract(
    deps.client,
    route,
    {
      transfer_remote: {
        dest_domain: parseInt(opts.targetDomain),
        recipient: addPad(deps.client.signer_addr),
        amount: opts.amount ? `${opts.amount}` : `${1_000_000n}`,
      },
    },
    [
      {
        amount: opts.amount ? `${opts.amount}` : `${1_000_000n}`,
        denom: opts.bridgedDenom || 'uosmo' 
      },
      {
        amount: `${50n}`,
        denom: opts.feeDenom || 'uosmo' 
      },
    ],
  );
}
