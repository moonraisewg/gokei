use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

use instructions::*;
use state::*;
use errors::*;

declare_id!("C9AbkZSb8ugQep3oQu7qthuzHb2oSHrxcxwAdpQcRMmM");

#[program]
pub mod moon_wallet_program {
    use super::*;

    // Khởi tạo ví MultiSign
    pub fn initialize_multisig(
        ctx: Context<InitializeMultisig>,
        name: String,
        threshold: u8,
    ) -> Result<()> {
        instructions::wallet::initialize_multisig(ctx, name, threshold)
    }

    // Cấu hình WebAuthn cho ví
    pub fn configure_webauthn(
        ctx: Context<ConfigureWebAuthn>,
        webauthn_pubkey: [u8; 32],
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
}

#[derive(Accounts)]
pub struct Initialize {}
