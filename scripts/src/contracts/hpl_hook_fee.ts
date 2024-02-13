import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplHookFee extends BaseContract {
  contractName: string = "hpl_hook_fee";
}
