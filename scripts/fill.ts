import {
  ExecuteResult,
  SigningCosmWasmClient,
} from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";

import { ARTIFACTS } from "./artifacts";

async function getSigningClient(): Promise<{
  client: SigningCosmWasmClient;
  address: string;
}> {
  const mnemonic = process.env["SIGNING_MNEMONIC"] as string;
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: "osmo",
  });
  const [{ address }] = await wallet.getAccounts();

  const client = await SigningCosmWasmClient.connectWithSigner(
    "https://rpc.osmotest5.osmosis.zone/",
    wallet,
    {
      gasPrice: GasPrice.fromString("0.025uosmo"),
    }
  );
  return { client, address };
}

async function main() {
  const { client, address: owner } = await getSigningClient();

  const {
    hpl_hub: { address: hpl_hub },
    hpl_igp_core: { address: hpl_igp_core },
    hpl_igp_gas_oracle: { address: hpl_igp_gas_oracle },
    hpl_ism_multisig: { address: hpl_ism_multisig },
    hpl_ism_routing: { address: hpl_ism_routing },
    hpl_mailbox: { address: hpl_mailbox },
    hpl_multicall: { address: hpl_multicall },
    hpl_validator_announce: { address: hpl_validator_announce },
  } = ARTIFACTS.contracts;

  let execRes: ExecuteResult;

  // =========================== hpl_hub
  {
    const originDomain = await client.queryContractSmart(hpl_hub, {
      origin_domain: {},
    });
    console.log("OriginDomain:", originDomain);
  }

  // =========================== hpl_igp_gas_oracle
  {
    execRes = await client.execute(
      owner,
      hpl_igp_gas_oracle,
      {
        set_remote_gas_data_configs: {
          configs: [
            {
              remote_domain: 26657,
              token_exchange_rate: "1300000000000000000",
              gas_price: "1600000000000",
            },
          ],
        },
      },
      "auto"
    );
    console.log("SetRemoteGasDataConfigs:", execRes.transactionHash);

    const gasOracleConfigQuery = { config: {} };
    const gasOracleConfig = await client.queryContractSmart(
      hpl_igp_gas_oracle,
      gasOracleConfigQuery
    );
    console.log("GasOracleConfig:", gasOracleConfig);

    const getERAndGPQuery = {
      get_exchange_rate_and_gas_price: { dest_domain: 26657 },
    };
    const getERAndGP = await client.queryContractSmart(
      hpl_igp_gas_oracle,
      getERAndGPQuery
    );
    console.log("GetERAndGP:", getERAndGP);
  }

  // =========================== hpl_igp_core

  {
    execRes = await client.execute(
      owner,
      hpl_igp_core,
      {
        set_gas_oracles: {
          configs: [
            {
              remote_domain: 26657,
              gas_oracle: hpl_igp_gas_oracle,
            },
          ],
        },
      },
      "auto"
    );
    console.log("SetGasOracles:", execRes.transactionHash);

    const quoteGasPaymentQuery = {
      quote_gas_payment: { dest_domain: 26657, gas_amount: "123456" },
    };
    const quoteGasPayment = await client.queryContractSmart(
      hpl_igp_core,
      quoteGasPaymentQuery
    );
    console.log("QuoteGasPayment:", quoteGasPayment);

    const getERAndGPQuery = {
      get_exchange_rate_and_gas_price: { dest_domain: 26657 },
    };
    const getERAndGP = await client.queryContractSmart(
      hpl_igp_core,
      getERAndGPQuery
    );
    console.log("GetERAndGP:", getERAndGP);
  }

  // =========================== hpl_ism_multisig
  {
    execRes = await client.execute(
      owner,
      hpl_ism_multisig,
      {
        enroll_validator: {
          set: {
            domain: 26657,
            validator: owner,
            validator_pubkey: "ArVogl2Oa9JxkUZyhnLz7OcvLCrgeNh7dkaGRrCRuZ3w",
          },
        },
      },
      "auto"
    );
    console.log("EnrollValidator:", execRes.transactionHash);

    execRes = await client.execute(
      owner,
      hpl_ism_multisig,
      {
        set_threshold: {
          set: {
            domain: 26657,
            threshold: 1,
          },
        },
      },
      "auto"
    );
    console.log("SetThreshold:", execRes.transactionHash);
  }

  // =========================== hpl_ism_routing
  {
    execRes = await client.execute(
      owner,
      hpl_ism_routing,
      {
        set: {
          ism: {
            domain: 26657,
            address: hpl_ism_multisig,
          },
        },
      },
      "auto"
    );
    console.log("Set:", execRes.transactionHash);
  }

  // =========================== hpl_mailbox
  {
    execRes = await client.execute(
      owner,
      hpl_mailbox,
      {
        set_default_ism: {
          ism: hpl_ism_routing,
        },
      },
      "auto"
    );
    console.log("SetDefaultIsm:", execRes.transactionHash);
  }
}

main().catch(console.error);
