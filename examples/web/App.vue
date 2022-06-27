<template>
    <h2>mina-signer-wasm demo</h2>
    <p>wasm: {{ wasmStatus() }}</p>
    <p><button @click="genKeys">Generate Keypair</button>
    </p>
    <p>
        {{ JSON.stringify(keypair, null, 2) }}
    </p>
    <p>Payment:
    </p>
    <p><textarea v-model="paymentStr"></textarea></p>
    <p><button @click="signPayment">Sign Payment</button>
    </p>
    <p><textarea v-model="signedPaymentStr"></textarea></p>
    <p><button @click="verifySignedPayment">Verify Signed Payment</button>
    </p>
    <p>{{ isSignedPaymentValid }}</p>
    <p><button @click="hashSignedPayment">Hash Signed Payment</button>
    </p>
    <p>{{ signedPaymentHash }}</p>
</template>
<script lang="ts">
import wasmUrl from "raw:./pkg/mina_signer_wasm_bg.wasm";
import init, { Client } from "./pkg/mina_signer_wasm";

export default {
    data() {
        return {
            client: null,
            wasmLoaded: false,
            keypair: {
                privateKey: '',
                publicKey: '',
            },
            paymentStr: JSON.stringify({
                to: 'B62qnsHmPQpZSKnrp978ZHFYwCJFBZtY1qE3UD97dd7taQarEV6ZpuG',
                from: 'B62qnqEqsuH7kST9ZrbksRzihXD2tgHfvq9TF73XKAMj47gisT9xsJ5',
                fee: 1,
                amount: 2,
                nonce: "3",
                validUntil: 0xFFFFFFFF,
            }, null, 2),
            signedPaymentStr: '',
            signedPaymentHash: '',
            isSignedPaymentValid: '',
        };
    },
    async created() {
        await init(await fetch(wasmUrl));
        this.wasmLoaded = true;
        this.client = new Client({ network: 'mainnet' });
    },
    methods: {
        wasmStatus() {
            return this.wasmLoaded ? "loaded" : "loading";
        },
        genKeys() {
            this.keypair = this.client.genKeys();
        },
        signPayment() {
            try {
                if (!this.keypair.privateKey) {
                    alert('Generate keypair first');
                    return;
                }
                const payment = JSON.parse(this.paymentStr);
                const signedPayment = this.client.signPayment(payment, this.keypair.privateKey);
                this.signedPaymentStr = JSON.stringify(signedPayment, null, 2);
            } catch (e) {
                this.signedPaymentStr = `${e}`;
            }
        },
        verifySignedPayment() {
            try {
                const signedPayment = JSON.parse(this.signedPaymentStr);
                this.isSignedPaymentValid = `${this.client.verifyPayment(signedPayment)}`;
            } catch (e) {
                this.isSignedPaymentValid = `${e}`;
            }
        },
        hashSignedPayment() {
            try {
                const signedPayment = JSON.parse(this.signedPaymentStr);
                this.signedPaymentHash = this.client.hashPayment(signedPayment);
            } catch (e) {
                this.signedPaymentHash = `${e}`;
            }
        },
    }
};
</script>
<style lang="scss" scoped>
textarea {
    width: 75%;
    height: 200px;
}
</style>
