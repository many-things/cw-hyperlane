import { Config } from "./config";
import { Context, ContextHook } from "./context";
import { extractByte32AddrFromBech32 } from "./utils";
import { getContractInfo } from "./wasm";

export type HplAgentConfig = {
  name: string;
  domainId: string;
  chainId: string;
  protocol: "cosmos";
  rpcUrls: { http: string }[];
  grpcUrl: string;
  canonicalAsset: string; // usually same as gas token
  bech32Prefix: string; // hrp
  gasPrice: {
    amount: string;
    denom: string;
  };
  contractAddressBytes: 32;
  index: {
    from?: number;
    chunk: number;
  };
  blocks: {
    reorgPeriod: 0; // instant finality ⭐️
  };

  mailbox: string; // hexed
  interchainGasPaymaster: string; // hexed
  validatorAnnounce: string; // hexed
  merkleTreeHook: string; // hexed
  testRecipient: string; // hexed
};

export async function fromContext(
  network: Config["networks"][number],
  context: Context
): Promise<HplAgentConfig> {
  const toHex = (v: string) => `0x${extractByte32AddrFromBech32(v)}`;

  const { hooks, core, test } = context.deployments;

  const mailboxAddr = core?.mailbox?.address!;
  const mailboxContractInfo = await getContractInfo(network, mailboxAddr);

  const igp =
    findHook(hooks?.default!, "hpl_igp") ||
    findHook(hooks?.required!, "hpl_igp");
  if (!igp) throw new Error("no igp on this context");

  const merkleTreeHook =
    findHook(hooks?.default!, "hpl_hook_merkle") ||
    findHook(hooks?.required!, "hpl_hook_merkle");
  if (!merkleTreeHook) throw new Error("no merkle tree hook on this context");

  const agent: HplAgentConfig = {
    name: network.id.split("-").join(""),
    domainId: network.domain.toString(),
    chainId: network.id,
    protocol: "cosmos",

    rpcUrls: [{ http: network.endpoint.rpc }],
    grpcUrl: network.endpoint.grpc,
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
        mailboxContractInfo &&
        parseInt(mailboxContractInfo.contract_info.created.block_height) - 1,
      chunk: 10_000,
    },
    blocks: {
      reorgPeriod: 1,
    },

    // contract addresses
    mailbox: toHex(core?.mailbox?.address!),
    validatorAnnounce: toHex(core?.validator_announce?.address!),
    interchainGasPaymaster: toHex(igp.address),
    merkleTreeHook: toHex(merkleTreeHook.address),
    testRecipient: toHex(test?.msg_receiver?.address!),
  };

  return agent;
}

// map filter reverse pop
function mfrpHooks(
  hooks: ContextHook[],
  want: ContextHook["type"]
): ContextHook | undefined {
  return hooks
    .map((v) => findHook(v, want))
    .filter((v) => v !== undefined)
    .reverse()
    .pop();
}

function findHook(
  hook: ContextHook,
  want: ContextHook["type"]
): ContextHook | undefined {
  if (hook.type === want) return hook;

  switch (hook.type) {
    case "hpl_hook_aggregate":
      return mfrpHooks(hook.hooks, want);
    case "hpl_hook_routing":
      return mfrpHooks(Object.values(hook.hooks), want);
    case "hpl_hook_routing_custom":
      return mfrpHooks(
        Object.values(hook.hooks)
          .map((v) => Object.values(v))
          .flat(),
        want
      );
    case "hpl_hook_routing_fallback":
      return mfrpHooks(Object.values(hook.hooks), want);

    default:
      return undefined;
  }
}
