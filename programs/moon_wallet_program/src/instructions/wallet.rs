use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

// Context cho việc khởi tạo ví MultiSign
#[derive(Accounts)]
pub struct InitializeMultisig<'info> {
    #[account(
        init,
        payer = fee_payer,
        space = 8 + // discriminator
               32 + // owner: Pubkey
               4 + 32 + // name: String (tối đa 32 ký tự)
               1 + // threshold: u8
               1 + // has_webauthn: bool
               32 + // webauthn_pubkey: [u8; 32]
               1 + // guardian_count: u8
               32 + // recovery_hash: [u8; 32]
               1, // bump: u8
        seeds = [b"multisig", owner.key().as_ref()],
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
        seeds = [b"multisig", owner.key().as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    pub owner: Signer<'info>,
}

// Hàm khởi tạo ví MultiSign
pub fn initialize_multisig(
    ctx: Context<InitializeMultisig>,
    name: String,
    threshold: u8,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    
    require!(threshold > 0 && name.len() <= 16, WalletError::InvalidConfig);
    
    multisig.owner = ctx.accounts.owner.key();
    multisig.name = name;
    multisig.threshold = threshold;
    multisig.bump = ctx.bumps.multisig;
    // Các trường khác mặc định là 0/false

    Ok(())
}

// Hàm cấu hình WebAuthn
pub fn configure_webauthn(
    ctx: Context<ConfigureWebAuthn>,
    webauthn_pubkey: [u8; 32],
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
        seeds = [b"multisig", owner.key().as_ref()],
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
        seeds = [b"multisig", multisig.owner.as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    /// CHECK: Người dùng mới sẽ trở thành owner
    pub new_owner: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

// Hàm lưu hash recovery key
pub fn store_recovery_hash(
    ctx: Context<StoreRecoveryHash>,
    recovery_hash: [u8; 32],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let owner = &ctx.accounts.owner;

    // Kiểm tra quyền sở hữu
    require!(multisig.owner == owner.key(), WalletError::InvalidOperation);

    // Lưu hash recovery key đã được tính toán ở frontend
    multisig.recovery_hash = recovery_hash;

    msg!("Recovery hash đã được lưu trữ");
    Ok(())
}

// Hàm khôi phục quyền truy cập
pub fn recover_access(
    ctx: Context<RecoverAccess>,
    recovery_hash: [u8; 32],  // Đã được hash ở frontend
    new_webauthn_pubkey: [u8; 32],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let new_owner = &ctx.accounts.new_owner;
    
    // Xác minh hash đã được tính toán ở frontend
    require!(multisig.recovery_hash == recovery_hash, WalletError::InvalidRecoveryKey);
    
    // Cập nhật thông tin owner mới
    multisig.owner = new_owner.key();
    
    // Cập nhật WebAuthn mới
    multisig.webauthn_pubkey = new_webauthn_pubkey;
    multisig.has_webauthn = true;
    
    msg!("Quyền truy cập đã được khôi phục thành công");
    Ok(())
}

// Loại bỏ hàm hash_recovery_key vì sẽ được xử lý ở frontend