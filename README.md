# mina-signer-wasm

[![CI](https://github.com/hanabi1224/mina-signer-wasm/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/hanabi1224/mina-signer-wasm/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/hanabi1224/mina-signer-wasm/status.svg?style=flat-square)](https://deps.rs/repo/github/hanabi1224/mina-signer-wasm)

This library is a drop-in replacement of [mina-signer](https://www.npmjs.com/package/mina-signer/v/1.2.0), APIs are compatable with [mina-signer 1.2.0](https://www.npmjs.com/package/mina-signer/v/1.2.0)

## Integration tests and benchmarks
```bash
cd examples/integration_tests
yarn
yarn test
```

## API list

- [x] genKeys
- [x] verifyKeypair
- [x] derivePublicKey
- [x] signMessage
- [x] verifyMessage
- [x] signPayment
- [x] verifyPayment
- [x] signStakeDelegation
- [x] verifyStakeDelegation
- [ ] hashPayment
- [ ] hashStakeDelegation
- [ ] signedRosettaTransactionToSignedCommand
- [x] publicKeyToRaw

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
```
