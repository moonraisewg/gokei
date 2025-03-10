use anchor_lang::prelude::*;

// Cấu trúc dữ liệu cho ví MultiSign - Tối ưu hóa
#[account]
pub struct MultiSigWallet {
    pub owner: Pubkey,               // 32 bytes
    pub name: String,                // Giảm max_length xuống 16 ký tự
    pub threshold: u8,               // 1 byte
    pub has_webauthn: bool,          // 1 byte
    pub webauthn_pubkey: [u8; 32],   // 32 bytes
    pub guardian_count: u8,          // 1 byte
    pub recovery_hash: [u8; 32],     // 32 bytes
    pub bump: u8,                    // 1 byte
}