import { AgentConfig } from '@hyperlane-xyz/sdk';
import { ProtocolType } from '@hyperlane-xyz/utils';
import fs from 'fs';
import path from 'path';

import { Config } from './config';
import { defaultContextPath } from './constants';
import { Context, ContextHook } from './context';
import { extractByte32AddrFromBech32 as fromBech32 } from './utils';
import { getContractInfo } from './wasm';

export async function fromContext(
  network: Config['networks'][number],
  context: Context,
): Promise<AgentConfig> {
  const { hooks, core } = context.deployments;

  const mailboxAddr = core?.mailbox?.address!;
  const mailboxContractInfo = await getContractInfo(network, mailboxAddr);

  const igp =
    findHook(hooks?.default!, 'hpl_igp') ||
    findHook(hooks?.required!, 'hpl_igp');
  if (!igp) throw new Error('no igp on this context');

  const merkleTreeHook =
    findHook(hooks?.default!, 'hpl_hook_merkle') ||
    findHook(hooks?.required!, 'hpl_hook_merkle');
  if (!merkleTreeHook) throw new Error('no merkle tree hook on this context');

  const agent: AgentConfig = {
    chains: {
      [network.id.split('-').join('')]: {
        name: network.id.split('-').join(''),
        domainId: network.domain,
        chainId: network.id,
        protocol: ProtocolType.Cosmos,

        rpcUrls: [{ http: network.endpoint.rpc }],
        grpcUrls: [{ http: network.endpoint.grpc }],
        canonicalAsset: network.gas.denom,
        bech32Prefix: network.hrp,

        gasPrice: {
          amount: network.gas.price,
          denom: network.gas.denom,
        },
        contractAddressBytes: 32,

        index: {
          from:
            // sub 1 block to make sure we don't miss any block
            parseInt(mailboxContractInfo.contract_info.created.block_height) -
            1,
          chunk: 10_000,
        },
        blocks: {
          confirmations: 0,
          reorgPeriod: 0,
        },

        // contract addresses
        mailbox: fromBech32(core?.mailbox?.address!),
        validatorAnnounce: fromBech32(core?.validator_announce?.address!),
        interchainGasPaymaster: fromBech32(igp.address),
        merkleTreeHook: fromBech32(merkleTreeHook.address),
      },
    },
  };

  return agent;
}

export async function saveAgentConfig(
  network: Config['networks'][number],
  context: Context,
  { contextPath }: { contextPath: string } = {
    contextPath: defaultContextPath,
  },
): Promise<AgentConfig> {
  const agentConfig = await fromContext(network, context);
  const fileName = path.join(contextPath, `${network.id}.config.json`);
  fs.writeFileSync(fileName, JSON.stringify(agentConfig, null, 2));
  return agentConfig;
}

// map filter reverse pop
function mfrpHooks(
  hooks: ContextHook[],
  want: ContextHook['type'],
): ContextHook | undefined {
  return hooks
    .map((v) => findHook(v, want))
    .filter((v) => v !== undefined)
    .reverse()
    .pop();
}

function findHook(
  hook: ContextHook,
  want: ContextHook['type'],
): ContextHook | undefined {
  if (hook.type === want) return hook;

  switch (hook.type) {
    case 'hpl_hook_aggregate':
      return mfrpHooks(hook.hooks, want);
    case 'hpl_hook_routing':
      return mfrpHooks(Object.values(hook.hooks), want);
    case 'hpl_hook_routing_custom':
      return mfrpHooks(
        Object.values(hook.hooks)
          .map((v) => Object.values(v))
          .flat(),
        want,
      );
    case 'hpl_hook_routing_fallback':
      return mfrpHooks(Object.values(hook.hooks), want);

    default:
      return undefined;
  }
}
