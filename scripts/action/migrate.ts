import "reflect-metadata";

import { Event } from "@cosmjs/cosmwasm-stargate";
import { config, getSigningClient } from "../src/config";
import { loadContext } from "../src/load_context";
import { ContractFetcher } from "./fetch";

const parseEventLog = (events: readonly Event[]) => {
  return events.map((v) => ({
    "@type": v.type.slice(5),
    ...Object.fromEntries(v.attributes.map((x) => [x.key, x.value])),
  }));
};

async function main() {
  const client = await getSigningClient(config);

  const ctx = loadContext(config.network.id);

  const contracts = new ContractFetcher(ctx, client).getContracts();

  const migrations: [string, number][] = [
    [
      "neutron1q75ky8reksqzh0lkhk9k3csvjwv74jjquahrj233xc7dvzz5fv4qtvw0qg",
      contracts.isms.multisig.codeId!,
    ],
    [
      "neutron12p8wntzra3vpfcqv05scdx5sa3ftaj6gjcmtm7ynkl0e6crtt4ns8cnrmx",
      contracts.igp.core.codeId!,
    ],
    [
      "neutron17w4q6efzym3p4c6umyp4cjf2ustjtmwfqdhd7rt2fpcpk9fmjzsq0kj0f8",
      contracts.core.va.codeId!,
    ],
  ];

  for (const [addr, code_id] of migrations) {
    const contract_info = await client.wasm.getContract(addr);

    if (!contract_info.admin) {
      console.log(`skipping ${addr} as it has no admin`);
      continue;
    }

    if (contract_info.admin !== client.signer) {
      console.log(
        `skipping ${addr} as it is not admin. actual: ${contract_info.admin}`
      );
      continue;
    }

    const migrate_resp = await client.wasm.migrate(
      client.signer,
      addr,
      code_id,
      {},
      "auto"
    );
    console.log(parseEventLog(migrate_resp.events));
  }

  const set_gas_resp = await client.wasm.execute(
    client.signer,
    "neutron12p8wntzra3vpfcqv05scdx5sa3ftaj6gjcmtm7ynkl0e6crtt4ns8cnrmx",
    {
      set_default_gas: {
        gas: "200000",
      },
    },
    "auto"
  );
  console.log(parseEventLog(set_gas_resp.events));
}

main().catch(console.error);
