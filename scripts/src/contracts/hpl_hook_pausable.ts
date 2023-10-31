import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplHookPausable extends BaseContract {
  contractName: string = "hpl_hook_pausable";
}
