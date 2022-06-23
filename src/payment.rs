use crate::*;
use mina_hasher::{Hashable, ROInput};
use mina_serialization_types::{common::*, json::*};
use mina_signer::{CompressedPubKey, NetworkId};
use num_traits::identities::One;

#[wasm_bindgen]
extern "C" {
    pub type Payment;

    #[wasm_bindgen(method, getter)]
    pub fn to(this: &Payment) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn from(this: &Payment) -> String;

    // u64 can be either f64 or bigint in js,
    // use JsValue to handle the conversion
    #[wasm_bindgen(method, getter)]
    pub fn fee(this: &Payment) -> JsValue;

    // u64 can be either f64 or bigint in js,
    // use JsValue to handle the conversion
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Payment) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn nonce(this: &Payment) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn memo(this: &Payment) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = validUntil)]
    pub fn valid_until(this: &Payment) -> Option<u32>;

    pub type SignedPayment;

    #[wasm_bindgen(method, getter)]
    pub fn signature(this: &SignedPayment) -> Signature;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &SignedPayment) -> Payment;
}

#[wasm_bindgen(inline_js = r#"
        function new_payment(to, from, fee, amount, nonce, memo, validUntil) {
            return {
                to,
                from,
                fee,
                amount,
                nonce,
                memo,
                validUntil,
            }
        }

        function new_signed_payment(signature, data) {
            return {
                signature,
                data,
            }
        }

        module.exports = {
            new_payment, new_signed_payment
        }
    "#)]
extern "C" {
    pub fn new_payment(
        to: String,
        from: String,
        fee: u64,
        amount: u64,
        nonce: u32,
        memo: Option<String>,
        valid_until: Option<u32>,
    ) -> Payment;

    pub fn new_signed_payment(signature: Signature, data: Payment) -> SignedPayment;
}

impl Payment {
    pub fn try_to_mina_payment(&self) -> Result<MinaPayment, JsError> {
        Ok(MinaPayment {
            to: CompressedPubKey::from_address(self.to().as_str()).map_err(map_js_err)?,
            from: CompressedPubKey::from_address(self.from().as_str()).map_err(map_js_err)?,
            fee: js_to_string(&self.fee()).parse().map_err(map_js_err)?,
            amount: js_to_string(&self.amount()).parse().map_err(map_js_err)?,
            nonce: self.nonce(),
            memo: string_to_memo(self.memo()),
            valid_until: match self.valid_until() {
                None => u32::max_value(),
                Some(i) => i,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct MinaPayment {
    pub to: CompressedPubKey,
    pub from: CompressedPubKey,
    pub fee: u64,
    pub amount: u64,
    pub nonce: u32,
    pub memo: [u8; constants::MEMO_BYTES],
    pub valid_until: u32,
}

impl From<MinaPayment> for Payment {
    fn from(v: MinaPayment) -> Self {
        let mut memo_len = v.memo.len();
        while memo_len > 0 && v.memo[memo_len - 1] == 0 {
            memo_len -= 1;
        }
        let memo = if memo_len == 0 {
            None
        } else {
            Some(unsafe { String::from_utf8_unchecked(v.memo[..memo_len].into()) })
        };
        new_payment(
            v.to.into_address(),
            v.from.into_address(),
            v.fee,
            v.amount,
            v.nonce,
            memo,
            Some(v.valid_until),
        )
    }
}

impl TryFrom<Payment> for MinaPayment {
    type Error = JsError;

    fn try_from(v: Payment) -> Result<Self, Self::Error> {
        v.try_to_mina_payment()
    }
}

impl TryFrom<Payment> for SignedCommandJson {
    type Error = JsError;

    fn try_from(v: Payment) -> Result<Self, Self::Error> {
        let p: MinaPayment = v.try_into()?;
        let dummy_signature = MinaSignature {
            rx: <CurvePoint as AffineCurve>::BaseField::one(),
            s: <CurvePoint as AffineCurve>::ScalarField::one(),
        };
        Ok(Self {
            payload: SignedCommandPayloadJson {
                common: SignedCommandPayloadCommonJson {
                    fee: DecimalJson(p.fee),
                    fee_token: U64Json(1),
                    nonce: U32Json(p.nonce),
                    valid_until: U32Json(p.valid_until),
                    fee_payer_pk: compressed_pubkey_to_json(p.from),
                    memo: SignedCommandMemoJson(p.memo.to_vec()),
                },
                body: SignedCommandPayloadBodyJson::PaymentPayload(PaymentPayloadJson {
                    source_pk: compressed_pubkey_to_json(p.from),
                    receiver_pk: compressed_pubkey_to_json(p.to),
                    token_id: U64Json(1),
                    amount: U64Json(p.amount),
                }),
            },
            signer: compressed_pubkey_to_json(p.from),
            signature: signature_to_json(dummy_signature),
        })
    }
}

impl TryFrom<SignedPayment> for SignedCommandJson {
    type Error = JsError;

    fn try_from(v: SignedPayment) -> Result<Self, Self::Error> {
        let mut result: Self = v.data().try_into()?;
        result.signature = signature_to_json(v.signature().try_into()?);
        Ok(result)
    }
}

impl Hashable for MinaPayment {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        roi.append_field(self.from.x);
        roi.append_field(self.from.x);
        roi.append_field(self.to.x);

        roi.append_u64(self.fee);
        // fee token
        roi.append_u64(1);
        roi.append_bool(self.from.is_odd);
        roi.append_u32(self.nonce);
        roi.append_u32(self.valid_until);
        roi.append_bytes(&self.memo);

        for tag_bit in constants::PAYMENT_TX_TAG {
            roi.append_bool(tag_bit);
        }

        roi.append_bool(self.from.is_odd);
        roi.append_bool(self.to.is_odd);
        // token id
        roi.append_u64(1);
        roi.append_u64(self.amount);
        // token locked
        roi.append_bool(false);

        roi
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        // Domain strings must have length <= 20
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}
