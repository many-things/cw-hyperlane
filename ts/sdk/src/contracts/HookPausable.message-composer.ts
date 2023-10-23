/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { InstantiateMsg, ExecuteMsg, OwnableMsg, PausableMsg, HexBinary, PostDispatchMsg, QueryMsg, PausableQueryMsg, OwnableQueryMsg, HookQueryMsg, QuoteDispatchMsg, Addr, OwnerResponse, PendingOwnerResponse, MailboxResponse, PauseInfoResponse, Uint128, QuoteDispatchResponse, Coin } from "./HookPausable.types";
export interface HookPausableMsg {
  contractAddress: string;
  sender: string;
  ownable: (ownableMsg: OwnableMsg, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  pausable: (pausableMsg: PausableMsg, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  postDispatch: ({
    message,
    metadata
  }: {
    message: HexBinary;
    metadata: HexBinary;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class HookPausableMsgComposer implements HookPausableMsg {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.ownable = this.ownable.bind(this);
    this.pausable = this.pausable.bind(this);
    this.postDispatch = this.postDispatch.bind(this);
  }

  ownable = (ownableMsg: OwnableMsg, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          ownable: ownableMsg
        })),
        funds: _funds
      })
    };
  };
  pausable = (pausableMsg: PausableMsg, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          pausable: pausableMsg
        })),
        funds: _funds
      })
    };
  };
  postDispatch = ({
    message,
    metadata
  }: {
    message: HexBinary;
    metadata: HexBinary;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          post_dispatch: {
            message,
            metadata
          }
        })),
        funds: _funds
      })
    };
  };
}