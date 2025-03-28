use anchor_lang::prelude::*;


#[account]
pub struct MultiSigWallet {
    pub owner: Pubkey,               
    pub threshold: u8,               
    pub has_webauthn: bool,          
    pub webauthn_pubkey: [u8; 33],   
    pub credential_id: Vec<u8>,      
    pub guardian_count: u8,          
    pub recovery_hash: [u8; 32],     
    pub recovery_salt: [u8; 16],     
    pub recovery_nonce: u64,         
    pub bump: u8,                    
    pub transaction_nonce: u64,      
    pub last_transaction_timestamp: i64, 
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ActionParams {
    pub amount: Option<u64>,        
    pub destination: Option<Pubkey>, 
    pub token_mint: Option<Pubkey>, 
}