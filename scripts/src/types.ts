import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { getWasmPath } from "./load_wasm";
import fs from "fs";

export interface ContractContext {
  codeId: number | undefined;
  digest: string;
  address: string | undefined;
}

export class Context {
  contracts: {
    [key: string]: ContractContext;
  };
  latestMigration: string | undefined;
  address: string | undefined;
}

export interface CodeCreate {
  contractName: string;
  digest: string;
}

export interface CodeUpdate {
  contractName: string;
  codeId: number;
  digest: string;
}

export type CodeUploads = CodeCreate | CodeUpdate;

export interface Contract {
  address: string | undefined;
  codeId: number | undefined;
  contractName: string;
  digest: string;
  client: SigningCosmWasmClient;
  signer: string;

  getContractContext(): ContractContext;

  upload(): Promise<ContractContext>;
  instantiate(msg: any): Promise<ContractContext>;
}

export abstract class BaseContract implements Contract {
  contractName: string;

  address: string | undefined;
  codeId: number | undefined;
  digest: string;
  client: SigningCosmWasmClient;
  signer: string;

  constructor(
    address: string | undefined,
    codeId: number | undefined,
    digest: string,
    signer: string,
    client: SigningCosmWasmClient
  ) {
    this.address = address;
    this.client = client;
    this.digest = digest;
    this.codeId = codeId;
    this.signer = signer;
  }

  public getContractContext(): ContractContext {
    return {
      address: this.address,
      codeId: this.codeId,
      digest: this.digest,
    };
  }

  public async upload(): Promise<ContractContext> {
    const wasm = fs.readFileSync(getWasmPath(this.contractName));
    const uploadReceipt = await this.client.upload(this.signer, wasm, "auto");

    this.codeId = uploadReceipt.codeId;
    return this.getContractContext();
  }

  public async instantiate(msg: any): Promise<ContractContext> {
    const instantiateMsg = msg as HplMailboxInstantiateMsg;
    const contract = await this.client.instantiate(
      this.signer,
      this.codeId!,
      instantiateMsg,
      this.contractName,
      "auto",
      { admin: this.signer }
    );

    this.address = contract.contractAddress;
    return this.getContractContext();
  }
}

export interface ContractConstructor {
  new (
    address: string | undefined,
    codeId: number | undefined,
    digest: string,
    signer: string,
    client: SigningCosmWasmClient
  ): Contract;
}

export interface Migration {
  name: string;
  after: string;

  run(): Promise<Context>;
  setContext(ctx: Context): void;
}

export interface HplHubInstantiateMsg {
  origin_domain: number;
  mailbox_code: number;
}

export interface HplIgpCoreInstantiateMsg {
  owner: string;
  gas_token: string;
  beneficiary: string;
}

export interface HplIgpGasOracleInstantiateMsg {}

export interface HplIsmMultisigInstantiateMsg {
  owner: string;
  addr_prefix: string;
}

export interface HplIsmRoutingInstantiateMsg {
  owner: string;
  isms: {
    domain: number;
    address: string;
  }[];
}

export interface HplMailboxInstantiateMsg {
  owner: string;
  hrp: string;
  domain: number;
}

export interface HplMulticallInstantiateMsg {
  owner: string;
  mailbox: string;
}

export interface HplValidatorAnnounceInstantiateMsg {
  addr_prefix: string;
  mailbox: string;
  local_domain: number; // u32
}
