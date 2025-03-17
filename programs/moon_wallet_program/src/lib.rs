use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

use instructions::*;


declare_id!("8z1cp83P8qTvmQzrF6PJfjq565MnKcpKkk2EUe4g574C");

#[program]
pub mod moon_wallet_program {
    use super::*;

    // Đảm bảo định nghĩa này khớp với IDL đã deploy
    pub fn initialize_multisig(
        ctx: Context<InitializeMultisig>, 
        threshold: u8,
        recovery_hash: [u8; 32]
    ) -> Result<()> {
        instructions::wallet::initialize_multisig(ctx, threshold, recovery_hash)
    }

    // Cấu hình WebAuthn cho ví
    pub fn configure_webauthn(
        ctx: Context<ConfigureWebAuthn>,
        webauthn_pubkey: [u8; 65],
    ) -> Result<()> {
        instructions::wallet::configure_webauthn(ctx, webauthn_pubkey)
    }

    // Thêm guardian vào ví
    pub fn add_guardian(
        ctx: Context<AddGuardian>,
        guardian_pubkey: Pubkey,
        guardian_name: String,
    ) -> Result<()> {
        instructions::guardian::add_guardian(ctx, guardian_pubkey, guardian_name)
    }

    // Lưu trữ hash mật khẩu
    pub fn store_password_hash(
        ctx: Context<StorePasswordHash>,
        password_hash: [u8; 32],
    ) -> Result<()> {
        instructions::security::store_password_hash(ctx, password_hash)
    }

    pub fn remove_guardian(ctx: Context<RemoveGuardian>) -> Result<()> {
        instructions::guardian::remove_guardian(ctx)
    }

    pub fn update_guardian_status(ctx: Context<UpdateGuardianStatus>, is_active: bool) -> Result<()> {
        instructions::guardian::update_guardian_status(ctx, is_active)
    }

    // Thêm API mới cho recovery key
    
    // Lưu hash recovery key và salt
    pub fn store_recovery_hash(
        ctx: Context<StoreRecoveryHash>,
        recovery_hash_intermediate: [u8; 32], // hash secp256r1 từ frontend
        recovery_salt: [u8; 16],
    ) -> Result<()> {
        instructions::wallet::store_recovery_hash(ctx, recovery_hash_intermediate, recovery_salt)
    }

    // Khôi phục quyền truy cập
    pub fn recover_access(
        ctx: Context<RecoverAccess>,
        recovery_hash_intermediate: [u8; 32],
        new_webauthn_pubkey: [u8; 65],
    ) -> Result<()> {
        instructions::wallet::recover_access(ctx, recovery_hash_intermediate, new_webauthn_pubkey)
    }
}

#[derive(Accounts)]
pub struct Initialize {}