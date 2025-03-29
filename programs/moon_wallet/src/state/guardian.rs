use anchor_lang::prelude::*;

#[account]
pub struct Guardian {
    pub wallet: Pubkey,              
    pub guardian_id: u64,       // Thay đổi từ Pubkey sang u64 để tiết kiệm kích thước     
    pub name: String,                
    pub is_active: bool,             
    pub recovery_hash: [u8; 32],     
    pub is_owner: bool,             
    pub webauthn_pubkey: Option<[u8; 33]>,
    pub bump: u8,                    
}
