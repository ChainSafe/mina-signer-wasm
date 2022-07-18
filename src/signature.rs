use crate::*;
use ark_ff::BigInteger256;
use num_bigint::BigUint;
use num_traits::Num;

#[wasm_bindgen(typescript_custom_section)]
const SIGNATURE: &'static str = r#"
export interface Signature {
    field: string;
    scalar: string;
}

export interface Signed<SignableData> {
    signature: Signature;
    data: SignableData;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Signature")]
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
        let field: BigInteger256 = value.rx.into();
        let field: BigUint = field.into();
        let scalar: BigInteger256 = value.s.into();
        let scalar: BigUint = scalar.into();
        new_signature(field.to_str_radix(10), scalar.to_str_radix(10))
    }
}

impl TryFrom<Signature> for MinaSignature {
    type Error = JsError;

    fn try_from(value: Signature) -> Result<Self, Self::Error> {
        let field: BigInteger256 = BigUint::from_str_radix(value.field().as_str(), 10)
            .map_err(map_js_err)?
            .try_into()
            .map_err(map_js_err)?;
        let scalar: BigInteger256 = BigUint::from_str_radix(value.scalar().as_str(), 10)
            .map_err(map_js_err)?
            .try_into()
            .map_err(map_js_err)?;
        Ok(MinaSignature {
            rx: field.into(),
            s: scalar.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn signature_convert_roundtrip() {
        let field = "7951369555944720312047598238918799034092275686554435996661497930824346682019";
        let scalar = "7951369555944720312047598238918799034092275686554435996661497930824346682019";
        let sig_js = new_signature(field.into(), scalar.into());
        let sig_rs: MinaSignature = sig_js
            .try_into()
            .map_err(|_| "fail to convert sig_js into sig_rs")
            .unwrap();
        let sig_js_2: Signature = sig_rs.into();
        assert_eq!(&sig_js_2.field(), field);
        assert_eq!(&sig_js_2.scalar(), scalar);
    }
}
