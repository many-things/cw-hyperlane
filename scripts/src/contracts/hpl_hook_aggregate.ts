import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplHookAggregate extends BaseContract {
  contractName: string = "hpl_hook_aggregate";
}
