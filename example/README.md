# Deploy Guide with Local CosmWasm Chain

> This guide will help you to setup Hyperlane betweeen Local CosmWasm Chain and Ethereum Sepolia Testnet.

## Prerequisites

- Sepolia Testnet account with enough balance
  - will use for deploying test contracts on Sepolia
  - and relaying / validating messages between localwasmd and Sepolia

## 0. Run Local CosmWasm Chain

```bash
$ docker run --rm -it \
    -e PASSWORD=xxxxxxxx -e STAKE_TOKEN="uwasm" -e CHAIN_ID="localwasmd" -e MONIKER="localwasmd" \
    -p 26657:26657 -p 26656:26656 -p 9090:9090 -p 1317:1317 \
    cosmwasm/wasmd:latest /opt/setup_and_run.sh wasm12smx2wdlyttvyzvzg54y2vnqwq2qjate7wuwgt
```

## 1. Create `config.yaml` with your network config

> Don't forget to setup deploy settings below

Below is an example of `config.yaml` file for localwasmd.

You can check full list of example in [config.example.yaml](../config.example.yaml) file.

```yaml
networks:
  - id: "localwasmd"
    hrp: "uwasm"
    endpoint:
      rpc: "http://localhost:26657"
      rest: "http://localhost:1317"
      grpc: "http://localhost:9090"
    gas:
      price: "0.025"
      denom: "uwasm"
    # localwasmd -> ascii / decimal -> sum.
    # It's very arbitrary value, Perhaps you must need to change this value.
    domain: 1063

# default mnemonic key of localwasmd (https://github.com/osmosis-labs/osmosis/blob/d45a3baf684e55cdc83ef23c4fc11ae1df1726af/tests/localwasmd/scripts/setup.sh#L9C11-L9C159)
# osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj
# 0xae7d1F30e324D4e348EF04D9a9e867F863f23067
# 9ff80c31b47c7f2946654f569a6b1530db78d7fa5b3ea16db82570cdfd6d43f6
signer: "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort"

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
# - context with artifacts (default path: {project-root}/context/localwasmd.json)
$ yarn cw-hpl upload local -n localwasmd
```

### Remote

```bash
$ yarn install

# check all versions of contract codes from Github
$ yarn cw-hpl upload remote-list -n localwasmd

# This command will make one file.
# - context with artifacts (default path: {project-root}/context/localwasmd.json)
$ yarn cw-hpl upload remote v0.0.6-rc8 -n localwasmd
```

## 3. Instantiate Contracts

If you configured / uploaded contract codes correctly, you can deploy contract with one simple command.

```bash
# This command will output two results.
# - context + deployment    (default path: ./context/localwasmd.json)
# - Hyperlane agent-config  (default path: ./context/localwasmd.config.json)
$ yarn cw-hpl deploy -n localwasmd
```

## 4. Setup Validator / Relayer config

Replace every `{private_key}` from files below with your Sepolia Testnet private key.

- [./hyperlane/relayer.json](./hyperlane/relayer.json)
- [./hyperlane/validator.sepolia.json](./hyperlane/validator.sepolia.json)

And run with below command.

```bash
# Merge localwasmd.config.json and agent-config.docker.json
$ LOCALWASMD_AGENT_CONFIG=$(cat ../context/localwasmd.config.json) && \
  LOCALWASMD_AGENT_CONFIG_NAME=$(echo $LOCALWASMD_AGENT_CONFIG | jq -r '.name') && \
    cat ./hyperlane/agent-config.docker.json \
      | jq ".chains.$LOCALWASMD_AGENT_CONFIG_NAME=$(echo $LOCALWASMD_AGENT_CONFIG)" > merge.tmp && \
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

# 2. Deploy MultisigIsm for validating localwasmd network
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

### Sepolia -> localwasmd

```bash
# Below address is mailbox came from agent-config.docker.json
$ cast send \
    0xfFAEF09B3cd11D9b20d1a19bECca54EEC2884766 --value 1wei \
    'dispatch(uint32,bytes32,bytes)' \
    1063 $LOCALWASMD_TEST_RECIPIENT_ADDRESS 0x68656c6c6f \ # 0x68656c6c6f -> 'hello'
    --rpc-url 'https://rpc.sepolia.org' \
    --private-key $SEPOLIA_PRIVATE_KEY
```

### localwasmd -> Sepolia

```bash
# [dest-domain] [recipient-address] [message]
$ yarn cw-hpl contract test-dispatch -n localwasmd 11155111 $SEPOLIA_TEST_RECIPIENT_ADDRESS hello
```
