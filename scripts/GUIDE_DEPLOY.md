# Deploy Guide with LocalOsmosis

> This guide will help you to setup Hyperlane betweeen LocalOsmosis and Ethereum Sepolia Testnet.

## 0. Run LocalOsmosis

```bash
# Move to your working directory
$ cd {working_directory}

# Clone osmosis repository & cd to it
$ git clone https://github.com/osmosis-labs/osmosis.git && cd osmosis

# Run localnet in background
$ make localnet-startd

# Stop / Clean localnet
$ make localnet-stop
$ make localnet-clean
```

## 1. Create `config.yaml` with your network config

> Don't forget to setup deploy settings below

Below is an example of `config.yaml` file for localosmosis.

You can check full list of example in [config.example.yaml](../config.example.yaml) file.

```yaml
networks:
  - id: localosmosis
    hrp: osmo
    endpoint:
      rpc: http://localhost:26657
      rest: http://localhost:1317
      grpc: http://localhost:9090
    gas:
      price: 0.025
      denom: uosmo
    # localosmosis -> ascii / decimal -> sum.
    # It's very arbitrary value, Perhaps you must need to change this value.
    domain: 1304

signer: <private_key> | <mnemonic>

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

You can upload contract codes from local environment or from [Github](https://github.com/many-things/cw-hyperlane/releases/).

### Local

```bash
# Build contracts from local environment
$ make optimize
# Run compatibility test
$ make check

# This command will make one file.
# - context with artifacts (default path: ./context/localosmosis.json)
$ yarn cw-hpl upload local -n localosmosis
```

### Remote

```bash
# check all versions of contract codes from Github
$ yarn cw-hpl upload remote-list -n localosmosis

# This command will make one file.
# - context with artifacts (default path: ./context/localosmosis.json)
$ yarn cw-hpl upload remote v0.0.6-rc8 -n localosmosis
```

## 3. Instantiate Contracts

If you configured / uploaded contract codes correctly, you can deploy contract with one simple command.

```bash
# This command will output two results.
# - context + deployment    (default path: ./context/localosmosis.json)
# - Hyperlane agent-config  (default path: ./context/localosmosis.config.json)
$ yarn cw-hpl deploy -n localosmosis
```

## 4. Setup Validator / Relayer config
