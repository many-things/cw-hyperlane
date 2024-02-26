import { contractNames } from "./constants";
import { Context } from "./context";
import { waitTx } from "./utils";
import { Client } from "./config";
import { IndexedTx } from "@cosmjs/stargate";

export type ContractNames = (typeof contractNames)[number];

export async function deployContract<T extends ContractNames>(
  ctx: Context,
  { wasm, stargate, signer }: Client,
  contractName: T,
  initMsg: any
): Promise<{ type: T; address: string }> {
  console.log(`Deploying ${contractName}`);

  const codeId = ctx.artifacts[contractName];
  const res = await wasm.instantiate(
    signer,
    codeId,
    initMsg,
    `cw-hpl: ${contractName}`,
    "auto"
  );
  const receipt = await waitTx(res.transactionHash, stargate);
  if (receipt.code > 0) {
    throw new Error(`Error deploying ${contractName}: ${receipt.hash}`);
  }

  console.log(`Deployed ${contractName} at ${res.contractAddress}`);
  return { type: contractName, address: res.contractAddress };
}

export async function executeContract(
  { wasm, stargate, signer }: Client,
  deployment: { type: ContractNames; address: string },
  msg: any,
  funds: { amount: string; denom: string }[] = []
): Promise<IndexedTx> {
  console.log(`Executing ${deployment.type}'s ${Object.keys(msg)[0]}`);

  const res = await wasm.execute(
    signer,
    deployment.address,
    msg,
    "auto",
    undefined,
    funds
  );
  const receipt = await waitTx(res.transactionHash, stargate);
  if (receipt.code > 0) {
    throw new Error(`Error executing ${deployment.type}: ${receipt.hash}`);
  }

  return receipt;
}

export async function executeMultiMsg(
  { wasm, stargate, signer }: Client,
  msgs: { contract: { type: ContractNames; address: string }; msg: any }[]
): Promise<IndexedTx> {
  const long = msgs
    .map((v) => v.contract.type.length)
    .reduce((max, v) => Math.max(v, max), 0);

  console.log("Executing multiple messages.");
  for (const msg of msgs) {
    console.log(
      `- ${msg.contract.type.padEnd(long, " ")}: ${Object.keys(msg.msg)[0]}`
    );
  }

  const res = await wasm.executeMultiple(
    signer,
    msgs.map((v) => ({
      contractAddress: v.contract.address,
      msg: v.msg,
    })),
    "auto"
  );
  const receipt = await waitTx(res.transactionHash, stargate);
  if (receipt.code > 0) {
    throw new Error(`Error executing multiple contracts: ${receipt.hash}`);
  }
  return receipt;
}
