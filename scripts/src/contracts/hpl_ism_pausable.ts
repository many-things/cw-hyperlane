import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplIsmPausable extends BaseContract {
  contractName: string = "hpl_ism_pausable";
}
