use anchor_lang::prelude::*;


#[account]
pub struct MultiSigWallet {
    pub threshold: u8,               
    pub guardian_count: u8,          
    pub recovery_nonce: u64,         
    pub bump: u8,                    
    pub transaction_nonce: u64,      
    pub last_transaction_timestamp: i64, 
    pub owner: Pubkey,               // Public key của người tạo ví
    pub credential_id: String,       // Credential ID từ WebAuthn, dùng làm seed
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ActionParams {
    pub amount: Option<u64>,        
    pub destination: Option<Pubkey>, 
    pub token_mint: Option<Pubkey>, 
}