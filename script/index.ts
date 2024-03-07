import colors from 'colors';
import { Command, Option } from 'commander';
import 'reflect-metadata';

import { version } from '../package.json';
import { contractCmd, deployCmd, migrateCmd, uploadCmd } from './commands';
import { config, getSigningClient } from './shared/config';
import { loadContext } from './shared/context';
import { CONTAINER, Dependencies } from './shared/ioc';

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

cli.addCommand(uploadCmd);
cli.addCommand(deployCmd);
cli.addCommand(contractCmd);
cli.addCommand(migrateCmd);

cli.parseAsync(process.argv).catch(console.error);

async function injectDependencies(cmd: Command): Promise<void> {
  const { networkId } = cmd.optsWithGlobals();

  const client = await getSigningClient(networkId, config);
  const ctx = loadContext(networkId);

  CONTAINER.bind(Dependencies).toConstantValue({
    ctx,
    client,
  });
}
