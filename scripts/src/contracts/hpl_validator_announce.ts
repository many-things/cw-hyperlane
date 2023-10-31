import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplValidatorAnnounce extends BaseContract {
  contractName: string = "hpl_validator_announce";
}
