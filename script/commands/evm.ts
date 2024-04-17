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
  
  evmCmd.command('deploy-warp')
    .addOption(
      new Option(
        '--evm-network-name <evmNetworkName>',
        'specify the EVM network name',
      )
      .choices(config.evm_networks.map((v) => v.name))
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
    .option(
      '--ism-threshold <ism-threshold>', 
      'Threshold for the number of validators in the ISM',
      "1",
    )
    .action(deployWarpRoute);
  
  export { evmCmd };
  
  type DeployWarpRouteArgs = {
    evmNetworkName: string,
    contractName: string,
    assetName: string,
    createNewIsm?: boolean,
    warpIsmAddress?: `0x${string}`,
    ismValidatorAddress?: `0x${string}`,
    ismThreshold?: number,
  };
  
  async function deployWarpRoute({
    evmNetworkName,
    contractName,
    assetName,
    createNewIsm,
    warpIsmAddress,
    ismValidatorAddress,
    ismThreshold,
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
  
    // If the option was specifed to create a new ISM, deploy the multisig ISM contract
    if (createNewIsm) {
      ismValidatorAddress = ismValidatorAddress ? ismValidatorAddress : account.address
  
      const multisigIsmAddr = await query.readContract({
        abi: StaticMessageIdMultisigIsmFactory__factory.abi,
        address: evmNetwork.multisig_ism_factory_address,
        functionName: 'getAddress',
        args: [[ismValidatorAddress], Number(ismThreshold)],
      });
      console.log(`Deploying multisigIsm at "${multisigIsmAddr.green}"...`);
    
      {
        const tx = await exec.writeContract({
          abi: StaticMessageIdMultisigIsmFactory__factory.abi,
          address: evmNetwork.multisig_ism_factory_address,
          functionName: 'deploy',
          args: [[ismValidatorAddress], Number(ismThreshold)],
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
  