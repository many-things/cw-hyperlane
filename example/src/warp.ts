import { 
  HypERC20__factory, 
  StaticMessageIdMultisigIsmFactory__factory,
} from '@hyperlane-xyz/core';
import { Command, Option} from 'commander';
import { isAddress } from 'viem';

import { HYP_MAILBOX, HYP_MULTSIG_ISM_FACTORY } from './constants';
import { CONTAINER, Dependencies } from './ioc';
import {
  expectNextContractAddr,
  extractByte32AddrFromBech32,
  logTx,
} from './utils';

const warpCmd = new Command('warp');

warpCmd.command('deploy')
  .option(
    '--contract-name <contract-name>', 
    'Warp contract name e.g. Hyperlane Bridged TIA',
    'Hyperlane Bridged Osmo'
  )
  .option(
    '--asset-name <asset-name>', 
    'Warp route asset name e.g. TIA',
    'TIA'
  )
  .option(
    '--create-new-ism', 
    'Option to create a new ISM for the the warp route',
    false
  )
  .option(
    '--warp-ism-address <warp-ism-address>', 
    'ISM to set on the warp route recipient'
  )
  .option(
    '--ism-validator-address <ism-validator-address>', 
    'Validator address on the ism',
  )
  .action(deployWarpRoute);

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

type DeployWarpRouteArgs = {
  contractName: string,
  assetName: string,
  createNewIsm?: boolean,
  warpIsmAddress?: `0x${string}`,
  ismValidatorAddress?: `0x${string}`,
};

async function deployWarpRoute({
  contractName,
  assetName,
  createNewIsm,
  warpIsmAddress,
  ismValidatorAddress,
}: DeployWarpRouteArgs) {
  const {
    account,
    provider: { query, exec },
  } = CONTAINER.get(Dependencies);

  if (createNewIsm && warpIsmAddress !== undefined) {
    throw new Error("invalid options: cannot create a new ISM and pass a custom ISM address at the same time")
  }

  // deploy hyp erc20 (implementation)
  const hypErc20Addr = await expectNextContractAddr(query, account);
  console.log(`Deploying HypERC20 at "${hypErc20Addr.green}"...`);

  {
    const tx = await exec.deployContract({
      abi: HypERC20__factory.abi,
      bytecode: HypERC20__factory.bytecode,
      args: [6, HYP_MAILBOX],
    });
    logTx('Deploying HypERC20', tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  {
    const tx = await exec.writeContract({
      abi: HypERC20__factory.abi,
      address: hypErc20Addr,
      functionName: 'initialize',
      args: [0n, contractName ? contractName : 'Hyperlane Bridged OSMO', assetName ? assetName : 'OSMO'],
    });
    logTx('Initialize HypERC20', tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  // If the option was specifed to create a new ISM, deploy the multisig ISM contract
  if (createNewIsm) {
    ismValidatorAddress = ismValidatorAddress ? ismValidatorAddress : account.address

    const multisigIsmAddr = await query.readContract({
      abi: StaticMessageIdMultisigIsmFactory__factory.abi,
      address: HYP_MULTSIG_ISM_FACTORY,
      functionName: 'getAddress',
      args: [[ismValidatorAddress], 1],
    });
    console.log(`Deploying multisigIsm at "${multisigIsmAddr.green}"...`);
  
    {
      const tx = await exec.writeContract({
        abi: StaticMessageIdMultisigIsmFactory__factory.abi,
        address: HYP_MULTSIG_ISM_FACTORY,
        functionName: 'deploy',
        args: [[ismValidatorAddress], 1],
      });
      logTx('Deploy multisig ism', tx);
      await query.waitForTransactionReceipt({ hash: tx });
    }

    warpIsmAddress = multisigIsmAddr
  }
  
  // If a custom ISM address was specified or if a new ISM was created,
  // register that address in the warp contract
  // Otherwise, the default ISM will be used
  if (warpIsmAddress !== undefined) {
    const tx = await exec.writeContract({
      abi: HypERC20__factory.abi,
      address: hypErc20Addr,
      functionName: 'setInterchainSecurityModule',
      args: [warpIsmAddress],
    });
    logTx('Set ism for warp route', tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  console.log('== Done! ==');

  console.log({
    hypErc20: hypErc20Addr,
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
