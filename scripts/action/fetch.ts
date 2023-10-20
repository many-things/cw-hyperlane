import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Context } from "../src/types";
import { Client } from "../src/config";

type Const<T> = new (
  address: string | undefined,
  codeId: number | undefined,
  digest: string,
  signer: string,
  client: SigningCosmWasmClient
) => T;

export class ContractFetcher {
  constructor(private ctx: Context, private client: Client) {}

  public get<T>(f: Const<T>, name: string): T {
    return new f(
      this.ctx.contracts[name].address,
      this.ctx.contracts[name].codeId,
      this.ctx.contracts[name].digest,
      this.client.signer,
      this.client.wasm
    );
  }
}
