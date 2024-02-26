# cw-hpl

## Prerequisites

- \>= Node v20
- \>= Yarn v4.1

## Configuration

Create a `config.yaml` file in the root directory of the project.

You can check the [config.example.yaml](../config.example.yaml) file to see all supported options. And also you can check the full list of options in the [config.ts](./src/config.ts) file.

After setup your config file, you can use cw-hpl command-line toolkit from now.

## Usage

### Contract

```bash
# List all supported contracts
$ yarn cw-hpl contract list

# Test dispatch to a mailbox (needs to run 'cw-hpl deploy' first)
$ yarn cw-hpl contract test-dispatch
```

### Deploy

```bash
# Deploy all contracts based on setting in config file
$ yarn cw-hpl deploy
```

### Upload

```bash
# Upload contract codes that from local environment
$ yarn cw-hpl upload local

# Fetch & Upload contract codes from Github
$ yarn cw-hpl upload remote

# List all versions of contract codes from Github
$ yarn cw-hpl upload remote-list
```

## Maintaining

### Adding a new contract

- ism

  1. Append [contractNames](./shared/constants.ts) with new contract name
  2. Add new ISM type to [ISMType](./shared/config.ts) and [ContextIsm](./shared/context.ts)
  3. Write deploy script for new ISM in [ism.ts](./deploy/ism.ts)
  4. Done!

- hook

  1. Append [contractNames](./shared/constants.ts) with new contract name
  2. Add new Hook type to [HookType](./shared/config.ts) and [ContextHook](./shared/context.ts)
  3. Write deploy script for new Hook in [hook.ts](./deploy/hook.ts)
  4. Done!

- others

  1. Append [contractNames](./shared/constants.ts) with new contract name
  2. Add new config type to [Config](./shared/config.ts) if it needs to be configured.
  3. Add new contract field to [ContextDeployment](./shared/context.ts)
  4. Write deploy script for new contract in [deploy.ts](./commands/deploy.ts)
  5. Done!
