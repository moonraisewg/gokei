use anchor_lang::prelude::*;

// Cấu trúc dữ liệu cho bảo mật - Tối ưu hóa
#[account]
pub struct Security {
    pub wallet: Pubkey,              
    pub password_hash: [u8; 32],     
    pub bump: u8,                    
}