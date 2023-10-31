import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplIsmRouting extends BaseContract {
  contractName: string = "hpl_ism_routing";
}
