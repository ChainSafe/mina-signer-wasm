const ClientJs = require("mina-signer");
const wasm = require("./pkg/mina_signer_wasm")
const ClientWasm = wasm.Client

const clientJs = new ClientJs({ network: "mainnet" });
const clientWasm = new ClientWasm({ network: "mainnet" });

const Benchmark = require('benchmark');

test("genKeys", () => {
    const keypair = clientWasm.genKeys();
    expect(clientWasm.verifyKeypair(keypair))
    expect(clientJs.verifyKeypair(keypair))
});

test("verifyKeypair", () => {
    const keypair = clientJs.genKeys();
    expect(clientWasm.verifyKeypair(keypair))
    expect(clientJs.verifyKeypair(keypair))
});

test("derivePublicKey", () => {
    const privateKey = clientJs.genKeys().privateKey;
    const derivedPublicKeyJs = clientJs.derivePublicKey(privateKey);
    const derivedPublicKeyWasm = clientWasm.derivePublicKey(privateKey);
    expect(derivedPublicKeyJs).toBe(derivedPublicKeyWasm)
});

test("publicKeyToRaw", () => {
    const pubkey = clientJs.genKeys().publicKey;
    const rawPubkeyJs = clientJs.publicKeyToRaw(pubkey);
    const rawPubkeyWasm = clientWasm.publicKeyToRaw(pubkey);
    expect(rawPubkeyJs).toBe(rawPubkeyWasm)
});

test("signMessage and verifyMessage", () => {
    const message = "This is a sample message."
    const keypair = clientWasm.genKeys();

    const signedMessageJs = clientJs.signMessage(message, keypair);
    expect(clientJs.verifyMessage(signedMessageJs));
    expect(clientWasm.verifyMessage(signedMessageJs));

    const signedMessageWasm = clientWasm.signMessage(message, keypair);
    expect(clientJs.verifyMessage(signedMessageWasm));
    expect(clientWasm.verifyMessage(signedMessageWasm));
});

test("signPayment and verifyPayment", () => {
    const fromKeypair = clientWasm.genKeys();
    const toKeypair = clientWasm.genKeys();
    const payment = {
        to: toKeypair.publicKey,
        from: fromKeypair.publicKey,
        // fee(u64) can be either f64 or bigint
        fee: 1n,
        // amount(u64) can be either f64 or bigint
        amount: 2,
        nonce: 3,
        memo: 'memo',
        validUntil: 0xFFFFFFFF,
    };
    const signedPaymentJs = clientJs.signPayment(payment, fromKeypair.privateKey);
    expect(clientJs.verifyPayment(signedPaymentJs));
    expect(clientWasm.verifyPayment(signedPaymentJs));

    const signedPaymentWasm = clientWasm.signPayment(payment, fromKeypair.privateKey);
    expect(clientJs.verifyPayment(signedPaymentWasm));
    expect(clientWasm.verifyPayment(signedPaymentWasm));
});

test("signStakeDelegation and verifyStakeDelegation", () => {
    const fromKeypair = clientWasm.genKeys();
    const toKeypair = clientWasm.genKeys();
    const stakeDelegation = {
        to: toKeypair.publicKey,
        from: fromKeypair.publicKey,
        // fee(u64) can be either f64 or bigint
        fee: 1n,
        nonce: 3,
        memo: 'memo',
        validUntil: 0xFFFFFFFF,
    };
    const signedStakeDelegationJs = clientJs.signStakeDelegation(stakeDelegation, fromKeypair.privateKey);
    expect(clientJs.verifyStakeDelegation(signedStakeDelegationJs));
    expect(clientWasm.verifyStakeDelegation(signedStakeDelegationJs));

    const signedStakeDelegationWasm = clientWasm.signStakeDelegation(stakeDelegation, fromKeypair.privateKey);
    expect(clientJs.verifyStakeDelegation(signedStakeDelegationWasm));
    expect(clientWasm.verifyStakeDelegation(signedStakeDelegationWasm));
});

test("benchmarks", () => {
    const keypair = clientJs.genKeys();
    const message = "This is a sample message."
    const signedMessage = clientWasm.signMessage(message, keypair);

    const fromKeypair = clientWasm.genKeys();
    const toKeypair = clientWasm.genKeys();
    const payment = {
        to: toKeypair.publicKey,
        from: fromKeypair.publicKey,
        // fee(u64) can be either f64 or bigint
        fee: 1n,
        // amount(u64) can be either f64 or bigint
        amount: 2,
        nonce: 3,
        memo: 'memo',
        validUntil: 0xFFFFFFFF,
    };
    const signedPayment = clientJs.signPayment(payment, fromKeypair.privateKey);
    const stakeDelegation = {
        to: toKeypair.publicKey,
        from: fromKeypair.publicKey,
        // fee(u64) can be either f64 or bigint
        fee: 1n,
        nonce: 3,
        memo: 'memo',
        validUntil: 0xFFFFFFFF,
    };
    const signedStakeDelegation = clientJs.signStakeDelegation(stakeDelegation, fromKeypair.privateKey);

    new Benchmark.Suite()
        .on('cycle', function (event) {
            console.log('\x1b[35m%s\x1b[0m', String(event.target));
        })

        .add('[js]   genKeys', function () {
            clientJs.genKeys();
        })
        .add('[wasm] genKeys', function () {
            clientWasm.genKeys();
        })

        .add('[js]   verifyKeypair', function () {
            clientJs.verifyKeypair(keypair)
        })
        .add('[wasm] verifyKeypair', function () {
            clientWasm.verifyKeypair(keypair);
        })

        .add('[js]   derivePublicKey', function () {
            clientJs.derivePublicKey(keypair.privateKey)
        })
        .add('[wasm] derivePublicKey', function () {
            clientWasm.derivePublicKey(keypair.privateKey);
        })

        .add('[js]   publicKeyToRaw', function () {
            clientJs.publicKeyToRaw(keypair.publicKey)
        })
        .add('[wasm] publicKeyToRaw', function () {
            clientWasm.publicKeyToRaw(keypair.publicKey);
        })

        .add('[js]   signMessage', function () {
            clientJs.signMessage(message, keypair)
        })
        .add('[wasm] signMessage', function () {
            clientWasm.signMessage(message, keypair);
        })

        .add('[js]   verifyMessage', function () {
            clientJs.verifyMessage(signedMessage)
        })
        .add('[wasm] verifyMessage', function () {
            clientWasm.verifyMessage(signedMessage);
        })

        .add('[js]   signPayment', function () {
            clientJs.signPayment(payment, fromKeypair.privateKey)
        })
        .add('[wasm] signPayment', function () {
            clientWasm.signPayment(payment, fromKeypair.privateKey)
        })

        .add('[js]   verifyPayment', function () {
            clientJs.verifyPayment(signedPayment)
        })
        .add('[wasm] verifyPayment', function () {
            clientWasm.verifyPayment(signedPayment)
        })

        .add('[js]   signStateDelegation', function () {
            clientJs.signStakeDelegation(stakeDelegation, fromKeypair.privateKey)
        })
        .add('[wasm] signStateDelegation', function () {
            clientWasm.signStakeDelegation(stakeDelegation, fromKeypair.privateKey)
        })

        .add('[js]   verifyStateDelegation', function () {
            clientJs.verifyStakeDelegation(signedStakeDelegation)
        })
        .add('[wasm] verifyStateDelegation', function () {
            clientWasm.verifyStakeDelegation(signedStakeDelegation)
        })

        .run()
})
