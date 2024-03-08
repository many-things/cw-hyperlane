import colors from 'colors';
import { Command } from 'commander';
import 'reflect-metadata';

import { version } from '../package.json';
import { injectDependencies } from './src/ioc';
import { recipientCmd } from './src/recipient';
import { warpCmd } from './src/warp';

colors.enable();

const cli = new Command();

cli
  .name('cw-hpl-example')
  .version(version)
  .configureHelp({ showGlobalOptions: true })
  .option('--pk --private-key <privateKey>', 'private key')
  .option('--mn --mnemonic <mnemonic>', 'mnemonic phrase')
  .option('--rpc --endpoint <endpoint>', 'endpoint')
  .hook('preAction', injectDependencies)
  .description('CosmWasm Hyperlane Examples');

cli.addCommand(warpCmd);
cli.addCommand(recipientCmd);

cli.parseAsync(process.argv).catch(console.error);
