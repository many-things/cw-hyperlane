import colors from 'colors';
import { Command, Option } from 'commander';
import 'reflect-metadata';

import { version } from '../package.json';
import {
  contextCmd,
  contractCmd,
  evmCmd,
  deployCmd,
  migrateCmd,
  uploadCmd,
  walletCmd,
  warpCmd,
} from './commands';
import { config, getNetwork, getSigningClient } from './shared/config';
import { loadContext } from './shared/context';
import { CONTAINER, Dependencies } from './shared/ioc';
import { updateCmd } from './commands/update';

colors.enable();

const optNetworkId = new Option(
  '-n, --network-id <networkId>',
  'specify network id',
)
  .choices(config.networks.map((v) => v.id))
  .makeOptionMandatory();

const cli = new Command();

cli
  .name('cw-hpl')
  .version(version)
  .description('CLI toolkit for CosmWasm Hyperlane')
  .addOption(optNetworkId)
  .hook('preAction', injectDependencies);

cli.addCommand(contextCmd);
cli.addCommand(contractCmd);
cli.addCommand(deployCmd);
cli.addCommand(migrateCmd);
cli.addCommand(updateCmd);
cli.addCommand(uploadCmd);
cli.addCommand(walletCmd);
cli.addCommand(warpCmd);
cli.addCommand(evmCmd);

cli.parseAsync(process.argv).catch(console.error);

async function injectDependencies(cmd: Command): Promise<void> {
  const { networkId } = cmd.optsWithGlobals();

  const client = await getSigningClient(networkId, config);
  const ctx = loadContext(networkId);
  const network = getNetwork(networkId);

  const deps = { ctx, client, network };

  CONTAINER.bind(Dependencies).toConstantValue(deps);
}
