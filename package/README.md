# Mina Signer Wasm

## Installation
##### with npm

```bash
npm i @chainsafe/mina-signer-wasm
```
##### with yarn
```bash
yarn add @chainsafe/mina-signer-wasm
```

## Initialization
##### for NodeJs
```js
import { Client } from "@chainsafe/mina-signer-wasm";

const client = new Client({ network: "testnet" });
```

##### for Web
```js
import init, { Client } from "@chainsafe/mina-signer-wasm";

await init();
const client = new Client({ network: "testnet" });
```

## Examples
```js
// Generate keys
const keypair = client.genKeys();

// Sign and verify message
const signed = client.signMessage("hello", keypair);
if (client.verifyMessage(signed)) {
    console.log("Message was verified successfully");
}

// Sign and verify a payment
const signedPayment = client.signPayment(
    {
        to: keypair.publicKey,
        from: keypair.publicKey,
        amount: 1,
        fee: 1,
        nonce: 0,
    },
    keypair.privateKey
);
if (client.verifyPayment(signedPayment)) {
    console.log("Payment was verified successfully");
}

// Sign and verify a stake delegation
const signedDelegation = client.signStakeDelegation(
    {
        to: keypair.publicKey,
        from: keypair.publicKey,
        fee: "1",
        nonce: "0",
    },
    keypair.privateKey
);
if (client.verifyStakeDelegation(signedDelegation)) {
    console.log("Delegation was verified successfully");
}
```
