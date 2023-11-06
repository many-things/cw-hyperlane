import { injectable } from "inversify";
import { Context, Migration } from "../types";
import {
  HplMailbox,
  HplHookMerkle,
  HplIgpGasOracle,
  HplIsmMultisig,
  HplTestMockHook,
} from "../contracts";

@injectable()
export default class InitializeStandalone implements Migration {
  name: string = "initialize_standalone";
  after: string = "";

  constructor(
    private ctx: Context,
    private mailbox: HplMailbox,
    private hook_merkle: HplHookMerkle,
    private igp: HplIgp,
    private igp_oracle: HplIgpGasOracle,
    private ism_multisig: HplIsmMultisig,
    private test_mock_hook: HplTestMockHook
  ) {}

  run = async (): Promise<Context> => {
    // init mailbox
    this.ctx.contracts[this.mailbox.contractName] =
      await this.mailbox.instantiate({
        hrp: "dual",
        owner: this.ctx.address!,
        domain: 33333,
      });

    // init merkle hook - (required hook)
    this.ctx.contracts[this.hook_merkle.contractName] =
      await this.hook_merkle.instantiate({
        owner: this.ctx.address!,
        mailbox: this.ctx.contracts[this.mailbox.contractName].address,
      });

    // init mock hook - (default hook)
    this.ctx.contracts[this.test_mock_hook.contractName] =
      await this.test_mock_hook.instantiate({});

    // init igp oracle
    this.ctx.contracts[this.igp_oracle.contractName] =
      await this.igp_oracle.instantiate({
        owner: this.ctx.address!,
      });

    // init igp
    this.ctx.contracts[this.igp.contractName] = await this.igp.instantiate({
      hrp: "dual",
      owner: this.ctx.address!,
      mailbox: this.ctx.contracts[this.mailbox.contractName].address,
      gas_token: "token",
      beneficiary: this.ctx.address!,
    });

    // init ism multisig
    this.ctx.contracts[this.ism_multisig.contractName] =
      await this.ism_multisig.instantiate({
        hrp: "dual",
        owner: this.ctx.address!,
      });

    return this.ctx;
  };

  setContext = (ctx: Context) => {
    this.ctx = ctx;
  };
}
