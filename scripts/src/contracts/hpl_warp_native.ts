import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplWarpNative extends BaseContract {
  contractName: string = "hpl_warp_native";
}
