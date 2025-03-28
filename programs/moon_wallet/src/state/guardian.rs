use anchor_lang::prelude::*;

#[account]
pub struct Guardian {
    pub wallet: Pubkey,              
    pub pubkey: Pubkey,              
    pub name: String,                
    pub is_active: bool,             
    pub recovery_hash: [u8; 32],     
    pub bump: u8,                    
}
