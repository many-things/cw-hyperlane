import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplMailbox extends BaseContract {
  contractName: string = "hpl_mailbox";
}
