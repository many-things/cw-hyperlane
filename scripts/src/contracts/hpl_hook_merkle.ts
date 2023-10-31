import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplHookMerkle extends BaseContract {
  contractName: string = "hpl_hook_merkle";
}
