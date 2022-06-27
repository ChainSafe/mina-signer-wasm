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

## Web examples
```bash
cd examples/web
yarn
yarn build
yarn dev
```

## Benchmark result from CI job
```
integration_tests: [js]   genKeys x 14.93 ops/sec ±0.66% (41 runs sampled)
integration_tests: [wasm] genKeys x 1,705 ops/sec ±0.79% (95 runs sampled)
integration_tests: [js]   verifyKeypair x 2.04 ops/sec ±0.55% (10 runs sampled)
integration_tests: [wasm] verifyKeypair x 1,608 ops/sec ±0.08% (98 runs sampled)
integration_tests: [js]   derivePublicKey x 15.10 ops/sec ±0.19% (41 runs sampled)
integration_tests: [wasm] derivePublicKey x 1,867 ops/sec ±0.07% (98 runs sampled)
integration_tests: [js]   publicKeyToRaw x 41.02 ops/sec ±0.44% (54 runs sampled)
integration_tests: [wasm] publicKeyToRaw x 10,104 ops/sec ±0.08% (98 runs sampled)
integration_tests: [js]   signMessage x 4.36 ops/sec ±0.17% (15 runs sampled)
integration_tests: [wasm] signMessage x 636 ops/sec ±0.07% (96 runs sampled)
integration_tests: [js]   verifyMessage x 5.69 ops/sec ±0.38% (19 runs sampled)
integration_tests: [wasm] verifyMessage x 484 ops/sec ±0.13% (93 runs sampled)
integration_tests: [js]   signPayment x 3.17 ops/sec ±0.38% (12 runs sampled)
integration_tests: [wasm] signPayment x 303 ops/sec ±0.09% (90 runs sampled)
integration_tests: [js]   verifyPayment x 3.97 ops/sec ±0.42% (14 runs sampled)
integration_tests: [wasm] verifyPayment x 320 ops/sec ±0.04% (90 runs sampled)
integration_tests: [js]   signStateDelegation x 3.18 ops/sec ±0.29% (12 runs sampled)
integration_tests: [wasm] signStateDelegation x 310 ops/sec ±0.07% (91 runs sampled)
integration_tests: [js]   verifyStateDelegation x 3.89 ops/sec ±0.29% (14 runs sampled)
integration_tests: [wasm] verifyStateDelegation x 313 ops/sec ±0.07% (93 runs sampled)
integration_tests: [js]   hashPayment x 20.43 ops/sec ±0.35% (38 runs sampled)
integration_tests: [wasm] hashPayment x 2,631 ops/sec ±0.10% (99 runs sampled)
integration_tests: [js]   hashStakeDelegation x 20.58 ops/sec ±0.23% (38 runs sampled)
integration_tests: [wasm] hashStakeDelegation x 2,694 ops/sec ±0.12% (97 runs sampled)
integration_tests: [js]   signedRosettaTransactionToSignedCommand - Payment x 79.29 ops/sec ±0.54% (69 runs sampled)
integration_tests: [wasm] signedRosettaTransactionToSignedCommand - Payment x 3,967 ops/sec ±0.62% (96 runs sampled)
integration_tests: [js]   signedRosettaTransactionToSignedCommand - StakeDelegation x 75.74 ops/sec ±0.40% (77 runs sampled)
integration_tests: [wasm] signedRosettaTransactionToSignedCommand - StakeDelegation x 3,958 ops/sec ±0.49% (97 runs sampled)
```
