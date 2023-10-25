/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { InstantiateMsg, ExecuteMsg, HexBinary, QueryMsg, GetAnnounceStorageLocationsResponse, GetAnnouncedValidatorsResponse } from "./ValidatorAnnounce.types";
export interface ValidatorAnnounceMsg {
  contractAddress: string;
  sender: string;
  announce: ({
    signature,
    storageLocation,
    validator
  }: {
    signature: HexBinary;
    storageLocation: string;
    validator: HexBinary;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class ValidatorAnnounceMsgComposer implements ValidatorAnnounceMsg {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.announce = this.announce.bind(this);
  }

  announce = ({
    signature,
    storageLocation,
    validator
  }: {
    signature: HexBinary;
    storageLocation: string;
    validator: HexBinary;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          announce: {
            signature,
            storage_location: storageLocation,
            validator
          }
        })),
        funds: _funds
      })
    };
  };
}