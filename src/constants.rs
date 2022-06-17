pub const PRIVATE_KEY_BASE58_CHECK_VERSION_BYTE: u8 = 0x5a;

pub const PUBLIC_KEY_BASE58_CHECK_VERSION_BYTE: u8 = 0xcb;

pub const MEMO_BYTES: usize = 34;

const TAG_BITS: usize = 3;

pub const PAYMENT_TX_TAG: [bool; TAG_BITS] = [false, false, false];

pub const DELEGATION_TX_TAG: [bool; TAG_BITS] = [false, false, true];
