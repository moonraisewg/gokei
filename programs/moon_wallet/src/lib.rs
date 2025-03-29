use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

use instructions::*;
use state::wallet::ActionParams;


declare_id!("DSteVhVB7YDw4UMRPid2y4rNqZyEyfHPMp6jWvFyQqjw");

#[program]
pub mod moon_wallet_program {
    use super::*;

    pub fn initialize_multisig(
        ctx: Context<InitializeMultisig>, 
        threshold: u8,
    ) -> Result<()> {
        instructions::wallet::initialize_multisig(ctx, threshold)
    }

    pub fn add_guardian(
        ctx: Context<AddGuardian>,
        guardian_pubkey: Pubkey,
        guardian_name: String,
        recovery_hash_intermediate: [u8; 32],
        is_owner: bool,
        webauthn_pubkey: Option<[u8; 33]>
    ) -> Result<()> {
        instructions::guardian::add_guardian(ctx, guardian_pubkey, guardian_name, recovery_hash_intermediate, is_owner, webauthn_pubkey)
    }

    pub fn remove_guardian(ctx: Context<RemoveGuardian>) -> Result<()> {
        instructions::guardian::remove_guardian(ctx)
    }

    pub fn update_guardian_status(ctx: Context<UpdateGuardianStatus>, is_active: bool) -> Result<()> {
        instructions::guardian::update_guardian_status(ctx, is_active)
    }

    pub fn recover_access_by_guardian(
        ctx: Context<RecoverAccessByGuardian>,
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
}

