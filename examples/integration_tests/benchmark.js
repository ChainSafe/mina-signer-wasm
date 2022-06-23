const ClientJs = require("mina-signer");
const wasm = require("./pkg/mina_signer_wasm")
const ClientWasm = wasm.Client

const clientJs = new ClientJs({ network: "mainnet" });
const clientWasm = new ClientWasm({ network: "mainnet" });

const Benchmark = require('benchmark');

function main() {
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
    const signedRosettaTransactionPayment = JSON.stringify({
        signature: "389ac7d4077f3d485c1494782870979faa222cd906b25b2687333a92f41e40b925adb08705eddf2a7098e5ac9938498e8a0ce7c70b25ea392f4846b854086d43",
        payment: {
            to: "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
            from: "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
            fee: "10000000",
            token: "1",
            nonce: "0",
            memo: null,
            amount: "1000000000",
            valid_until: "4294967295"
        },
        stake_delegation: null,
        create_token: null,
        create_token_account: null,
        mint_tokens: null
    });
    const signedRosettaTransactionStakeDelegation = JSON.stringify({
        signature: "389ac7d4077f3d485c1494782870979faa222cd906b25b2687333a92f41e40b925adb08705eddf2a7098e5ac9938498e8a0ce7c70b25ea392f4846b854086d43",
        payment: null,
        stake_delegation: {
            new_delegate: "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
            delegator: "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
            fee: "10000000",
            nonce: "0",
            memo: null,
            valid_until: "4294967295"
        },
        create_token: null,
        create_token_account: null,
        mint_tokens: null
    });

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

        .add('[js]   hashPayment', function () {
            clientJs.hashPayment(signedPayment)
        })
        .add('[wasm] hashPayment', function () {
            clientWasm.hashPayment(signedPayment)
        })

        .add('[js]   hashStakeDelegation', function () {
            clientJs.hashStakeDelegation(signedStakeDelegation)
        })
        .add('[wasm] hashStakeDelegation', function () {
            clientWasm.hashStakeDelegation(signedStakeDelegation)
        })

        .add('[js]   signedRosettaTransactionToSignedCommand - Payment', function () {
            clientJs.signedRosettaTransactionToSignedCommand(signedRosettaTransactionPayment)
        })
        .add('[wasm] signedRosettaTransactionToSignedCommand - Payment', function () {
            clientWasm.signedRosettaTransactionToSignedCommand(signedRosettaTransactionPayment)
        })

        .add('[js]   signedRosettaTransactionToSignedCommand - StakeDelegation', function () {
            clientJs.signedRosettaTransactionToSignedCommand(signedRosettaTransactionStakeDelegation)
        })
        .add('[wasm] signedRosettaTransactionToSignedCommand - StakeDelegation', function () {
            clientWasm.signedRosettaTransactionToSignedCommand(signedRosettaTransactionStakeDelegation)
        })

        .run()
}

main()
