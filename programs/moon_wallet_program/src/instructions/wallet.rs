use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use sha2::{Sha256, Digest};

// Context cho việc khởi tạo ví MultiSign
#[derive(Accounts)]
#[instruction(threshold: u8, recovery_hash: [u8; 32], credential_id: Vec<u8>)]
pub struct InitializeMultisig<'info> {
    #[account(
        init,
        payer = fee_payer,
        space = 8 + // discriminator
               32 + // owner: Pubkey
               1 +  // threshold: u8
               1 +  // has_webauthn: bool
               65 + // webauthn_pubkey: [u8; 65]
               4 + credential_id.len() + // credential_id (4 bytes cho chiều dài + dữ liệu)
               1 +  // guardian_count: u8
               32 + // recovery_hash: [u8; 32]
               16 + // recovery_salt: [u8; 16]
               8 +  // recovery_nonce: u64
               1,   // bump: u8
        seeds = [b"multisig", credential_id.as_slice()],
        bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    /// CHECK: Owner của multisig wallet
    pub owner: AccountInfo<'info>,
    
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Context cho việc cấu hình WebAuthn
#[derive(Accounts)]
pub struct ConfigureWebAuthn<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    pub owner: Signer<'info>,
}

// Hàm khởi tạo ví MultiSign
pub fn initialize_multisig(
    ctx: Context<InitializeMultisig>,
    threshold: u8,
    recovery_hash: [u8; 32],
    credential_id: Vec<u8>,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let owner = &ctx.accounts.owner;
    
    require!(threshold > 0, WalletError::InvalidConfig);
    
    // Lưu các trường thông tin
    multisig.owner = owner.key();
    multisig.threshold = threshold;
    multisig.has_webauthn = false;
    multisig.webauthn_pubkey = [0; 65];
    multisig.credential_id = credential_id;
    multisig.guardian_count = 0;
    multisig.recovery_hash = recovery_hash;
    multisig.recovery_salt = [0; 16];
    multisig.recovery_nonce = 0;
    multisig.bump = ctx.bumps.multisig;
    
    msg!("Đã khởi tạo ví MultiSign thành công");
    Ok(())
}

// Hàm cấu hình WebAuthn
pub fn configure_webauthn(
    ctx: Context<ConfigureWebAuthn>,
    webauthn_pubkey: [u8; 65],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    require!(multisig.owner == ctx.accounts.owner.key(), WalletError::InvalidOperation);

    multisig.webauthn_pubkey = webauthn_pubkey;
    multisig.has_webauthn = true;

    Ok(())
}

// Context cho việc lưu hash recovery key
#[derive(Accounts)]
pub struct StoreRecoveryHash<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    pub owner: Signer<'info>,
}

// Context cho việc khôi phục quyền truy cập
#[derive(Accounts)]
pub struct RecoverAccess<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    pub new_owner: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

// Hàm lưu hash recovery key
pub fn store_recovery_hash(
    ctx: Context<StoreRecoveryHash>,
    recovery_hash_intermediate: [u8; 32], // hash secp256r1 từ frontend
    recovery_salt: [u8; 16],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let owner = &ctx.accounts.owner;

    // Kiểm tra quyền sở hữu
    require!(multisig.owner == owner.key(), WalletError::InvalidOperation);

    // Hash SHA-256 onchain trên kết quả hash secp256r1 từ frontend
    let mut hasher = Sha256::new();
    hasher.update(recovery_hash_intermediate);
    let final_hash: [u8; 32] = hasher.finalize().into();

    // Lưu hash cuối cùng và salt
    multisig.recovery_hash = final_hash;
    multisig.recovery_salt = recovery_salt;
    multisig.recovery_nonce += 1;

    msg!("Recovery hash và salt đã được lưu trữ");
    Ok(())
}

// Hàm khôi phục quyền truy cập
pub fn recover_access(
    ctx: Context<RecoverAccess>,
    recovery_hash_intermediate: [u8; 32],
    new_webauthn_pubkey: [u8; 65],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let new_owner = &ctx.accounts.new_owner;
    
    // Hash SHA-256 onchain trên kết quả hash secp256r1 từ frontend
    let mut hasher = Sha256::new();
    hasher.update(recovery_hash_intermediate);
    let final_hash: [u8; 32] = hasher.finalize().into();
    
    // Xác minh hash đã được tính toán
    require!(multisig.recovery_hash == final_hash, WalletError::InvalidRecoveryKey);
    
    // Cập nhật thông tin owner mới
    multisig.owner = new_owner.key();
    
    // Cập nhật WebAuthn mới
    multisig.webauthn_pubkey = new_webauthn_pubkey;
    multisig.has_webauthn = true;
    
    // Tăng nonce để tránh replay attack
    multisig.recovery_nonce += 1;
    
    msg!("Quyền truy cập đã được khôi phục thành công");
    Ok(())
}

