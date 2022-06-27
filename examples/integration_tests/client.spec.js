const ClientJs = require("mina-signer");
const wasm = require("./pkg/mina_signer_wasm");
const ClientWasm = wasm.Client;

const clientJs = new ClientJs({ network: "mainnet" });
const clientWasm = new ClientWasm({ network: "mainnet" });

test(
	"genKeys",
	() => {
		const keypair = clientWasm.genKeys();
		expect(clientWasm.verifyKeypair(keypair));
		expect(clientJs.verifyKeypair(keypair));
	},
);

test(
	"verifyKeypair",
	() => {
		const keypair = clientJs.genKeys();
		expect(clientWasm.verifyKeypair(keypair));
		expect(clientJs.verifyKeypair(keypair));
	},
);

test(
	"derivePublicKey",
	() => {
		const privateKey = clientJs.genKeys().privateKey;
		const derivedPublicKeyJs = clientJs.derivePublicKey(privateKey);
		const derivedPublicKeyWasm = clientWasm.derivePublicKey(privateKey);
		expect(derivedPublicKeyJs).toBe(derivedPublicKeyWasm);
	},
);

test(
	"publicKeyToRaw",
	() => {
		const pubkey = clientJs.genKeys().publicKey;
		const rawPubkeyJs = clientJs.publicKeyToRaw(pubkey);
		const rawPubkeyWasm = clientWasm.publicKeyToRaw(pubkey);
		expect(rawPubkeyJs).toBe(rawPubkeyWasm);
	},
);

test(
	"signMessage and verifyMessage",
	() => {
		const message = "This is a sample message.";
		const keypair = clientWasm.genKeys();

		const signedMessageJs = clientJs.signMessage(message, keypair);
		expect(clientJs.verifyMessage(signedMessageJs));
		expect(clientWasm.verifyMessage(signedMessageJs));

		const signedMessageWasm = clientWasm.signMessage(message, keypair);
		expect(clientJs.verifyMessage(signedMessageWasm));
		expect(clientWasm.verifyMessage(signedMessageWasm));
	},
);

test(
	"signPayment and verifyPayment",
	() => {
		const fromKeypair = clientWasm.genKeys();
		const toKeypair = clientWasm.genKeys();
		const payment = {
			to: toKeypair.publicKey,
			from: fromKeypair.publicKey,
			// fee(u64) can be either f64 or bigint
			fee: 1n,
			// amount(u64) can be either f64 or bigint
			amount: 2,
			nonce: "3",
			memo: "memo",
			validUntil: 0xFFFFFFFF,
		};
		const signedPaymentJs = clientJs.signPayment(
			payment,
			fromKeypair.privateKey,
		);
		expect(clientJs.verifyPayment(signedPaymentJs));
		expect(clientWasm.verifyPayment(signedPaymentJs));

		const signedPaymentWasm = clientWasm.signPayment(
			payment,
			fromKeypair.privateKey,
		);
		expect(clientJs.verifyPayment(signedPaymentWasm));
		expect(clientWasm.verifyPayment(signedPaymentWasm));
	},
);

test(
	"signStakeDelegation and verifyStakeDelegation",
	() => {
		const fromKeypair = clientWasm.genKeys();
		const toKeypair = clientWasm.genKeys();
		const stakeDelegation = {
			to: toKeypair.publicKey,
			from: fromKeypair.publicKey,
			// fee(u64) can be either f64 or bigint
			fee: 1n,
			nonce: "3",
			memo: "memo",
			validUntil: 0xFFFFFFFF,
		};
		const signedStakeDelegationJs = clientJs.signStakeDelegation(
			stakeDelegation,
			fromKeypair.privateKey,
		);
		expect(clientJs.verifyStakeDelegation(signedStakeDelegationJs));
		expect(clientWasm.verifyStakeDelegation(signedStakeDelegationJs));

		const signedStakeDelegationWasm = clientWasm.signStakeDelegation(
			stakeDelegation,
			fromKeypair.privateKey,
		);
		expect(clientJs.verifyStakeDelegation(signedStakeDelegationWasm));
		expect(clientWasm.verifyStakeDelegation(signedStakeDelegationWasm));
	},
);

test(
	"hashPayment",
	() => {
		// From block mainnet-117896-3NKjZ5fjms6BMaH4aq7DopPGyMY7PbG6vhRsX5XnYRxih8i9G7dj
		const payment = {
			to: "B62qnsHmPQpZSKnrp978ZHFYwCJFBZtY1qE3UD97dd7taQarEV6ZpuG",
			from: "B62qnqEqsuH7kST9ZrbksRzihXD2tgHfvq9TF73XKAMj47gisT9xsJ5",
			fee: 200100000n,
			amount: "16640000000000",
			nonce: 1,
		};
		const signedPayment = clientWasm.signPayment(
			payment,
			clientWasm.genKeys().privateKey,
		);
		const hashJs = clientJs.hashPayment(signedPayment);
		const hashWasm = clientWasm.hashPayment(signedPayment);
		expect(hashJs).toBe(hashWasm);
	},
);

test(
	"hashStakeDelegation",
	() => {
		const stakeDelegation = {
			to: "B62qnsHmPQpZSKnrp978ZHFYwCJFBZtY1qE3UD97dd7taQarEV6ZpuG",
			from: "B62qnqEqsuH7kST9ZrbksRzihXD2tgHfvq9TF73XKAMj47gisT9xsJ5",
			fee: 200100000,
			nonce: 1,
		};
		const signedStakeDelegation = clientWasm.signStakeDelegation(
			stakeDelegation,
			clientWasm.genKeys().privateKey,
		);
		const hashJs = clientJs.hashStakeDelegation(signedStakeDelegation);
		const hashWasm = clientWasm.hashStakeDelegation(signedStakeDelegation);

		expect(hashJs).toBe(hashWasm);
	},
);

test(
	"signedRosettaTransactionToSignedCommand - Payment",
	() => {
		const signedRosettaTransaction = {
			signature:
				"389ac7d4077f3d485c1494782870979faa222cd906b25b2687333a92f41e40b925adb08705eddf2a7098e5ac9938498e8a0ce7c70b25ea392f4846b854086d43",
			payment: {
				to: "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
				from: "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
				fee: "10000000",
				token: "1",
				nonce: "0",
				memo: null,
				amount: "1000000000",
				valid_until: "4294967295",
			},
			stake_delegation: null,
			create_token: null,
			create_token_account: null,
			mint_tokens: null,
		};
		const signedGraphQLCommandJs = clientJs.signedRosettaTransactionToSignedCommand(
			JSON.stringify(signedRosettaTransaction),
		);
		expect(signedGraphQLCommandJs).toBeDefined();
		const signedGraphQLCommandWasm = clientWasm.signedRosettaTransactionToSignedCommand(
			JSON.stringify(signedRosettaTransaction),
		);
		expect(signedGraphQLCommandWasm).toBeDefined();
		expect(JSON.parse(signedGraphQLCommandJs)).toEqual(
			JSON.parse(signedGraphQLCommandWasm),
		);
	},
);

test(
	"signedRosettaTransactionToSignedCommand - StakeDelegation",
	() => {
		const signedRosettaTransaction = {
			signature:
				"389ac7d4077f3d485c1494782870979faa222cd906b25b2687333a92f41e40b925adb08705eddf2a7098e5ac9938498e8a0ce7c70b25ea392f4846b854086d43",
			payment: null,
			stake_delegation: {
				new_delegate: "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
				delegator: "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
				fee: "10000000",
				nonce: "0",
				memo: null,
				valid_until: "4294967295",
			},
			create_token: null,
			create_token_account: null,
			mint_tokens: null,
		};
		const signedGraphQLCommandJs = clientJs.signedRosettaTransactionToSignedCommand(
			JSON.stringify(signedRosettaTransaction),
		);
		expect(signedGraphQLCommandJs).toBeDefined();
		const signedGraphQLCommandWasm = clientWasm.signedRosettaTransactionToSignedCommand(
			JSON.stringify(signedRosettaTransaction),
		);
		expect(signedGraphQLCommandWasm).toBeDefined();
		expect(JSON.parse(signedGraphQLCommandJs)).toEqual(
			JSON.parse(signedGraphQLCommandWasm),
		);
	},
);
