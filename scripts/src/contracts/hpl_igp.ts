import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplIgp extends BaseContract {
  contractName: string = "hpl_igp";
}
