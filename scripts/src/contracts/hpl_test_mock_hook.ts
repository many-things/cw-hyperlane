import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplTestMockHook extends BaseContract {
  contractName: string = "hpl_test_mock_hook";
}
