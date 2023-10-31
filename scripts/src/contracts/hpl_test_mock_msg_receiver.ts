import { injectable } from "inversify";
import { BaseContract } from "../types";

@injectable()
export class HplTestMockMsgReceiver extends BaseContract {
  contractName: string = "hpl_test_mock_msg_receiver";
}
