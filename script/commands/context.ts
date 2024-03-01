import { Command } from "commander";

import { CONTAINER, Dependencies } from "../shared/ioc";
import { saveAgentConfig } from "../shared/agent";
import { getNetwork } from "../shared/config";

const contextCmd = new Command("context");

contextCmd
  .command("make-agent-config")
  .description("Make an agent config")
  .option("-o --output <output-dir>", "The output directory")
  .action(async (_, cmd) => {
    const opts = cmd.optsWithGlobals();
    const { ctx } = CONTAINER.get(Dependencies);
    const network = getNetwork(opts.networkId);

    await saveAgentConfig(
      network,
      ctx,
      opts.output && { contextPath: opts.output }
    );
  });

export { contextCmd };
