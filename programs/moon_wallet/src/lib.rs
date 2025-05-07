use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use state::wallet::ActionParams;

// Cập nhật version số
declare_id!("6Y3N5AQRQtviTrmRcf29yHWBu3ft7xEoj8kqmykDGvKP");

mod state;
mod instructions;
mod errors;

use instructions::*;

#[program]
pub mod moon_wallet_program {
    use super::*;

    pub fn initialize_multisig(
        ctx: Context<InitializeMultisig>, 
        threshold: u8,
        credential_id: String
    ) -> Result<()> {
        instructions::wallet::initialize_multisig(ctx, threshold, credential_id)
    }

    pub fn add_guardian(
        ctx: Context<AddGuardian>,
        guardian_id: u64,
        guardian_name: String,
        recovery_hash_intermediate: [u8; 32],
        is_owner: bool,
        webauthn_pubkey: Option<[u8; 33]>
    ) -> Result<()> {
        instructions::guardian::add_guardian(ctx, guardian_id, guardian_name, recovery_hash_intermediate, is_owner, webauthn_pubkey)
    }

    pub fn remove_guardian(
        ctx: Context<RemoveGuardian>,
        _guardian_id: u64,
        _owner_guardian_id: u64
    ) -> Result<()> {
        instructions::guardian::remove_guardian(ctx)
    }

    pub fn update_guardian_status(
        ctx: Context<UpdateGuardianStatus>, 
        _guardian_id: u64,
        _owner_guardian_id: u64,
        is_active: bool
    ) -> Result<()> {
        instructions::guardian::update_guardian_status(ctx, is_active)
    }

    pub fn recover_access_by_guardian(
        ctx: Context<RecoverAccessByGuardian>,
        _old_guardian_id: u64,
        _new_guardian_id: u64,
        recovery_hash_intermediate: [u8; 32],
        new_webauthn_pubkey: [u8; 33],
    ) -> Result<()> {
        instructions::guardian::recover_access_by_guardian(ctx, recovery_hash_intermediate, new_webauthn_pubkey)
    }

    pub fn verify_and_execute(
        ctx: Context<VerifyAndExecute>,
        action: String,
        params: ActionParams,
        nonce: u64,
        timestamp: i64,
        message: Vec<u8>
    ) -> Result<()> {
        instructions::wallet::verify_and_execute(ctx, action, params, nonce, timestamp, message)
    }
    
    // Chức năng mới: Tạo đề xuất giao dịch
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_id: u64,
        description: String,
        proposer_guardian_id: u64,
        action: String,
        params: ActionParams
    ) -> Result<()> {
        instructions::proposal::create_proposal(ctx, proposal_id, description, proposer_guardian_id, action, params)
    }
    
    // Chức năng mới: Phê duyệt đề xuất giao dịch
    pub fn approve_proposal(
        ctx: Context<ApproveProposal>,
        proposal_id: u64,
        guardian_id: u64,
        timestamp: i64,
        message: Vec<u8>
    ) -> Result<()> {
        instructions::proposal::approve_proposal(ctx, proposal_id, guardian_id, timestamp, message)
    }
    
    // Chức năng mới: Thực thi đề xuất giao dịch
    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
        proposal_id: u64
    ) -> Result<()> {
        instructions::proposal::execute_proposal(ctx, proposal_id)
    }
    
    // Chức năng mới: Từ chối đề xuất giao dịch
    pub fn reject_proposal(
        ctx: Context<RejectProposal>,
        proposal_id: u64,
        guardian_id: u64,
        timestamp: i64,
        message: Vec<u8>
    ) -> Result<()> {
        instructions::proposal::reject_proposal(ctx, proposal_id, guardian_id, timestamp, message)
    }
}

