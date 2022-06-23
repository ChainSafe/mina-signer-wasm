use crate::*;
use ark_ff::PrimeField;
use blake2::digest::VariableOutput;
use mina_serialization_types::{json::*, v1::*};
use mina_signer::{NetworkId, PubKey, Signer};
use std::io::Write;

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
        let pk = PubKey::from_address(public_key).map_err(map_js_err)?;
        let point = pk.into_point();
        Ok(format!("{}{}", point.x.into_repr(), point.y.into_repr(),))
    }

    #[wasm_bindgen(js_name = publicKeyToRawBeta)]
    pub fn public_key_to_raw_beta(&self, public_key: &str) -> Result<String, JsError> {
        let mut decoded = bs58::decode(public_key)
            .with_check(Some(constants::PUBLIC_KEY_BASE58_CHECK_VERSION_BYTE))
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
    pub fn verify_message(&self, signed_message: SignedMessage) -> Result<bool, JsError> {
        self.client().verify_message(signed_message)
    }

    #[wasm_bindgen(js_name = signPayment)]
    pub fn sign_payment(
        &self,
        payment: Payment,
        private_key: String,
    ) -> Result<SignedPayment, JsError> {
        let client = self.client();
        let keypair: MinaKeypair = new_keypair(
            private_key.as_str(),
            client.derive_public_key(private_key.clone())?.as_str(),
        )
        .try_into()?;
        let signature = client.sign_payment(&payment.try_to_mina_payment()?, &keypair);
        Ok(new_signed_payment(signature.into(), payment))
    }

    #[wasm_bindgen(js_name = verifyPayment)]
    pub fn verify_payment(&self, signed_payment: SignedPayment) -> Result<bool, JsError> {
        self.client().verify_payment(signed_payment)
    }

    #[wasm_bindgen(js_name = signStakeDelegation)]
    pub fn sign_stake_delegation(
        &self,
        stake_delegation: StakeDelegation,
        private_key: String,
    ) -> Result<SignedStakeDelegation, JsError> {
        let client = self.client();
        let keypair: MinaKeypair = new_keypair(
            private_key.as_str(),
            client.derive_public_key(private_key.clone())?.as_str(),
        )
        .try_into()?;
        let signature = client
            .sign_stake_delegation(&stake_delegation.try_to_mina_stake_delegation()?, &keypair);
        Ok(new_signed_stake_delegation(
            signature.into(),
            stake_delegation,
        ))
    }

    #[wasm_bindgen(js_name = verifyStakeDelegation)]
    pub fn verify_stake_delegation(
        &self,
        signed_stake_delegation: SignedStakeDelegation,
    ) -> Result<bool, JsError> {
        self.client()
            .verify_stake_delegation(signed_stake_delegation)
    }

    #[wasm_bindgen(js_name = hashPayment)]
    pub fn hash_payment(&self, signed_payment: SignedPayment) -> Result<String, JsError> {
        self.client()
            .hash_signed_command_json(signed_payment.data().try_into()?)
    }

    #[wasm_bindgen(js_name = hashStakeDelegation)]
    pub fn hash_stake_delegation(
        &self,
        signed_stake_delegation: SignedStakeDelegation,
    ) -> Result<String, JsError> {
        self.client()
            .hash_signed_command_json(signed_stake_delegation.data().try_into()?)
    }

    #[wasm_bindgen(js_name = signedRosettaTransactionToSignedCommand)]
    pub fn signed_rosetta_transaction_to_signed_command(
        &self,
        signed_rosetta_transaction: String,
    ) -> Result<String, JsError> {
        let json = SignedCommandGraphQLJson {
            data: self
                .client()
                .signed_rosetta_transaction_to_signed_command(signed_rosetta_transaction)?,
        };
        serde_json::to_string(&json).map_err(map_js_err)
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
        let message: StringMessage = message.into();
        let mut ctx = mina_signer::create_legacy(self.network_id());
        ctx.sign(keypair, &message)
    }

    pub fn verify_message(&self, signed_message: SignedMessage) -> Result<bool, JsError> {
        let signature: MinaSignature = signed_message.signature().signature().try_into()?;
        let data = signed_message.data();
        let public_key = PubKey::from_address(data.public_key().as_str()).map_err(map_js_err)?;
        let payload: StringMessage = data.message().into();
        let mut ctx = mina_signer::create_legacy(self.network_id());
        Ok(ctx.verify(&signature, &public_key, &payload))
    }

    pub fn sign_payment(&self, payment: &MinaPayment, keypair: &MinaKeypair) -> MinaSignature {
        let mut ctx = mina_signer::create_legacy(self.network_id());
        ctx.sign(keypair, payment)
    }

    pub fn verify_payment(&self, signed_payment: SignedPayment) -> Result<bool, JsError> {
        let signature: MinaSignature = signed_payment.signature().try_into()?;
        let payment = signed_payment.data();
        let public_key = PubKey::from_address(payment.from().as_str()).map_err(map_js_err)?;
        let payload: MinaPayment = payment.try_into()?;
        let mut ctx = mina_signer::create_legacy(self.network_id());
        Ok(ctx.verify(&signature, &public_key, &payload))
    }

    pub fn sign_stake_delegation(
        &self,
        stake_delegation: &MinaStakeDelegation,
        keypair: &MinaKeypair,
    ) -> MinaSignature {
        let mut ctx = mina_signer::create_legacy(self.network_id());
        ctx.sign(keypair, stake_delegation)
    }

    pub fn verify_stake_delegation(
        &self,
        signed_stake_delegation: SignedStakeDelegation,
    ) -> Result<bool, JsError> {
        let signature: MinaSignature = signed_stake_delegation.signature().try_into()?;
        let stake_delegation = signed_stake_delegation.data();
        let public_key =
            PubKey::from_address(stake_delegation.from().as_str()).map_err(map_js_err)?;
        let payload: MinaStakeDelegation = stake_delegation.try_into()?;
        let mut ctx = mina_signer::create_legacy(self.network_id());
        Ok(ctx.verify(&signature, &public_key, &payload))
    }

    pub fn hash_signed_command_json(
        &self,
        signed_command_json: SignedCommandJson,
    ) -> Result<String, JsError> {
        let v1: SignedCommandV1 = signed_command_json.into();
        let mut binprot_bytes = Vec::new();
        bin_prot::to_writer(&mut binprot_bytes, &v1).map_err(map_js_err)?;
        let binprot_bytes_bs58 = bs58::encode(&binprot_bytes[..])
            .with_check_version(0x13)
            .into_string();
        let mut hasher = blake2::Blake2bVar::new(32).unwrap();
        hasher.write_all(binprot_bytes_bs58.as_bytes()).unwrap();
        let mut hash = hasher.finalize_boxed().to_vec();
        hash.insert(0, hash.len() as u8);
        hash.insert(0, 1);
        Ok(bs58::encode(hash).with_check_version(0x12).into_string())
    }

    pub fn signed_rosetta_transaction_to_signed_command(
        &self,
        signed_rosetta_transaction: String,
    ) -> Result<SignedCommandJson, JsError> {
        if let Some(signed_rosetta_transaction) =
            signed_rosetta_transaction_from_str(signed_rosetta_transaction)
        {
            let signature = signed_rosetta_transaction.signature();
            let mut sig_field_bytes =
                hex::decode(&signature[..(signature.len() / 2)]).map_err(map_js_err)?;
            sig_field_bytes.reverse();
            let rx = <CurvePoint as AffineCurve>::BaseField::from_bytes(&sig_field_bytes[..])
                .map_err(map_js_err)?;
            let mut sig_scalar_bytes =
                hex::decode(&signature[(signature.len() / 2)..]).map_err(map_js_err)?;
            sig_scalar_bytes.reverse();
            let s = <CurvePoint as AffineCurve>::ScalarField::from_bytes(&sig_scalar_bytes[..])
                .map_err(map_js_err)?;
            let signature = MinaSignature { rx, s };
            let mut cmd: SignedCommandJson = if let Some(payment) =
                signed_rosetta_transaction.payment()
            {
                payment.try_into()?
            } else if let Some(stake_delegation) = signed_rosetta_transaction.stake_delegation() {
                stake_delegation.try_into()?
            } else {
                return Err(JsError::new(
                    "Either payment or stake_delegation should be set",
                ));
            };
            cmd.signature = signature_to_json(signature);
            Ok(cmd)
        } else {
            Err(JsError::new("Failed"))
        }
    }

    fn network_id(&self) -> NetworkId {
        match self.network.as_str() {
            "mainnet" => NetworkId::MAINNET,
            _ => NetworkId::TESTNET,
        }
    }
}
