import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplIsmMultisig extends BaseContract {
  contractName: string = "hpl_ism_multisig";
}
