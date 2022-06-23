use crate::*;
use mina_serialization_types::json::*;
use mina_signer::CompressedPubKey;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    pub type SignedRosettaTransaction;

    #[wasm_bindgen(method, getter)]
    pub fn signature(this: &SignedRosettaTransaction) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn payment(this: &SignedRosettaTransaction) -> Option<Payment>;

    #[wasm_bindgen(method, getter)]
    pub fn stake_delegation(this: &SignedRosettaTransaction) -> Option<RosettaStakeDelegation>;

    #[wasm_bindgen(method, getter)]
    pub fn create_token(this: &SignedRosettaTransaction) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn create_token_account(this: &SignedRosettaTransaction) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn mint_tokens(this: &SignedRosettaTransaction) -> Option<String>;

    pub type RosettaStakeDelegation;

    #[wasm_bindgen(method, getter)]
    pub fn new_delegate(this: &RosettaStakeDelegation) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn delegator(this: &RosettaStakeDelegation) -> String;

    // u64 can be either f64 or bigint in js,
    // use JsValue to handle the conversion
    #[wasm_bindgen(method, getter)]
    pub fn fee(this: &RosettaStakeDelegation) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn nonce(this: &RosettaStakeDelegation) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn memo(this: &RosettaStakeDelegation) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = validUntil)]
    pub fn valid_until(this: &RosettaStakeDelegation) -> JsValue;
}

impl RosettaStakeDelegation {
    pub fn fee_u64(&self) -> Result<u64, JsError> {
        js_to_string(&self.fee()).parse().map_err(map_js_err)
    }

    pub fn nonce_u32(&self) -> Result<u32, JsError> {
        js_to_string(&self.nonce()).parse().map_err(map_js_err)
    }

    pub fn valid_until_u32(&self) -> Result<Option<u32>, JsError> {
        let valid_until = self.valid_until();
        if valid_until.is_null() || valid_until.is_undefined() {
            Ok(None)
        } else {
            let s = js_to_string(&valid_until);
            if s.is_empty() {
                Ok(None)
            } else {
                Ok(Some(s.parse().map_err(map_js_err)?))
            }
        }
    }

    pub fn try_to_mina_stake_delegation(&self) -> Result<MinaStakeDelegation, JsError> {
        Ok(MinaStakeDelegation {
            to: CompressedPubKey::from_address(self.new_delegate().as_str()).map_err(map_js_err)?,
            from: CompressedPubKey::from_address(self.delegator().as_str()).map_err(map_js_err)?,
            fee: self.fee_u64()?,
            nonce: self.nonce_u32()?,
            memo: string_to_memo(self.memo()),
            valid_until: match self.valid_until_u32()? {
                None => u32::max_value(),
                Some(i) => i,
            },
        })
    }
}

impl TryFrom<RosettaStakeDelegation> for MinaStakeDelegation {
    type Error = JsError;

    fn try_from(v: RosettaStakeDelegation) -> Result<Self, Self::Error> {
        v.try_to_mina_stake_delegation()
    }
}

impl TryFrom<RosettaStakeDelegation> for SignedCommandJson {
    type Error = JsError;

    fn try_from(v: RosettaStakeDelegation) -> Result<Self, Self::Error> {
        let p: MinaStakeDelegation = v.try_into()?;
        Ok(p.into())
    }
}

#[wasm_bindgen(inline_js = r#"
        function signed_rosetta_transaction_from_str(s) {
            try {
                return JSON.parse(s)
            } catch(e) {
                console.log(`signed_rosetta_transaction_from_str: ${e}`)
                return null
            }
        }

        module.exports = {
            signed_rosetta_transaction_from_str
        }
    "#)]
extern "C" {
    pub fn signed_rosetta_transaction_from_str(s: String) -> Option<SignedRosettaTransaction>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedCommandGraphQLJson {
    pub data: SignedCommandJson,
}
