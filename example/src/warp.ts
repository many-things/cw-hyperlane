import { Command } from "commander";
import { Hex, isAddress } from "viem";

import { HypERC20 } from "../abi/HypERC20";

import { HYP_MAILBOX } from "./constants";
import { CONTAINER, Dependencies } from "./ioc";
import {
  expectNextContractAddr,
  extractByte32AddrFromBech32,
  logTx,
} from "./utils";

const warpCmd = new Command("warp");

warpCmd.command("deploy").action(deployWarpRoute);
warpCmd
  .command("link")
  .argument("<warp>", "address of warp route")
  .argument("<domain>", "destination domain to set")
  .argument("<route>", "destination address to set")
  .action(linkWarpRoute);

export { warpCmd };

async function deployWarpRoute() {
  const {
    account,
    provider: { query, exec },
  } = CONTAINER.get(Dependencies);

  // deploy hyp erc20 (implementation)

  const hypErc20OsmoAddr = await expectNextContractAddr(query, account);
  console.log(`Deploying HypERC20 at "${hypErc20OsmoAddr.green}"...`);

  {
    const tx = await exec.deployContract({
      abi: HypERC20.abi,
      bytecode: HypERC20.bytecode.object as Hex,
      args: [6, HYP_MAILBOX],
    });
    logTx("Deploying HypERC20Osmo", tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  {
    const tx = await exec.writeContract({
      abi: HypERC20.abi,
      address: hypErc20OsmoAddr,
      functionName: "initialize",
      args: [0n, "Hyperlane Bridged Osmosis", "OSMO"],
    });
    logTx("Initialize HypERC20Osmo", tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  console.log("== Done! ==");

  console.log({
    hypErc20Osmo: hypErc20OsmoAddr,
  });
}

async function linkWarpRoute(warp: string, domain: string, route: string) {
  const {
    provider: { exec, query },
  } = CONTAINER.get(Dependencies);

  if (!isAddress(warp)) throw new Error("Invalid warp address");

  const tx = await exec.writeContract({
    abi: HypERC20.abi,
    address: warp,
    functionName: "enrollRemoteRouter",
    args: [parseInt(domain), `0x${extractByte32AddrFromBech32(route)}`],
  });
  logTx(`Linking warp route with external chain ${domain}`, tx);
  await query.waitForTransactionReceipt({ hash: tx });
}
