export * from "./hpl_hook_aggregate";
export * from "./hpl_hook_merkle";
export * from "./hpl_hook_pausable";
export * from "./hpl_hook_routing";
export * from "./hpl_hook_routing_custom";
export * from "./hpl_hook_routing_fallback";
export * from "./hpl_igp";
export * from "./hpl_igp_oracle";
export * from "./hpl_ism_aggregate";
export * from "./hpl_ism_multisig";
export * from "./hpl_ism_pausable";
export * from "./hpl_ism_routing";
export * from "./hpl_mailbox";
export * from "./hpl_test_mock_hook";
export * from "./hpl_test_mock_msg_receiver";
export * from "./hpl_validator_announce";
export * from "./hpl_warp_cw20";
export * from "./hpl_warp_native";

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { readdirSync } from "fs";
import { Container } from "inversify";
import { Context, Contract, ContractConstructor } from "../types";

const contractNames: string[] = readdirSync(__dirname)
  .filter((f) => f !== "index.ts")
  .map((f) => f.replace(".ts", ""));

const contractHandlers: { [key: string]: ContractConstructor } =
  contractNames.reduce((acc, cur) => {
    const className = cur
      .split("_")
      .map((v) => v[0].toUpperCase() + v.slice(1))
      .join("");

    acc[cur] = require(`./${cur}`)[className];
    return acc;
  }, {} as { [key: string]: ContractConstructor });

export function getTargetContractName(): string[] {
  return contractNames;
}

export function getTargetContract(
  ctx: Context,
  client: SigningCosmWasmClient,
  signer: string,
  container: Container
): { [key: string]: Contract } {
  return Object.keys(contractHandlers).reduce((acc, cur) => {
    const { codeId, digest, address } = ctx.contracts[cur] || {};

    try {
      acc[cur] = new contractHandlers[cur](
        address,
        codeId,
        digest,
        signer,
        client
      );
    } catch (e) {
      throw Error(`Failed to instantiate contract ${cur}: ${e}`);
    }

    container.bind(contractHandlers[cur]).toConstantValue(acc[cur]);
    return acc;
  }, {} as { [key: string]: Contract });
}
