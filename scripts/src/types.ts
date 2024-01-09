import {
    ExecuteResult,
    SigningCosmWasmClient,
} from "@cosmjs/cosmwasm-stargate";
import { fromBech32 } from "@cosmjs/encoding";
import fs from "fs";
import { getWasmPath } from "./load_wasm";

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

  constructor(
    public address: string | undefined,
    public codeId: number | undefined,
    public digest: string,
    public signer: string,
    public client: SigningCosmWasmClient
  ) {}

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
    const contract = await this.client.instantiate(
      this.signer,
      this.codeId!,
      msg,
      this.contractName,
      "auto",
      { admin: this.signer }
    );

    console.log(
      [
        this.contractName.padEnd(30),
        contract.contractAddress.padEnd(65),
        Buffer.from(fromBech32(contract.contractAddress).data)
          .toString("hex")
          .padEnd(65),
        contract.transactionHash.padEnd(65),
      ].join("| ")
    );

    this.address = contract.contractAddress;
    return this.getContractContext();
  }

  // overloads
  public async execute(msg: any): Promise<ExecuteResult>;
  public async execute(
    msg: any,
    funds: { denom: string; amount: string }[]
  ): Promise<ExecuteResult>;

  // implementation
  public async execute(
    msg: any,
    funds?: { denom: string; amount: string }[]
  ): Promise<ExecuteResult> {
    const res = await this.client.execute(
      this.signer,
      this.address!,
      msg,
      "auto",
      undefined,
      funds
    );
    console.log(
      [
        `${this.contractName}:${Object.keys(msg)[0]}`.padEnd(30),
        res.transactionHash.padEnd(65),
      ].join("| ")
    );

    return res;
  }

  public async query<T>(msg: any): Promise<T> {
    const res = await this.client.queryContractSmart(this.address!, msg);

    return res;
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

export interface HplIgpOracleInstantiateMsg {}

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
