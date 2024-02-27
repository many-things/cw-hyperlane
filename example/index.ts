import "reflect-metadata";
import colors from "colors";

import { Command } from "commander";

import { warpCmd } from "./src/warp";
import { recipientCmd } from "./src/recipient";
import { injectDependencies } from "./src/ioc";

import { version } from "../package.json";

colors.enable();

const cli = new Command();

cli
  .name("cw-hpl-example")
  .version(version)
  .configureHelp({ showGlobalOptions: true })
  .option("--pk --private-key <privateKey>", "private key")
  .option("--mn --mnemonic <mnemonic>", "mnemonic phrase")
  .option("--rpc --endpoint <endpoint>", "endpoint")
  .hook("preAction", injectDependencies)
  .description("CosmWasm Hyperlane Examples");

cli.addCommand(warpCmd);
cli.addCommand(recipientCmd);

cli.parseAsync(process.argv).catch(console.error);
