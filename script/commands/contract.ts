import { Command } from "commander";
import { contractNames } from "../shared/constants";

export const contractCmd = new Command("contract");

contractCmd.command("list").action(() => {
  console.log("Available contracts:".green);
  contractNames.forEach((v) => console.log("-", v));
});
