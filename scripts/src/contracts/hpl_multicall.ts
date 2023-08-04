import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Contract, ContractContext, HplMulticallInstantiateMsg } from "../types";
import { getWasmPath } from "../load_wasm";
import fs from "fs";


export default class HplMulticall implements Contract {
  contractName: string = "hpl_multicall"

  address: string | undefined;
  digest: string;
  codeId: number | undefined;
  client: SigningCosmWasmClient;
  signer: string;

  constructor(address: string | undefined, codeId: number | undefined, digest: string, signer: string, client: SigningCosmWasmClient) {
    this.address = address
    this.client = client
    this.digest = digest
    this.codeId = codeId
    this.signer = signer
  }

  getContractContext = (): ContractContext => {
    return {
      address: this.address,
      codeId: this.codeId,
      digest: this.digest
    }
  }

  uplaod = async(): Promise<ContractContext> => {
    const wasm = fs.readFileSync(getWasmPath(this.contractName));
    const uploadReceipt = await this.client.upload(this.signer, wasm, "auto");

    this.codeId = uploadReceipt.codeId;
    return this.getContractContext();
  }

  instantiate = async(msg: any): Promise<ContractContext> => {
    const instantiateMsg = msg as HplMulticallInstantiateMsg;
    const contract = await this.client.instantiate(this.signer, this.codeId!, instantiateMsg, this.contractName, "auto");

    this.address = contract.contractAddress;
    return this.getContractContext();
  }

}
