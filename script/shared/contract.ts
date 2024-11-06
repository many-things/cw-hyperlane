import { IndexedTx } from '@cosmjs/stargate';
import { MsgExecuteContract, MsgInstantiateContract } from '@injectivelabs/sdk-ts';

import { Client } from './config';
import { contractNames } from './constants';
import { Context } from './context';
import { Logger } from './logger';
import { extractByte32AddrFromBech32, sleep, waitTx } from './utils';

const logger = new Logger('contract');

export type ContractNames = (typeof contractNames)[number];

export async function deployContract<T extends ContractNames>(
  ctx: Context,
  client: Client,
  contractName: T,
  initMsg: object,
  retryAfter = 1000,
): Promise<{ type: T; address: string; hexed: string }> {
  const { wasm, stargate, signer, injective, injective_signer } = client;
  logger.debug(`deploying ${contractName}`);

  try {
    const codeId = ctx.artifacts[contractName];

    const msg = MsgInstantiateContract.fromJSON({
      sender: injective_signer,
      admin: injective_signer,
      codeId: ctx.artifacts[contractName],
      msg: initMsg,
      label: `cw-hpl: ${contractName}`,
    });

    const resp = await injective.broadcast({
      msgs: msg,
    });

    const instantiateEvent = resp.events!.find(
      (event) => event.type === 'cosmwasm.wasm.v1.EventContractInstantiated',
    );

    const address = JSON.parse(
      new TextDecoder().decode(instantiateEvent.attributes[2].value),
    );

    return {
      type: contractName,
      address,
      hexed: extractByte32AddrFromBech32(address),
    };

    const res = await wasm.instantiate(
      signer,
      codeId,
      initMsg,
      `cw-hpl: ${contractName}`,
      'auto',
    );
    const receipt = await waitTx(res.transactionHash, stargate);
    if (receipt.code > 0) {
      logger.error(
        'deploy tx failed.',
        `contract=${contractName}, hash=${receipt.hash}`,
      );
      throw new Error(JSON.stringify(receipt.events));
    }

    logger.info(`deployed ${contractName} at ${res.contractAddress}`);
    return {
      type: contractName,
      address: res.contractAddress,
      hexed: extractByte32AddrFromBech32(res.contractAddress),
    };
  } catch (e) {
    logger.error(`failed to deploy contract. retrying after ${retryAfter}ms`);
    logger.error('=> error: ', e);
    await sleep(retryAfter);
    return deployContract(ctx, client, contractName, initMsg, retryAfter * 2);
  }
}

export async function executeContract(
  { wasm, stargate, signer }: Client,
  deployment: { type: ContractNames; address: string },
  msg: object,
  funds: { amount: string; denom: string }[] = [],
): Promise<IndexedTx> {
  logger.debug(`executing ${deployment.type}'s ${Object.keys(msg)[0]}`);

  const res = await wasm.execute(
    signer,
    deployment.address,
    msg,
    'auto',
    undefined,
    funds,
  );
  const receipt = await waitTx(res.transactionHash, stargate);
  if (receipt.code > 0) {
    logger.error(
      'execute tx failed.',
      `contract=${deployment.type}, hash=${receipt.hash}`,
    );
    throw new Error(JSON.stringify(receipt.events));
  }

  logger.info(`executed ${deployment.type}'s ${Object.keys(msg)[0]}`);
  return receipt;
}

export async function executeMultiMsg(
  { wasm, stargate, signer, injective, injective_signer }: Client,
  msgs: { contract: { type: ContractNames; address: string }; msg: object }[],
): Promise<IndexedTx> {
  const long = msgs
    .map((v) => v.contract.type.length)
    .reduce((max, v) => Math.max(v, max), 0);

  logger.debug(
    `executing ${msgs.length} msgs.\n`,
    ...msgs.flatMap((v, i, arr) => [
      `- ${v.contract.type.padEnd(long, ' ')}:`,
      `${Object.keys(v.msg)[0]}${i === arr.length - 1 ? '' : '\n'}`,
    ]),
  );

  let executeMessages = msgs.map((v) => MsgExecuteContract.fromJSON({
    sender: injective_signer,
    contractAddress: v.contract.address,
    msg: v.msg,
  }));

  // avoid exceeding block gas limit
  const CHUNK_SIZE = 20;
  while (executeMessages.length > 0) {
    const chunk = executeMessages.splice(0, CHUNK_SIZE);
    const resp = await injective.broadcast({
      msgs: chunk
    });
    console.log({resp});
  }

  // @ts-ignore
  return;

  const res = await wasm.executeMultiple(
    signer,
    msgs.map((v) => ({
      contractAddress: v.contract.address,
      msg: v.msg,
    })),
    'auto',
  );
  const receipt = await waitTx(res.transactionHash, stargate);
  if (receipt.code > 0) {
    logger.error(
      `execute multiple tx failed.`,
      `msgs=${msgs.length}, hash=${receipt.hash}`,
    );
    throw new Error(JSON.stringify(receipt.events));
  }

  logger.info(
    `executed ${msgs.length} msgs.\n`,
    ...msgs.flatMap((v, i, arr) => [
      `- ${v.contract.type.padEnd(long, ' ')}:`,
      `${Object.keys(v.msg)[0]}${i === arr.length - 1 ? '' : '\n'}`,
    ]),
  );
  return receipt;
}
