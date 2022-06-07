use derive_more::{From, Into};
use mina_hasher::{Hashable, ROInput};
use mina_signer::NetworkId;

use crate::*;

#[wasm_bindgen]
extern "C" {
    pub type Message;

    #[wasm_bindgen(method, getter, js_name = publicKey)]
    pub fn public_key(this: &Message) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn message(this: &Message) -> String;

    pub type SignedMessage;

    pub type SignatureWrapper;

    #[wasm_bindgen(method, getter)]
    pub fn string(this: &SignatureWrapper) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn signer(this: &SignatureWrapper) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn signature(this: &SignatureWrapper) -> Signature;

    #[wasm_bindgen(method, getter, js_name = signature)]
    pub fn signature_wrapper(this: &SignedMessage) -> SignatureWrapper;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &SignedMessage) -> Message;
}

#[wasm_bindgen(inline_js = r#"
        function new_message(publicKey, message) {
            return {
                publicKey,
                message,
            }
        }

        function new_signed_message(signature, data) {
            return {
                signature: {
                    string: data.message,
                    signer: data.publicKey,
                    signature,
                },
                data,
            }
        }

        module.exports = {
            new_message, new_signed_message
        }
    "#)]
extern "C" {
    pub fn new_message(public_key: &str, message: &str) -> Message;

    pub fn new_signed_message(signature: Signature, message: Message) -> SignedMessage;
}

#[derive(Debug, Clone, From, Into)]
pub struct StringMessage(pub String);

impl Hashable for StringMessage {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        for &b in self.0.as_bytes() {
            let lo = b & 0x0f;
            let hi = (b & 0xf0) >> 4;
            for v in [hi, lo] {
                let bits = match v {
                    0x00 => [false, false, false, false],
                    0x01 => [false, false, false, true],
                    0x02 => [false, false, true, false],
                    0x03 => [false, false, true, true],
                    0x04 => [false, true, false, false],
                    0x05 => [false, true, false, true],
                    0x06 => [false, true, true, false],
                    0x07 => [false, true, true, true],
                    0x08 => [true, false, false, false],
                    0x09 => [true, false, false, true],
                    0x0a => [true, false, true, false],
                    0x0b => [true, false, true, true],
                    0x0c => [true, true, false, false],
                    0x0d => [true, true, false, true],
                    0x0e => [true, true, true, false],
                    0x0f => [true, true, true, true],
                    _ => panic!("unexpected value {v}"),
                };
                for b in bits {
                    roi.append_bool(b);
                }
            }
        }
        roi
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}
