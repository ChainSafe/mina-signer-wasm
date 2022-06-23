use crate::*;
use mina_hasher::{Hashable, ROInput};
use mina_serialization_types::{common::*, json::*};
use mina_signer::{CompressedPubKey, NetworkId};
use num_traits::identities::One;

#[wasm_bindgen]
extern "C" {
    pub type StakeDelegation;

    #[wasm_bindgen(method, getter)]
    pub fn to(this: &StakeDelegation) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn from(this: &StakeDelegation) -> String;

    // u64 can be either f64 or bigint in js,
    // use JsValue to handle the conversion
    #[wasm_bindgen(method, getter)]
    pub fn fee(this: &StakeDelegation) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn nonce(this: &StakeDelegation) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn memo(this: &StakeDelegation) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = validUntil)]
    pub fn valid_until(this: &StakeDelegation) -> Option<u32>;

    pub type SignedStakeDelegation;

    #[wasm_bindgen(method, getter)]
    pub fn signature(this: &SignedStakeDelegation) -> Signature;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &SignedStakeDelegation) -> StakeDelegation;
}

#[wasm_bindgen(inline_js = r#"
        function new_stake_delegation(to, from, fee, nonce, memo, validUntil) {
            return {
                to,
                from,
                fee,
                nonce,
                memo,
                validUntil,
            }
        }

        function new_signed_stake_delegation(signature, data) {
            return {
                signature,
                data,
            }
        }

        module.exports = {
            new_stake_delegation, new_signed_stake_delegation
        }
    "#)]
extern "C" {
    pub fn new_stake_delegation(
        to: String,
        from: String,
        fee: u64,
        nonce: u32,
        memo: Option<String>,
        valid_until: Option<u32>,
    ) -> StakeDelegation;

    pub fn new_signed_stake_delegation(
        signature: Signature,
        data: StakeDelegation,
    ) -> SignedStakeDelegation;
}

impl StakeDelegation {
    pub fn try_to_mina_stake_delegation(&self) -> Result<MinaStakeDelegation, JsError> {
        Ok(MinaStakeDelegation {
            to: CompressedPubKey::from_address(self.to().as_str()).map_err(map_js_err)?,
            from: CompressedPubKey::from_address(self.from().as_str()).map_err(map_js_err)?,
            fee: js_to_string(&self.fee()).parse().map_err(map_js_err)?,
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
pub struct MinaStakeDelegation {
    pub to: CompressedPubKey,
    pub from: CompressedPubKey,
    pub fee: u64,
    pub nonce: u32,
    pub memo: [u8; constants::MEMO_BYTES],
    pub valid_until: u32,
}

impl From<MinaStakeDelegation> for StakeDelegation {
    fn from(v: MinaStakeDelegation) -> Self {
        let mut memo_len = v.memo.len();
        while memo_len > 0 && v.memo[memo_len - 1] == 0 {
            memo_len -= 1;
        }
        let memo = if memo_len == 0 {
            None
        } else {
            Some(unsafe { String::from_utf8_unchecked(v.memo[..memo_len].into()) })
        };
        new_stake_delegation(
            v.to.into_address(),
            v.from.into_address(),
            v.fee,
            v.nonce,
            memo,
            Some(v.valid_until),
        )
    }
}

impl TryFrom<StakeDelegation> for MinaStakeDelegation {
    type Error = JsError;

    fn try_from(v: StakeDelegation) -> Result<Self, Self::Error> {
        v.try_to_mina_stake_delegation()
    }
}

impl TryFrom<StakeDelegation> for SignedCommandJson {
    type Error = JsError;

    fn try_from(v: StakeDelegation) -> Result<Self, Self::Error> {
        let p: MinaStakeDelegation = v.try_into()?;
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
                body: SignedCommandPayloadBodyJson::StakeDelegation(
                    StakeDelegationJson::SetDelegate {
                        delegator: compressed_pubkey_to_json(p.from),
                        new_delegate: compressed_pubkey_to_json(p.to),
                    },
                ),
            },
            signer: compressed_pubkey_to_json(p.from),
            signature: signature_to_json(dummy_signature),
        })
    }
}

impl TryFrom<SignedStakeDelegation> for SignedCommandJson {
    type Error = JsError;

    fn try_from(v: SignedStakeDelegation) -> Result<Self, Self::Error> {
        let mut result: Self = v.data().try_into()?;
        result.signature = signature_to_json(v.signature().try_into()?);
        Ok(result)
    }
}

impl Hashable for MinaStakeDelegation {
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

        for tag_bit in constants::DELEGATION_TX_TAG {
            roi.append_bool(tag_bit);
        }

        roi.append_bool(self.from.is_odd);
        roi.append_bool(self.to.is_odd);
        // token id
        roi.append_u64(1);
        // amount
        roi.append_u64(0);
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
