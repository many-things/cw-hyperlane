import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplWarpCw20 extends BaseContract {
  contractName: string = "hpl_warp_cw20";
}
