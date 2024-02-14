# Deploy Scripts

## Prerequisites

- [pnpm](https://pnpm.io/)

## Configuration

Create a `config.yaml` file in the root directory of the project. Default option for Osmosis testnet is following.

Also, you can check the full list of options in the [config.ts](./src/config.ts) file.

```yaml
network:
  id: "osmo-test-5"
  hrp: "osmo"
  url: "https://rpc.osmotest5.osmosis.zone/"
  gas:
    price: "0.025"
    denom: "uosmo"
  domain: 1037 # osmo-test-5 -> ascii / decimal -> sum

signer: { PRIVATE_KEY }

deploy:
  ism:
    type: multisig
    owner: { SIGNER_ADDRESS }
    validators:
      5:
        addrs:
          - { SIGNER_ETH_ADDRESS }
        threshold: 1
      420:
        addrs:
          - { SIGNER_ETH_ADDRESS }
        threshold: 1
      421613:
        addrs:
          - { SIGNER_ETH_ADDRESS }
        threshold: 1

  hooks:
    default:
      type: mock

    required:
      type: aggregate
      owner: { SIGNER_ADDRESS }
      hooks:
        - type: merkle

        - type: pausable
          owner: { SIGNER_ADDRESS }
          paused: false
        - type: fee
          owner: { SIGNER_ADDRESS }
          fee:
            denom: uosmo
            amount: 1
```

## Usage

### Uploading Contract Codes

```bash
pnpm upload
```

### Deploying Contracts

```bash
pnpm deploy
```

## Maintaining

### Adding a new contract

1. Add a new module with actual contract output name in the [contracts](./src/contracts/) directory.
2. Class name should be upper camel case conversion of the contract name.
3. Import new module [contracts/index.ts](./src/index.ts) file.
4. If a new contract is ISM or Hook, add a new option to config type.
5. Add a new field to the Contracts class in the [deploy.ts](./src/deploy.ts) file.
