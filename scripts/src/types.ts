import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";


export interface ContractContext {
  codeId: number | undefined,
  digest: string,
  address: string | undefined,
}

export interface Context {
  contracts: {
    [key: string]: ContractContext,
  }
}

export interface CodeCreate {
  contractName: string,
  digest: string,
}

export interface CodeUpdate {
  contractName: string,
  codeId: number,
  digest: string,
}

export type CodeUploads = CodeCreate | CodeUpdate;


export interface Contract {
  address: string | undefined;
  codeId: number | undefined;
  contractName: string;
  digest: string;
  client: SigningCosmWasmClient;
  signer: string;

  uplaod(): Promise<ContractContext>;
  instantiate(msg: any): Promise<ContractContext>;
}

export interface ContractConstructor {
  new(address: string | undefined, codeId: number | undefined, digest: string, signer: string, client: SigningCosmWasmClient): Contract;
}

export interface HplHubInstantiateMsg {
  origin_domain: number,
  mailbox_code: number,
}

export interface HplIgpCoreInstantiateMsg {
  owner: string,
  gas_token: string,
  beneficiary: string,
}

export interface HplIgpGasOracleInstantiateMsg {}

export interface HplIsmMultisigInstantiateMsg {
  owner: string,
  addr_prefix: string,
}

export interface HplIsmRoutingInstantiateMsg {
  owner: string,
  isms: {
    domain: number,
    address: string,
  }[],
}

export interface HplMailboxInstantiateMsg {
  owner: string,
  default_ism: string,
}

export interface HplMulticallInstantiateMsg {
  owner: string,
  mailbox: string,
}

export interface HplValidatorAnnounceInstantiateMsg {
  addr_prefix: string,
  mailbox: string,
  local_domain: number, // u32
}
