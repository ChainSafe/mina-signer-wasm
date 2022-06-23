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

## Benchmark result from CI job
```
integration_tests: [js]   genKeys x 11.77 ops/sec ±1.43% (33 runs sampled)
integration_tests: [wasm] genKeys x 1,361 ops/sec ±1.01% (86 runs sampled)
integration_tests: [js]   verifyKeypair x 1.65 ops/sec ±2.33% (9 runs sampled)
integration_tests: [wasm] verifyKeypair x 1,336 ops/sec ±1.39% (86 runs sampled)
integration_tests: [js]   derivePublicKey x 12.51 ops/sec ±1.34% (35 runs sampled)
integration_tests: [wasm] derivePublicKey x 1,595 ops/sec ±0.98% (89 runs sampled)
integration_tests: [js]   publicKeyToRaw x 39.21 ops/sec ±1.24% (51 runs sampled)
integration_tests: [wasm] publicKeyToRaw x 9,990 ops/sec ±0.88% (90 runs sampled)
integration_tests: [js]   signMessage x 3.54 ops/sec ±2.20% (13 runs sampled)
integration_tests: [wasm] signMessage x 84.88 ops/sec ±0.75% (71 runs sampled)
integration_tests: [js]   verifyMessage x 4.64 ops/sec ±1.81% (16 runs sampled)
integration_tests: [wasm] verifyMessage x 85.21 ops/sec ±1.08% (72 runs sampled)
integration_tests: [js]   signPayment x 2.61 ops/sec ±0.88% (11 runs sampled)
integration_tests: [wasm] signPayment x 75.53 ops/sec ±1.16% (65 runs sampled)
integration_tests: [js]   verifyPayment x 3.35 ops/sec ±1.96% (13 runs sampled)
integration_tests: [wasm] verifyPayment x 77.71 ops/sec ±0.79% (66 runs sampled)
integration_tests: [js]   signStateDelegation x 2.68 ops/sec ±2.18% (11 runs sampled)
integration_tests: [wasm] signStateDelegation x 74.94 ops/sec ±1.29% (66 runs sampled)
integration_tests: [js]   verifyStateDelegation x 3.24 ops/sec ±1.79% (13 runs sampled)
integration_tests: [wasm] verifyStateDelegation x 76.39 ops/sec ±0.97% (77 runs sampled)
integration_tests: [js]   hashPayment x 19.74 ops/sec ±0.98% (36 runs sampled)
integration_tests: [wasm] hashPayment x 2,637 ops/sec ±0.79% (91 runs sampled)
integration_tests: [js]   hashStakeDelegation x 19.54 ops/sec ±1.33% (36 runs sampled)
integration_tests: [wasm] hashStakeDelegation x 2,688 ops/sec ±0.93% (91 runs sampled)
integration_tests: [js]   signedRosettaTransactionToSignedCommand - Payment x 67.08 ops/sec ±1.38% (68 runs sampled)
integration_tests: [wasm] signedRosettaTransactionToSignedCommand - Payment x 3,359 ops/sec ±0.93% (91 runs sampled)
integration_tests: [js]   signedRosettaTransactionToSignedCommand - StakeDelegation x 65.48 ops/sec ±0.97% (67 runs sampled)
integration_tests: [wasm] signedRosettaTransactionToSignedCommand - StakeDelegation x 3,348 ops/sec ±1.15% (88 runs sampled)
```
