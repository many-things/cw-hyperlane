import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplHookRouting extends BaseContract {
  contractName: string = "hpl_hook_routing";
}
