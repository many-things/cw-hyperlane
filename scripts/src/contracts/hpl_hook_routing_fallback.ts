import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplHookRoutingFallback extends BaseContract {
  contractName: string = "hpl_hook_routing_fallback";
}
