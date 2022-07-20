import ClientJs from "mina-signer";
import { Client as ClientWasm } from "./pkg/mina_signer_wasm";

const clientJs = new ClientJs({ network: "mainnet" });
const clientWasm = new ClientWasm({ network: "mainnet" });

test(
	"genKeys",
	() => {
		const keypair = clientWasm.genKeys();
		expect(clientWasm.verifyKeypair(keypair)).toBe(true);
		expect(clientJs.verifyKeypair(keypair)).toBe(true);
	},
);

test(
	"verifyKeypair",
	() => {
		const keypair = clientJs.genKeys();
		expect(clientWasm.verifyKeypair(keypair)).toBe(true);
		expect(clientJs.verifyKeypair(keypair)).toBe(true);
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
		expect(clientJs.verifyMessage(signedMessageJs)).toBe(true);
		expect(clientWasm.verifyMessage(signedMessageJs)).toBe(true);

		const signedMessageWasm = clientWasm.signMessage(message, keypair);
		expect(clientJs.verifyMessage(signedMessageWasm)).toBe(true);
		expect(clientWasm.verifyMessage(signedMessageWasm)).toBe(true);
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
			fee: "1",
			amount: "1",
			nonce: 3,
			memo: "memo",
			validUntil: 0xffffffff,
		};
		const signedPaymentJs = clientJs.signPayment(
			payment,
			fromKeypair.privateKey,
		);
		expect(clientJs.verifyPayment(signedPaymentJs)).toBe(true);
		expect(clientWasm.verifyPayment(signedPaymentJs)).toBe(true);

		const signedPaymentWasm = clientWasm.signPayment(
			payment,
			fromKeypair.privateKey,
		);
		expect(clientJs.verifyPayment(signedPaymentWasm)).toBe(true);
		expect(clientWasm.verifyPayment(signedPaymentWasm)).toBe(true);
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
			fee: 1n,
			nonce: "3",
			memo: "memo",
			validUntil: 0xFFFFFFFF,
		};
		const signedStakeDelegationJs = clientJs.signStakeDelegation(
			stakeDelegation,
			fromKeypair.privateKey,
		);
		expect(clientJs.verifyStakeDelegation(signedStakeDelegationJs)).toBe(true);
		expect(clientWasm.verifyStakeDelegation(signedStakeDelegationJs)).toBe(
			true,
		);

		const signedStakeDelegationWasm = clientWasm.signStakeDelegation(
			stakeDelegation,
			fromKeypair.privateKey,
		);
		expect(clientJs.verifyStakeDelegation(signedStakeDelegationWasm)).toBe(
			true,
		);
		expect(clientWasm.verifyStakeDelegation(signedStakeDelegationWasm)).toBe(
			true,
		);
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
			memo: "memo",
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
			memo: "memo",
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
				memo: "memo",
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
				memo: "memo",
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
