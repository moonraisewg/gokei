use anchor_lang::prelude::*;

#[account]
pub struct Guardian {
    pub wallet: Pubkey,              // 32 bytes
    pub pubkey: Pubkey,              // 32 bytes
    pub name: String,                // Giảm max_length xuống 8 ký tự
    pub is_active: bool,             // 1 byte
    pub bump: u8,                    // 1 byte
}