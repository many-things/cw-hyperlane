# Deploy Guide with Osmosis Testnet

> This guide will help you to setup Hyperlane betweeen Osmosis Testnet and Ethereum Sepolia Testnet.

## Prerequisites

- [Cast](https://book.getfoundry.sh/cast/)

- Sepolia Testnet account with enough balance

  - [Visit faucet](https://sepolia-faucet.pk910.de/) to get some test tokens

- Osmosis Testnet account with enough balance

  - [Visit faucet](https://faucet.testnet.osmosis.zone/) to get some test tokens

- Recommanded to use same account for both networks

  - You can easily get the bech32 address by running below command (need to setup `config.yaml` first)

    - Get from private key

      `yarn cw-hpl wallet address -n [network-id] --private-key [private-key]`

    - Get from mnemonic phrase

      `yarn cw-hpl wallet address -n [network-id] --mnemonic [mnemonic]`

  - You also can get the ethereum address by running below command

    - Get from private key

      `cast wallet address --private-key [private-key]`

    - Get from mnemonic phrase

      `cast wallet address --mnemonic [mnemonic]`

  - Or, You can use `yarn cw-hpl wallet new -n [network-id]` to create new wallet

## 1. Create `config.yaml` with your network config

> Don't forget to setup deploy settings below

Below is an example of `config.yaml` file for localwasmd.

You can check full list of example in [config.example.yaml](../config.example.yaml) file.

```yaml
networks:
  - id: "osmo-test-5"
    hrp: "osmo"
    endpoint:
      rpc: "https://rpc.testnet.osmosis.zone"
      rest: "https://lcd.testnet.osmosis.zone"
      grpc: "https://grpc.testnet.osmosis.zone"
    gas:
      price: "0.025"
      denom: "uosmo"
    # osmo-test-5 -> ascii / decimal -> sum.
    # It's very arbitrary value, Perhaps you must need to change this value.
    domain: 1037 # osmo-test-5 -> ascii / decimal -> sum

signer: "<private-key> or <mnemonic>"

deploy:
  ism:
    - 11155111

  hooks:
    default:
      type: mock

    required:
      type: aggregate
      # if you keep it as "<signer>", the script will identify this as deployer address
      owner: <signer>
      hooks:
        - type: merkle

        - type: pausable
          owner: <signer>
          paused: false

        - type: fee
          owner: <signer>
          fee:
            # if you didn't set the denom, it will be set as gas denom of network config
            denom: uosmo
            amount: 1

        - type: igp
          owner: <signer>
          configs:
            11155111:
              exchange_rate: 3000
              gas_price: 5000
          default_gas_usage: 30000
```

## 2. Upload Contract Codes

You can upload contract codes from local environment or from [Github](https://github.com/many-things/cw-hyperlane/releases).

### Local

```bash
$ yarn install

# Build contracts from local environment
$ make optimize
# Run compatibility test
$ make check

# This command will make one file.
# - context with artifacts (default path: {project-root}/context/osmo-test-5.json)
$ yarn cw-hpl upload local -n osmo-test-5
```

### Remote

```bash
$ yarn install

# check all versions of contract codes from Github
$ yarn cw-hpl upload remote-list -n osmo-test-5

# This command will make one file.
# - context with artifacts (default path: {project-root}/context/osmo-test-5.json)
$ yarn cw-hpl upload remote v0.0.6-rc8 -n osmo-test-5
```

## 3. Instantiate Contracts

If you configured / uploaded contract codes correctly, you can deploy contract with one simple command.

```bash
# This command will output two results.
# - context + deployment    (default path: ./context/osmo-test-5.json)
# - Hyperlane agent-config  (default path: ./context/osmo-test-5.config.json)
$ yarn cw-hpl deploy -n osmo-test-5
```

## 4. Setup Validator / Relayer config

Replace every `{private_key}` from files below with your Sepolia Testnet private key.

- [./hyperlane/relayer.json](./hyperlane/relayer.json)
- [./hyperlane/validator.sepolia.json](./hyperlane/validator.sepolia.json)

And run with below command.

```bash
# Merge osmo-test-5.config.json and agent-config.docker.json
$ OSMOSIS_TESTNET_AGENT_CONFIG=$(cat ../context/osmo-test-5.config.json) && \
  OSMOSIS_TESTNET_AGENT_CONFIG_NAME=$(echo $OSMOSIS_TESTNET_AGENT_CONFIG | jq -r '.name') && \
    cat ./hyperlane/agent-config.docker.json \
      | jq ".chains.$OSMOSIS_TESTNET_AGENT_CONFIG_NAME=$(echo $OSMOSIS_TESTNET_AGENT_CONFIG)" > merge.tmp && \
  mv merge.tmp ./hyperlane/agent-config.docker.json

# Run Hyperlane with docker-compose
$ docker compose up

# Run this if you want to run in background
$ docker compose up -d

# Run this if you want to see logs
$ docker compose logs -f

# Run this if you want to stop
$ docker compose down
```

## 5. Deploy Test contracts on Sepolia

```bash
# 1. Deploy TestRecipient contract
$ cast send \
    --rpc-url https://rpc.sepolia.org \
    --private-key $SEPOLIA_PRIVATE_KEY \
    --create $(cat ./TestRecipient.bin)

# 2. Deploy MultisigIsm for validating osmo-test-5 network
# Below address is messageIdMultisigIsmFactory came from agent-config.docker.json
$ cast send \
    0xFEb9585b2f948c1eD74034205a7439261a9d27DD \
    'deploy(address[],uint8)(address)' \
    [$(cast wallet address --private-key $SEPOLIA_PRIVATE_KEY)] 1 \ # 1 validator and 1/1 threshold
    --rpc-url https://rpc.sepolia.org \
    --private-key $SEPOLIA_PRIVATE_KEY

# 3. Get deployed multisig ism address from receipt of above command
$ cast send \
    $SEPOLIA_TEST_RECIPIENT_ADDRESS \ # output of step 1
    'setInterchainSecurityModule(address)' \
    $SEPOLIA_MULTISIG_ISM_ADDRESS \ # output of step 2
    --rpc-url https://rpc.sepolia.org \
    --private-key $SEPOLIA_PRIVATE_KEY
```

## 6. Run Messaging Test

### Sepolia -> osmo-test-5

```bash
# Below address is mailbox came from agent-config.docker.json
$ cast send \
    0xfFAEF09B3cd11D9b20d1a19bECca54EEC2884766 --value 1wei \
    'dispatch(uint32,bytes32,bytes)' \
    1037 $OSMOSIS_TESTNET_TEST_RECIPIENT_ADDRESS 0x68656c6c6f \ # 0x68656c6c6f -> 'hello'
    --rpc-url 'https://rpc.sepolia.org' \
    --private-key $SEPOLIA_PRIVATE_KEY
```

### osmo-test-5 -> Sepolia

```bash
# [dest-domain] [recipient-address] [message]
$ yarn cw-hpl contract test-dispatch -n osmo-test-5 11155111 $SEPOLIA_TEST_RECIPIENT_ADDRESS hello
```
