import {
  TestRecipient__factory,
  StaticMessageIdMultisigIsmFactory__factory,
} from "@hyperlane-xyz/core";

import { CONTAINER, Dependencies } from "./ioc";
import { expectNextContractAddr, logTx } from "./utils";
import { HYP_MULTSIG_ISM_FACTORY } from "./constants";
import { Command } from "commander";

export const recipientCmd = new Command("deploy-test-recipient").action(
  deployTestRecipient
);

async function deployTestRecipient() {
  const {
    account,
    provider: { query, exec },
  } = CONTAINER.get(Dependencies);

  const testRecipientAddr = await expectNextContractAddr(query, account);
  console.log(`Deploying TestRecipient at "${testRecipientAddr.green}"...`);

  // deploy test recipient
  {
    const tx = await exec.deployContract({
      abi: TestRecipient__factory.abi,
      bytecode: TestRecipient__factory.bytecode,
      args: [],
    });
    logTx("Deploy test recipient", tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  // deploy multisig ism
  const multisigIsmAddr = await query.readContract({
    abi: StaticMessageIdMultisigIsmFactory__factory.abi,
    address: HYP_MULTSIG_ISM_FACTORY,
    functionName: "getAddress",
    args: [[account.address], 1],
  });
  console.log(`Deploying multisigIsm at "${multisigIsmAddr.green}"...`);

  {
    const tx = await exec.writeContract({
      abi: StaticMessageIdMultisigIsmFactory__factory.abi,
      address: HYP_MULTSIG_ISM_FACTORY,
      functionName: "deploy",
      args: [[account.address], 1],
    });
    logTx("Deploy multisig ism", tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  // set ism of test recipient

  console.log(`Setting ism of test recipient to "${multisigIsmAddr.green}"...`);
  {
    const tx = await exec.writeContract({
      abi: TestRecipient__factory.abi,
      address: testRecipientAddr,
      functionName: "setInterchainSecurityModule",
      args: [multisigIsmAddr],
    });
    logTx("Set multisig ism to test recipient", tx);
    await query.waitForTransactionReceipt({ hash: tx });
  }

  console.log("== Done! ==");

  console.log({
    testRecipient: testRecipientAddr,
    multisigIsm: multisigIsmAddr,
  });
}
