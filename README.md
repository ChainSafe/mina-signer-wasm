# mina-signer-wasm

[![CI](https://github.com/hanabi1224/mina-signer-wasm/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/hanabi1224/mina-signer-wasm/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/hanabi1224/mina-signer-wasm/status.svg?style=flat-square)](https://deps.rs/repo/github/hanabi1224/mina-signer-wasm)

This library is a drop-in replacement of [mina-signer](https://www.npmjs.com/package/mina-signer/v/1.1.0), APIs are compatable with [mina-signer v1.1.0](https://www.npmjs.com/package/mina-signer/v/1.1.0)

## API list

- [x] `genKeys`
- [x] `verifyKeypair`
- [x] `derivePublicKey`
- [x] `signMessage`
- [x] `verifyMessage`
- [x] `signPayment`
- [x] `verifyPayment`
- [x] `signStakeDelegation`
- [x] `verifyStakeDelegation`
- [x] `hashPayment`
- [x] `hashStakeDelegation`
- [x] `signedRosettaTransactionToSignedCommand`
- [x] `publicKeyToRaw`
- [x] `publicKeyToRawBeta` (this API is compatable with `publicKeyToRaw` in [mina-signer v1.2.0](https://www.npmjs.com/package/mina-signer/v/1.2.0))

## Unpacked size
< 0.5MB, while `mina-signer` v1.1.0 is > 3MB and `mina-signer` v1.2.0 is > 40MB

## Prerequisites

- [node (lts)](https://nodejs.org/en/download/)
- [yarn (v1)](https://classic.yarnpkg.com/)
- [rust (stable)](https://rustup.rs/)
- [wasm-pack (latest)](https://rustwasm.github.io/wasm-pack/)

## Build

to generate server side npm package
```bash
wasm-pack build -t nodejs -d pkg-node
```

to generate browser side npm package
```bash
wasm-pack build -t web -d pkg-web
```

## Usage

```
// const Client = require("mina-signer");
// is equivalent to 
const Client = require("./pkg-node/mina_signer_wasm").Client
```

## Integration tests and benchmarks
```bash
cd examples/integration_tests
yarn
yarn test
yarn benchmark
```

## Benchmark result
```
    [js]   genKeys x 575 ops/sec ±5.12% (73 runs sampled)
    [wasm] genKeys x 2,682 ops/sec ±3.32% (87 runs sampled)

    [js]   verifyKeypair x 31.69 ops/sec ±4.66% (55 runs sampled)
    [wasm] verifyKeypair x 2,785 ops/sec ±2.25% (87 runs sampled)

    [js]   derivePublicKey x 877 ops/sec ±3.20% (83 runs sampled)
    [wasm] derivePublicKey x 3,209 ops/sec ±2.01% (86 runs sampled)

    [js]   publicKeyToRaw x 1,564 ops/sec ±2.38% (87 runs sampled)
    [wasm] publicKeyToRaw x 206,306 ops/sec ±3.22% (78 runs sampled)

    [js]   signMessage x 102 ops/sec ±1.92% (75 runs sampled)
    [wasm] signMessage x 172 ops/sec ±1.92% (79 runs sampled)

    [js]   verifyMessage x 142 ops/sec ±3.63% (73 runs sampled)
    [wasm] verifyMessage x 161 ops/sec ±2.52% (82 runs sampled)

    [js]   signPayment x 54.27 ops/sec ±2.53% (64 runs sampled)
    [wasm] signPayment x 150 ops/sec ±2.06% (85 runs sampled)

    [js]   verifyPayment x 66.60 ops/sec ±2.62% (69 runs sampled)
    [wasm] verifyPayment x 149 ops/sec ±2.79% (84 runs sampled)

    [js]   signStateDelegation x 52.87 ops/sec ±4.26% (64 runs sampled)
    [wasm] signStateDelegation x 155 ops/sec ±0.87% (78 runs sampled)

    [js]   verifyStateDelegation x 64.04 ops/sec ±3.90% (66 runs sampled)
    [wasm] verifyStateDelegation x 146 ops/sec ±2.79% (80 runs sampled)

    [js]   hashPayment x 4.78 ops/sec ±1.84% (16 runs sampled)
    [wasm] hashPayment x 4,295 ops/sec ±3.16% (87 runs sampled)

    [js]   hashStakeDelegation x 4.82 ops/sec ±2.99% (16 runs sampled)
    [wasm] hashStakeDelegation x 4,546 ops/sec ±1.24% (88 runs sampled)

    [js]   signedRosettaTransactionToSignedCommand - Payment x 139 ops/sec ±1.85% (80 runs sampled)
    [wasm] signedRosettaTransactionToSignedCommand - Payment x 6,485 ops/sec ±0.82% (94 runs sampled)

    [js]   signedRosettaTransactionToSignedCommand - StakeDelegation x 135 ops/sec ±1.89% (76 runs sampled)
    [wasm] signedRosettaTransactionToSignedCommand - StakeDelegation x 6,135 ops/sec ±2.62% (89 runs sampled)
```
