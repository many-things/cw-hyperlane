import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplHookRoutingCustom extends BaseContract {
  contractName: string = "hpl_hook_routing_custom";
}
