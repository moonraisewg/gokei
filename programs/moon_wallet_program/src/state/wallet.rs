use anchor_lang::prelude::*;

// Cấu trúc dữ liệu cho ví MultiSign - Cập nhật với salt và nonce
#[account]
pub struct MultiSigWallet {
    pub owner: Pubkey,               // 32 bytes
    pub threshold: u8,               // 1 byte
    pub has_webauthn: bool,          // 1 byte
    pub webauthn_pubkey: [u8; 65],   // 65 bytes
    pub guardian_count: u8,          // 1 byte
    pub recovery_hash: [u8; 32],     // 32 bytes - hash SHA-256 của hash secp256r1
    pub recovery_salt: [u8; 16],     // 16 bytes - salt ngẫu nhiên
    pub recovery_nonce: u64,         // 8 bytes - nonce tránh replay attack
    pub bump: u8,                    // 1 byte
}