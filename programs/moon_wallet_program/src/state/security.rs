use anchor_lang::prelude::*;

// Cấu trúc dữ liệu cho bảo mật - Tối ưu hóa
#[account]
pub struct Security {
    pub wallet: Pubkey,              // 32 bytes
    pub password_hash: [u8; 32],     // 32 bytes
    pub bump: u8,                    // 1 byte
}