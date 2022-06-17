use crate::*;
use ark_ff::BigInteger256;
use num_bigint::BigUint;
use std::str::FromStr;

#[wasm_bindgen]
extern "C" {
    pub type Signature;

    #[wasm_bindgen(method, getter)]
    pub fn field(this: &Signature) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn scalar(this: &Signature) -> String;

    pub type SignatureWrapper;

    #[wasm_bindgen(method, getter)]
    pub fn string(this: &SignatureWrapper) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn signer(this: &SignatureWrapper) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn signature(this: &SignatureWrapper) -> Signature;
}

#[wasm_bindgen(inline_js = r#"
        function new_signature(field, scalar) {
            return {
                field,
                scalar,
            }
        }

        module.exports = {
            new_signature,
        }
    "#)]
extern "C" {
    pub fn new_signature(field: String, scalar: String) -> Signature;
}

impl From<MinaSignature> for Signature {
    fn from(value: MinaSignature) -> Self {
        let field: BigUint = value.rx.0.into();
        let scalar: BigUint = value.s.0.into();
        new_signature(field.to_str_radix(10), scalar.to_str_radix(10))
    }
}

impl TryFrom<Signature> for MinaSignature {
    type Error = JsError;

    fn try_from(value: Signature) -> Result<Self, Self::Error> {
        let field: BigInteger256 = BigUint::from_str(value.field().as_str())
            .map_err(map_js_err)?
            .try_into()
            .map_err(map_js_err)?;
        let scalar: BigInteger256 = BigUint::from_str(value.scalar().as_str())
            .map_err(map_js_err)?
            .try_into()
            .map_err(map_js_err)?;
        Ok(MinaSignature {
            rx: field.into(),
            s: scalar.into(),
        })
    }
}
