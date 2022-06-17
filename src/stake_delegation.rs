use crate::*;
use mina_hasher::{Hashable, ROInput};
use mina_signer::{CompressedPubKey, NetworkId};

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
        let mut memo = [0; constants::MEMO_BYTES];
        if let Some(s) = self.memo() {
            for (i, &b) in s.as_bytes().iter().enumerate() {
                memo[i] = b;
            }
        }

        Ok(MinaStakeDelegation {
            to: CompressedPubKey::from_address(self.to().as_str()).map_err(map_js_err)?,
            from: CompressedPubKey::from_address(self.from().as_str()).map_err(map_js_err)?,
            fee: self.fee().as_f64().unwrap_or_default() as u64,
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
