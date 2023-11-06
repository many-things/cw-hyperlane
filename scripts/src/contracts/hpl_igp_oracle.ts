import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplIgpOracle extends BaseContract {
  contractName: string = "hpl_igp_oracle";
}
