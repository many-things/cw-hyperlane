import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplTestMockIsm extends BaseContract {
  contractName: string = "hpl_test_mock_ism";
}
