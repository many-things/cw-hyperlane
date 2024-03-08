import { HypERC20__factory } from '@hyperlane-xyz/core';
import { Command } from 'commander';
import { isAddress } from 'viem';

import { HYP_MAILBOX } from './constants';
import { CONTAINER, Dependencies } from './ioc';
import {
  expectNextContractAddr,
  extractByte32AddrFromBech32,
  logTx,
} from './utils';

const warpCmd = new Command('warp');

warpCmd.command('deploy').action(deployWarpRoute);
warpCmd
  .command('link')
  .argument('<warp>', 'address of warp route')
  .argument('<domain>', 'destination domain to set')
  .argument('<route>', 'destination address to set')
  .action(linkWarpRoute);

warpCmd
  .command('transfer')
  .argument('<warp>', 'address of warp route')
  .argument('<domain>', 'destination domain to transfer')
  .argument('<to>', 'address to transfer')
  .action(transferWarpRoute);

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
      abi: HypERC20__factory.abi,
      bytecode: HypERC20__factory.bytecode,
      args: [6, HYP_MAILBOX],
    });
    logTx('Deploying HypERC20Osmo', tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  {
    const tx = await exec.writeContract({
      abi: HypERC20__factory.abi,
      address: hypErc20OsmoAddr,
      functionName: 'initialize',
      args: [0n, 'Hyperlane Bridged Osmosis', 'OSMO'],
    });
    logTx('Initialize HypERC20Osmo', tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  console.log('== Done! ==');

  console.log({
    hypErc20Osmo: hypErc20OsmoAddr,
  });
}

async function linkWarpRoute(warp: string, domain: string, route: string) {
  const {
    provider: { exec, query },
  } = CONTAINER.get(Dependencies);

  if (!isAddress(warp)) throw new Error('Invalid warp address');

  const tx = await exec.writeContract({
    abi: HypERC20__factory.abi,
    address: warp,
    functionName: 'enrollRemoteRouter',
    args: [parseInt(domain), `0x${extractByte32AddrFromBech32(route)}`],
  });
  logTx(`Linking warp route with external chain ${domain}`, tx);
  await query.waitForTransactionReceipt({ hash: tx });
}

async function transferWarpRoute(warp: string, domain: string, to: string) {
  const {
    provider: { exec, query },
  } = CONTAINER.get(Dependencies);

  if (!isAddress(warp)) throw new Error('Invalid warp address');

  const tx = await exec.writeContract({
    abi: HypERC20__factory.abi,
    address: warp,
    functionName: 'transferRemote',
    args: [
      parseInt(domain),
      `0x${extractByte32AddrFromBech32(to)}`,
      1_000_000n,
    ],
  });
  logTx(`Transferring warp route with external chain ${domain}`, tx);
  await query.waitForTransactionReceipt({ hash: tx });
}
