use crate::*;
use mina_hasher::{Hashable, ROInput};
use mina_signer::{CompressedPubKey, NetworkId};

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
        let mut memo = [0; constants::MEMO_BYTES];
        if let Some(s) = self.memo() {
            for (i, &b) in s.as_bytes().iter().enumerate() {
                memo[i] = b;
            }
        }

        Ok(MinaPayment {
            to: CompressedPubKey::from_address(self.to().as_str()).map_err(map_js_err)?,
            from: CompressedPubKey::from_address(self.from().as_str()).map_err(map_js_err)?,
            fee: self.fee().as_f64().unwrap_or_default() as u64,
            amount: self.amount().as_f64().unwrap_or_default() as u64,
            nonce: self.nonce(),
            memo,
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
