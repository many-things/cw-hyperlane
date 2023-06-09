/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, CosmosMsgForEmpty, BankMsg, Uint128, StakingMsg, DistributionMsg, Binary, IbcMsg, Timestamp, Uint64, WasmMsg, GovMsg, VoteOption, HexBinary, Coin, Empty, IbcTimeout, IbcTimeoutBlock, HandleMsg, QueryMsg, StaticCall, MigrateMsg, AggregateResponse } from "./Multicall.types";
export interface MulticallReadOnlyInterface {
  contractAddress: string;
  aggregateStatic: ({
    req
  }: {
    req: StaticCall[];
  }) => Promise<AggregateResponse>;
}
export class MulticallQueryClient implements MulticallReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.aggregateStatic = this.aggregateStatic.bind(this);
  }

  aggregateStatic = async ({
    req
  }: {
    req: StaticCall[];
  }): Promise<AggregateResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      aggregate_static: {
        req
      }
    });
  };
}
export interface MulticallInterface extends MulticallReadOnlyInterface {
  contractAddress: string;
  sender: string;
  aggregate: ({
    req
  }: {
    req: CosmosMsgForEmpty[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  handle: ({
    body,
    origin,
    sender
  }: {
    body: HexBinary;
    origin: number;
    sender: HexBinary;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class MulticallClient extends MulticallQueryClient implements MulticallInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.aggregate = this.aggregate.bind(this);
    this.handle = this.handle.bind(this);
  }

  aggregate = async ({
    req
  }: {
    req: CosmosMsgForEmpty[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      aggregate: {
        req
      }
    }, fee, memo, funds);
  };
  handle = async ({
    body,
    origin,
    sender
  }: {
    body: HexBinary;
    origin: number;
    sender: HexBinary;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      handle: {
        body,
        origin,
        sender
      }
    }, fee, memo, funds);
  };
}