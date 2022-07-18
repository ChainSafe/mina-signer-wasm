use crate::*;
use ark_ec::AffineCurve;
use mina_curves::pasta::pallas::Affine as CurvePoint;
use mina_signer::PubKey;

#[wasm_bindgen(typescript_custom_section)]
const KEYPAIR: &'static str = r#"
export type PublicKey = string;

export type PrivateKey = string;

export interface Keypair {
    privateKey: PrivateKey;
    publicKey: PublicKey;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Keypair")]
    pub type Keypair;

    #[wasm_bindgen(method, getter, js_name = privateKey)]
    pub fn private_key(this: &Keypair) -> String;

    #[wasm_bindgen(method, getter, js_name = publicKey)]
    pub fn public_key(this: &Keypair) -> String;
}

#[wasm_bindgen(inline_js = r#"
        function new_keypair(privateKey, publicKey) {
            return {
                privateKey,
                publicKey,
            }
        }

        module.exports = {
            new_keypair,
        }
    "#)]
extern "C" {
    pub fn new_keypair(private_key: &str, public_key: &str) -> Keypair;
}

impl TryFrom<Keypair> for MinaKeypair {
    type Error = JsError;

    fn try_from(value: Keypair) -> Result<Self, Self::Error> {
        let decoded = bs58::decode(value.private_key())
            .with_check(Some(constants::PRIVATE_KEY_BASE58_CHECK_VERSION_BYTE))
            .into_vec()
            .map_err(map_js_err)?;
        let private_key_bytes_le = &decoded[2..];
        let private_key =
            <CurvePoint as AffineCurve>::ScalarField::from_bytes(private_key_bytes_le)
                .map_err(map_js_err)?;
        let public_key = PubKey::from_address(value.public_key().as_str()).map_err(map_js_err)?;
        Ok(MinaKeypair::from_parts_unsafe(
            private_key,
            public_key.into_point(),
        ))
    }
}

impl From<MinaKeypair> for Keypair {
    fn from(value: MinaKeypair) -> Self {
        let public_key = value.public.into_address();
        let mut private_key_bytes_le = (*value.secret().scalar()).to_bytes();
        // binprot version byte
        private_key_bytes_le.insert(0, 1);
        let private_key = bs58::encode(private_key_bytes_le)
            .with_check_version(constants::PRIVATE_KEY_BASE58_CHECK_VERSION_BYTE)
            .into_string();
        new_keypair(private_key.as_str(), public_key.as_str())
    }
}
