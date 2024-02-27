import { Hex, encodeFunctionData, isAddress, isHex } from "viem";

import { HypERC20 } from "../abi/HypERC20";
import { ProxyAdmin } from "../abi/ProxyAdmin";
import { TransparentUpgradeableProxy } from "../abi/TransparentUpgradeableProxy";

import { HYP_MAILBOX } from "./constants";
import { CONTAINER, Dependencies } from "./ioc";
import { addPad, expectNextContractAddr, logTx } from "./utils";
import { Command } from "commander";

const warpCmd = new Command("warp");

warpCmd.command("deploy-route").action(deployWarpRoute);
warpCmd
  .command("link-route")
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
  if (!isHex(route)) throw new Error("Invalid route address");

  const tx = await exec.writeContract({
    abi: HypERC20.abi,
    address: warp,
    functionName: "enrollRemoteRouter",
    args: [parseInt(domain), `0x${addPad(route)}`],
  });
  logTx(`Linking warp route with external chain ${domain}`, tx);
  await query.waitForTransactionReceipt({ hash: tx });
}
