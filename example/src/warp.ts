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

  // deploy proxy admin
  const proxyAdminAddr = await expectNextContractAddr(query, account);
  console.log(`Deploying ProxyAdmin at "${proxyAdminAddr.green}"...`);

  {
    const tx = await exec.deployContract({
      abi: ProxyAdmin.abi,
      bytecode: ProxyAdmin.bytecode.object as Hex,
      args: [],
    });
    logTx("Deploying ProxyAdmin", tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  // deploy hyp erc20 (implementation)

  const hypErc20Addr = await expectNextContractAddr(query, account);
  console.log(`Deploying HypERC20 at "${hypErc20Addr.green}"...`);

  {
    const tx = await exec.deployContract({
      abi: HypERC20.abi,
      bytecode: HypERC20.bytecode.object as Hex,
      args: [6, HYP_MAILBOX],
    });
    logTx("Deploying HypERC20", tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  // deploy hyp erc20 (for uosmo!)

  const hypErc20UosmoAddr = await expectNextContractAddr(query, account);
  console.log(`Deploying HypERC20Uosmo at "${hypErc20UosmoAddr.green}"...`);

  {
    const tx = await exec.deployContract({
      abi: TransparentUpgradeableProxy.abi,
      bytecode: TransparentUpgradeableProxy.bytecode.object as Hex,
      args: [
        hypErc20Addr,
        proxyAdminAddr,
        encodeFunctionData({
          abi: HypERC20.abi,
          functionName: "initialize",
          args: [0n, "Hyperlane Bridged Osmosis", "OSMO"],
        }),
      ],
    });
    logTx("Deploy HypERC20Osmo", tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  console.log("== Done! ==");

  console.log({
    proxyAdmin: proxyAdminAddr,
    hypErc20: hypErc20Addr,
    hypErc20Uosmo: hypErc20UosmoAddr,
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
    args: [parseInt(domain), addPad(route) as Hex],
  });
  logTx(`Linking warp route with external chain ${domain}`, tx);
  await query.waitForTransactionReceipt({ hash: tx });
}
