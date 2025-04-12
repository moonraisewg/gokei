use anchor_lang::prelude::*;
use crate::state::wallet::ActionParams;

#[account]
pub struct TransactionProposal {
    pub multisig: Pubkey,             
    pub proposal_id: u64,             
    pub proposer: Pubkey,             
    pub description: String,          
    pub action: String,               
    pub params: ActionParams,         
    pub status: ProposalStatus,       
    pub created_at: i64,              
    pub executed_at: Option<i64>,     
    pub signatures_count: u8,         
    pub required_signatures: u8,      
    pub bump: u8,                     
}

#[account]
pub struct ProposalSignature {
    pub proposal: Pubkey,             
    pub guardian_id: u64,             
    pub signature_time: i64,          
    pub bump: u8,                     
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProposalStatus {
    Pending,        
    Executed,       
    Rejected,       
    Expired,        
} 