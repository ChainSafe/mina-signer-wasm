use crate::*;
use ark_ec::AffineCurve;
use mina_curves::pasta::pallas::Affine as CurvePoint;
use mina_signer::{NetworkId, PubKey, Signer};
use o1_utils::field_helpers::FieldHelpers;

#[wasm_bindgen]
extern "C" {
    pub type ClientOptions;

    #[wasm_bindgen(method, getter)]
    pub fn network(this: &ClientOptions) -> String;

}

/// <https://cdn.jsdelivr.net/npm/mina-signer@1.1.0/dist/src/MinaSigner.d.ts>
#[wasm_bindgen]
pub struct Client {
    ptr: *const ClientImpl,
}

impl Client {
    fn client(&self) -> &ClientImpl {
        unsafe { &*self.ptr }
    }
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(options: &ClientOptions) -> Result<Client, JsError> {
        let network = options.network();
        if network.is_empty() {
            Err(JsError::new(
                "Network field should not be empty, expect 'mainnet' or 'testnet'",
            ))
        } else {
            let client_impl = ClientImpl::new(network);
            Ok(Client {
                ptr: Box::into_raw(Box::new(client_impl)),
            })
        }
    }

    pub fn free(&self) {
        unsafe {
            Box::from_raw(self.ptr as *mut ClientImpl);
        }
    }

    #[wasm_bindgen(js_name = genKeys)]
    pub fn gen_keys(&self) -> Keypair {
        self.client().gen_keys()
    }

    #[wasm_bindgen(js_name = verifyKeypair)]
    pub fn verify_keypair(&self, keypair: Keypair) -> Result<bool, JsError> {
        self.client().verify_keypair(keypair)
    }

    #[wasm_bindgen(js_name = derivePublicKey)]
    pub fn derive_public_key(&self, private_key: String) -> Result<String, JsError> {
        self.client().derive_public_key(private_key)
    }

    #[wasm_bindgen(js_name = publicKeyToRaw)]
    pub fn public_key_to_raw(&self, public_key: &str) -> Result<String, JsError> {
        let mut decoded = bs58::decode(public_key)
            .with_check(Some(0xcb))
            .into_vec()
            .map_err(map_js_err)?;
        if decoded.len() == 36 {
            let odd = decoded[35];
            let compressed = &mut decoded[3..35];
            if odd > 0 {
                let last = &mut compressed[31];
                *last |= 0x80;
            }
            Ok(hex::encode_upper(compressed))
        } else {
            Err(JsError::new(&format!("Invalid length: {}", decoded.len())))
        }
    }

    #[wasm_bindgen(js_name = signMessage)]
    pub fn sign_message(
        &self,
        message: String,
        keypair: Keypair,
    ) -> Result<SignedMessage, JsError> {
        let js_message = new_message(keypair.public_key().as_str(), message.as_str());
        let signature = self.client().sign_message(message, &keypair.try_into()?);
        Ok(new_signed_message(signature.into(), js_message))
    }

    #[wasm_bindgen(js_name = verifyMessage)]
    pub fn verify_message(&self, message: SignedMessage) -> Result<bool, JsError> {
        self.client().verify_message(message)
    }
}

pub struct ClientImpl {
    pub network: String,
}

impl ClientImpl {
    pub fn new(network: String) -> Self {
        Self { network }
    }

    pub fn gen_keys(&self) -> Keypair {
        MinaKeypair::rand(&mut rand::rngs::OsRng).into()
    }

    pub fn verify_keypair(&self, keypair: Keypair) -> Result<bool, JsError> {
        let mina_keypair: MinaKeypair = keypair.try_into()?;
        Ok(mina_keypair.validate())
    }

    pub fn derive_public_key(&self, private_key: String) -> Result<String, JsError> {
        let decoded = bs58::decode(private_key)
            .with_check(Some(constants::PRIVATE_KEY_BASE58_CHECK_VERSION_BYTE))
            .into_vec()
            .map_err(map_js_err)?;
        let private_key_bytes_le = &decoded[2..];
        let private_key =
            <CurvePoint as AffineCurve>::ScalarField::from_bytes(private_key_bytes_le)
                .map_err(map_js_err)?;
        let keypair = MinaKeypair::from_secret(private_key).map_err(map_js_err)?;
        Ok(keypair.public.into_address())
    }

    pub fn sign_message(&self, message: String, keypair: &MinaKeypair) -> MinaSignature {
        let mut ctx = mina_signer::create_legacy::<StringMessage>(self.network_id());
        ctx.sign(keypair, &message.into())
    }

    pub fn verify_message(&self, message: SignedMessage) -> Result<bool, JsError> {
        let signature: MinaSignature = message.signature_wrapper().signature().try_into()?;
        let data = message.data();
        let public_key = PubKey::from_address(data.public_key().as_str()).map_err(map_js_err)?;
        let payload: StringMessage = data.message().into();
        let mut ctx = mina_signer::create_legacy::<StringMessage>(self.network_id());
        Ok(ctx.verify(&signature, &public_key, &payload))
    }

    fn network_id(&self) -> NetworkId {
        match self.network.as_str() {
            "mainnet" => NetworkId::MAINNET,
            _ => NetworkId::TESTNET,
        }
    }
}
