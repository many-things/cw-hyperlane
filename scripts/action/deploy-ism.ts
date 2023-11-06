
import { loadContext } from "../src/load_context";
import {
  config,
  getSigningClient,
} from "../src/config";

import { ContractFetcher } from "./fetch";
import { deploy_ism } from "../src/deploy";

async function main() {
  const client = await getSigningClient(config);
  let ctx = loadContext(config.network.id);
  const contracts = new ContractFetcher(ctx, client).getContracts();
  console.log('Deploying ISM')
  const ism = await deploy_ism(client, config.deploy.ism!, contracts)
  console.log(ism)
}


main().catch(console.error);
