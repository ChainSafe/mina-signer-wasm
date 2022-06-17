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

test("signMessage", () => {
    const message = "This is a sample message."
    const keypair = clientWasm.genKeys();
    const signedMessageJs = clientJs.signMessage(message, keypair);
    const signedMessageWasm = clientWasm.signMessage(message, keypair);
    expect(clientJs.verifyMessage(signedMessageJs));
    expect(clientWasm.verifyMessage(signedMessageJs));
    expect(clientJs.verifyMessage(signedMessageWasm));
    expect(clientWasm.verifyMessage(signedMessageWasm));
});

test("verifyMessage", () => {
    const message = "This is another sample message."
    const keypair = clientWasm.genKeys();
    const signedMessageJs = clientJs.signMessage(message, keypair);
    const signedMessageWasm = clientWasm.signMessage(message, keypair);
    expect(clientJs.verifyMessage(signedMessageJs));
    expect(clientWasm.verifyMessage(signedMessageJs));
    expect(clientJs.verifyMessage(signedMessageWasm));
    expect(clientWasm.verifyMessage(signedMessageWasm));
});

test("benchmarks", () => {
    const keypair = clientJs.genKeys();
    const message = "This is a sample message."
    const signedMessage = clientWasm.signMessage(message, keypair);

    new Benchmark.Suite()
        .on('cycle', function (event) {
            console.log('\x1b[35m%s\x1b[0m', String(event.target));
        })

        .add('[js] genKeys', function () {
            clientJs.genKeys();
        })
        .add('[wasm] genKeys', function () {
            clientWasm.genKeys();
        })

        .add('[js] verifyKeypair', function () {
            clientJs.verifyKeypair(keypair)
        })
        .add('[wasm] verifyKeypair', function () {
            clientWasm.verifyKeypair(keypair);
        })

        .add('[js] derivePublicKey', function () {
            clientJs.derivePublicKey(keypair.privateKey)
        })
        .add('[wasm] derivePublicKey', function () {
            clientWasm.derivePublicKey(keypair.privateKey);
        })

        .add('[js] publicKeyToRaw', function () {
            clientJs.publicKeyToRaw(keypair.publicKey)
        })
        .add('[wasm] publicKeyToRaw', function () {
            clientWasm.publicKeyToRaw(keypair.publicKey);
        })

        .add('[js] signMessage', function () {
            clientJs.signMessage(message, keypair)
        })
        .add('[wasm] signMessage', function () {
            clientWasm.signMessage(message, keypair);
        })

        .add('[js] verifyMessage', function () {
            clientJs.verifyMessage(signedMessage)
        })
        .add('[wasm] verifyMessage', function () {
            clientWasm.verifyMessage(signedMessage);
        })

        .run()
})
