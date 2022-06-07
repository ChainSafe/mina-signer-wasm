const ClientJs = require("mina-signer");
const wasm = require("./pkg/mina_signer_wasm")
const ClientWasm = wasm.Client

const clientJs = new ClientJs({ network: "mainnet" });
const clientWasm = new ClientWasm({ network: "mainnet" });

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
