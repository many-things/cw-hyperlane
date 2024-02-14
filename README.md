# CW Hyperlane

[![codecov](https://codecov.io/gh/many-things/cw-hyperlane/branch/main/graph/badge.svg?token=SGYE7FBTAO)](https://codecov.io/gh/many-things/cw-hyperlane)
[![crates.io](https://img.shields.io/crates/v/hpl-interface)](https://crates.io/crates/hpl-interface)

## Architecture Overview

![Architecture Overview](./asset/hyperlane-all.png)

### Components

```
├── contracts
│   │
│   ├── core
│   │   ├── mailbox
│   │   └── va
│   │
│   ├── hooks
│   │   ├── aggregate
│   │   ├── fee
│   │   ├── merkle
│   │   ├── pausable
│   │   ├── routing
│   │   ├── routing-custom
│   │   └── routing-fallback
│   │
│   ├── igps
│   │   ├── core
│   │   └── oracle
│   │
│   ├── isms
│   │   ├── aggregate
│   │   ├── multisig
│   │   ├── pausable
│   │   └── routing
│   │
│   ├── mocks
│   │   ├── mock-hook
│   │   ├── mock-ism
│   │   └── mock-msg-receiver
│   │
│   └── warp
│       ├── cw20
│       └── native
│
├── integration-test
│
├── packages
│   │
│   ├── connection
│   ├── interface
│   ├── ownable
│   ├── pausable
│   ├── router
│   └── schema
│
├── scripts
│
└── ts
    └── sdk
```

## Prerequisites

- rust (wasm32-wasm32-unknown target)
- go 1.20 or higher
- llvm-cov

## How to build

```bash
make install-dev

make build
```

## How to test

```bash
cargo test --workspace --exclude hpl-tests

cargo llvm-cov --workspace --exclude hpl-tests
```
