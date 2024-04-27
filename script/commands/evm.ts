import { 
  HypERC20__factory, 
  StaticMessageIdMultisigIsmFactory__factory,
} from '@hyperlane-xyz/core';
import { Command, Option } from 'commander';
import {
  Account,
  Chain,
  Hex,
  createPublicClient,
  createWalletClient,
  http,
} from 'viem';
import { mnemonicToAccount, privateKeyToAccount } from 'viem/accounts';
import { config, getEvmNetwork } from '../shared/config';
import {
  expectNextContractAddr,
  logTx,
} from '../../example/src/utils';

const evmCmd = new Command('evm')

evmCmd.command('deploy-ism')
  .addOption(
    new Option(
      '--evm-network-name <evmNetworkName>',
      'specify the EVM network name',
    )
    .choices(config.evm_networks ? config.evm_networks.map((v) => v.name) : [])
    .makeOptionMandatory()
  )
  .option(
    '--validator-addresses <validator-addresses>', 
    'Comma separated list of validator address on the ism',
  )
  .option(
    '--threshold <threshold>', 
    'Threshold for the number of validators in the ISM',
    "1",
  )
  .action(deployIsm);

evmCmd.command('deploy-warp')
  .addOption(
    new Option(
      '--evm-network-name <evmNetworkName>',
      'specify the EVM network name',
    )
    .choices(config.evm_networks ? config.evm_networks.map((v) => v.name) : [])
    .makeOptionMandatory()
  )
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
    '--ism-address <warp-ism-address>', 
    'ISM to set on the warp route recipient'
  )
  .action(deployWarpRoute);

export { evmCmd };

type DeployIsmArgs = {
  evmNetworkName: string,
  validatorAddresses?: `0x${string}`,
  threshold?: number,
};

async function deployIsm({
  evmNetworkName,
  validatorAddresses,
  threshold,
}: DeployIsmArgs) {
  const { signer } = config;
  const evmNetwork = getEvmNetwork(evmNetworkName)
  
  const account: Account =
    signer.split(' ').length > 1
      ? mnemonicToAccount(signer)
      : privateKeyToAccount(`0x${signer}` as Hex);

  const chain: Chain = {
    id: evmNetwork.chain_id,
    network: evmNetwork.network,
    name: evmNetwork.name,
    nativeCurrency: evmNetwork.native_currency,
    rpcUrls: {
      default: {
        http: [evmNetwork.rpc_endpoint],
      },
      public: {
        http: [evmNetwork.rpc_endpoint],
      },
    },
  }

  const query = createPublicClient({
    chain: chain,
    transport: http(evmNetwork.rpc_endpoint),
  })

  const exec = createWalletClient({
    chain: chain,
    account,
    transport: http(evmNetwork.rpc_endpoint),
  });

  const validatorAddressesString = validatorAddresses ? validatorAddresses : account.address
  const validatorAddressesList = validatorAddressesString.split(",").map(address => address as `0x${string}`)

  const multisigIsmAddr = await query.readContract({
    abi: StaticMessageIdMultisigIsmFactory__factory.abi,
    address: evmNetwork.multisig_ism_factory_address,
    functionName: 'getAddress',
    args: [validatorAddressesList, Number(threshold)],
  });
  console.log(`Multisig ISM Address to be deployed at: ${multisigIsmAddr.green}`);

  {
    const tx = await exec.writeContract({
      abi: StaticMessageIdMultisigIsmFactory__factory.abi,
      address: evmNetwork.multisig_ism_factory_address,
      functionName: 'deploy',
      args: [validatorAddressesList, Number(threshold)],
    });
    logTx('Deploying multisig ISM', tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  console.log(`\nMultisig ISM Address: ${multisigIsmAddr.blue}`);
}


type DeployWarpRouteArgs = {
  evmNetworkName: string,
  contractName: string,
  assetName: string,
  ismAddress?: `0x${string}`,
};

async function deployWarpRoute({
  evmNetworkName,
  contractName,
  assetName,
  ismAddress,
}: DeployWarpRouteArgs) {
  const { signer } = config;
  const evmNetwork = getEvmNetwork(evmNetworkName)
  
  const account: Account =
    signer.split(' ').length > 1
      ? mnemonicToAccount(signer)
      : privateKeyToAccount(`0x${signer}` as Hex);

  const chain: Chain = {
    id: evmNetwork.chain_id,
    network: evmNetwork.network,
    name: evmNetwork.name,
    nativeCurrency: evmNetwork.native_currency,
    rpcUrls: {
      default: {
        http: [evmNetwork.rpc_endpoint],
      },
      public: {
        http: [evmNetwork.rpc_endpoint],
      },
    },
  }

  const query = createPublicClient({
    chain: chain,
    transport: http(evmNetwork.rpc_endpoint),
  })

  const exec = createWalletClient({
    chain: chain,
    account,
    transport: http(evmNetwork.rpc_endpoint),
  });

  // deploy hyp erc20 (implementation)
  const hypErc20Addr = await expectNextContractAddr(query, account);
  console.log(`Deploying HypERC20 at "${hypErc20Addr.green}"...`);

  {
    const tx = await exec.deployContract({
      abi: HypERC20__factory.abi,
      bytecode: HypERC20__factory.bytecode,
      args: [6, evmNetwork.mailbox_address],
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
  
  // If a custom ISM address was specified, register that address in the warp contract
  // Otherwise, the default ISM will be used
  if (ismAddress !== undefined) {
    const tx = await exec.writeContract({
      abi: HypERC20__factory.abi,
      address: hypErc20Addr,
      functionName: 'setInterchainSecurityModule',
      args: [ismAddress],
    });
    logTx('Set ism for warp route', tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }
  
  console.log(`\nWarp ERC20: ${hypErc20Addr.blue}`);
}
