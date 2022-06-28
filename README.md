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

- [node (16)](https://nodejs.org/en/download/)
- [yarn (latest)](https://yarnpkg.com/)
- [rust (stable)](https://rustup.rs/)
- [wasm-pack (latest)](https://rustwasm.github.io/wasm-pack/)

## Build

to generate server side npm package (refer to examples/integration_tests)
```bash
wasm-pack build -t nodejs -d pkg-node
```

to generate browser side npm package (refer to examples/web), the static site is also deployed at https://mina-signer-wasm.vercel.app/
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
integration_tests: [js]   genKeys x 15.85 ops/sec ±0.56% (43 runs sampled)
integration_tests: [wasm] genKeys x 1,703 ops/sec ±0.68% (96 runs sampled)
integration_tests: [js]   verifyKeypair x 2.18 ops/sec ±0.18% (10 runs sampled)
integration_tests: [wasm] verifyKeypair x 1,602 ops/sec ±0.08% (98 runs sampled)
integration_tests: [js]   derivePublicKey x 15.93 ops/sec ±0.20% (43 runs sampled)
integration_tests: [wasm] derivePublicKey x 1,840 ops/sec ±0.11% (97 runs sampled)
integration_tests: [js]   publicKeyToRaw x 45.59 ops/sec ±0.51% (60 runs sampled)
integration_tests: [wasm] publicKeyToRaw x 10,642 ops/sec ±0.17% (98 runs sampled)
integration_tests: [js]   signMessage x 4.56 ops/sec ±0.15% (16 runs sampled)
integration_tests: [wasm] signMessage x 616 ops/sec ±0.15% (96 runs sampled)
integration_tests: [js]   verifyMessage x 5.98 ops/sec ±0.35% (19 runs sampled)
integration_tests: [wasm] verifyMessage x 474 ops/sec ±0.04% (95 runs sampled)
integration_tests: [js]   signPayment x 3.36 ops/sec ±0.22% (13 runs sampled)
integration_tests: [wasm] signPayment x 304 ops/sec ±0.07% (90 runs sampled)
integration_tests: [js]   verifyPayment x 4.17 ops/sec ±0.27% (15 runs sampled)
integration_tests: [wasm] verifyPayment x 317 ops/sec ±0.08% (93 runs sampled)
integration_tests: [js]   signStateDelegation x 3.37 ops/sec ±0.21% (13 runs sampled)
integration_tests: [wasm] signStateDelegation x 305 ops/sec ±0.02% (90 runs sampled)
integration_tests: [js]   verifyStateDelegation x 4.20 ops/sec ±0.19% (15 runs sampled)
integration_tests: [wasm] verifyStateDelegation x 318 ops/sec ±0.07% (89 runs sampled)
integration_tests: [js]   hashPayment x 21.57 ops/sec ±0.27% (40 runs sampled)
integration_tests: [wasm] hashPayment x 2,671 ops/sec ±0.10% (99 runs sampled)
integration_tests: [js]   hashStakeDelegation x 21.60 ops/sec ±0.25% (40 runs sampled)
integration_tests: [wasm] hashStakeDelegation x 2,734 ops/sec ±0.11% (96 runs sampled)
integration_tests: [js]   signedRosettaTransactionToSignedCommand - Payment x 84.25 ops/sec ±0.55% (72 runs sampled)
integration_tests: [wasm] signedRosettaTransactionToSignedCommand - Payment x 3,931 ops/sec ±0.58% (97 runs sampled)
integration_tests: [js]   signedRosettaTransactionToSignedCommand - StakeDelegation x 80.75 ops/sec ±0.38% (69 runs sampled)
integration_tests: [wasm] signedRosettaTransactionToSignedCommand - StakeDelegation x 3,946 ops/sec ±0.48% (98 runs sampled)
```
