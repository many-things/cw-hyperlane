import { injectable } from "inversify";
import {
  Context,
  HplHubInstantiateMsg,
  HplIgpCoreInstantiateMsg,
  HplIsmMultisigInstantiateMsg,
  Migration,
} from "../types";
import HplMailbox from "../contracts/hpl_mailbox";
import HplIgpGasOracle from "../contracts/hpl_igp_oracle";
import HplIgpCore from "../contracts/hpl_igp";
import HplIsmMultisig from "../contracts/hpl_ism_multisig";

@injectable()
export default class InitializeStandalone implements Migration {
  name: string = "initialize_standalone";
  after: string = "";

  constructor(
    private ctx: Context,
    private mailbox: HplMailbox,
    private gas_oracle: HplIgpGasOracle,
    private igp_core: HplIgpCore,
    private ism_multisig: HplIsmMultisig
  ) {}

  run = async (): Promise<Context> => {
    // init mailbox
    this.ctx.contracts[this.mailbox.contractName] =
      await this.mailbox.instantiate({});

    // init gas oracle
    this.ctx.contracts[this.gas_oracle.contractName] =
      await this.gas_oracle.instantiate({});

    // init gas core
    const igpInit: HplIgpCoreInstantiateMsg = {
      owner: this.ctx.address!,
      gas_token: "osmo",
      beneficiary: this.ctx.address!,
    };
    this.ctx.contracts[this.igp_core.contractName] =
      await this.igp_core.instantiate(igpInit);

    // init ism multisig
    const ismMultisigInit: HplIsmMultisigInstantiateMsg = {
      owner: this.ctx.address!,
      addr_prefix: "osmo",
    };
    this.ctx.contracts[this.ism_multisig.contractName] =
      await this.ism_multisig.instantiate(ismMultisigInit);

    return this.ctx;
  };

  setContext = (ctx: Context) => {
    this.ctx = ctx;
  };
}
