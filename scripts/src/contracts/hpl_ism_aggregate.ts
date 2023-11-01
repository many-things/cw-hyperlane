import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplIsmAggregate extends BaseContract {
  contractName: string = "hpl_ism_aggregate";
}
