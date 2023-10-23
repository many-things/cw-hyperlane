/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, OwnableMsg, HexBinary, RouterMsgForAddr, Addr, PostDispatchMsg, DomainRouteSetForAddr, QueryMsg, OwnableQueryMsg, RouterQueryForAddr, Order, HookQueryMsg, QuoteDispatchMsg, DomainsResponse, OwnerResponse, PendingOwnerResponse, RouteResponseForAddr, RoutesResponseForAddr, MailboxResponse, Empty, Uint128, QuoteDispatchResponse, Coin } from "./HookRoutingFallback.types";
export interface HookRoutingFallbackReadOnlyInterface {
  contractAddress: string;
  ownable: (ownableQueryMsg: OwnableQueryMsg) => Promise<OwnableResponse>;
  router: (routerQueryForAddr: RouterQueryForAddr) => Promise<RouterResponse>;
  hook: (hookQueryMsg: HookQueryMsg) => Promise<HookResponse>;
}
export class HookRoutingFallbackQueryClient implements HookRoutingFallbackReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.ownable = this.ownable.bind(this);
    this.router = this.router.bind(this);
    this.hook = this.hook.bind(this);
  }

  ownable = async (ownableQueryMsg: OwnableQueryMsg): Promise<OwnableResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      ownable: ownableQueryMsg
    });
  };
  router = async (routerQueryForAddr: RouterQueryForAddr): Promise<RouterResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      router: routerQueryForAddr
    });
  };
  hook = async (hookQueryMsg: HookQueryMsg): Promise<HookResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      hook: hookQueryMsg
    });
  };
}
export interface HookRoutingFallbackInterface extends HookRoutingFallbackReadOnlyInterface {
  contractAddress: string;
  sender: string;
  ownable: (ownableMsg: OwnableMsg, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  postDispatch: ({
    message,
    metadata
  }: {
    message: HexBinary;
    metadata: HexBinary;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  router: (routerMsgForAddr: RouterMsgForAddr, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  setFallbackHook: ({
    hook
  }: {
    hook: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class HookRoutingFallbackClient extends HookRoutingFallbackQueryClient implements HookRoutingFallbackInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.ownable = this.ownable.bind(this);
    this.postDispatch = this.postDispatch.bind(this);
    this.router = this.router.bind(this);
    this.setFallbackHook = this.setFallbackHook.bind(this);
  }

  ownable = async (ownableMsg: OwnableMsg, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      ownable: ownableMsg
    }, fee, memo, _funds);
  };
  postDispatch = async ({
    message,
    metadata
  }: {
    message: HexBinary;
    metadata: HexBinary;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      post_dispatch: {
        message,
        metadata
      }
    }, fee, memo, _funds);
  };
  router = async (routerMsgForAddr: RouterMsgForAddr, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      router: routerMsgForAddr
    }, fee, memo, _funds);
  };
  setFallbackHook = async ({
    hook
  }: {
    hook: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_fallback_hook: {
        hook
      }
    }, fee, memo, _funds);
  };
}