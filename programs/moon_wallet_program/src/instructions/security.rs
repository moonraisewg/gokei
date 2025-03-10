use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

// Context cho việc lưu trữ hash mật khẩu
#[derive(Accounts)]
pub struct StorePasswordHash<'info> {
    #[account(
        mut,
        seeds = [b"multisig", owner.key().as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 1, // Giảm từ 8 + 32 + 32 + 8 + 8 + 1
        seeds = [b"security", multisig.key().as_ref()],
        bump
    )]
    pub security: Account<'info, Security>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Hàm lưu trữ hash mật khẩu
pub fn store_password_hash(
    ctx: Context<StorePasswordHash>,
    password_hash: [u8; 32],
) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let security = &mut ctx.accounts.security;
    let owner = &ctx.accounts.owner;

    // Kiểm tra quyền sở hữu
    require!(multisig.owner == owner.key(), WalletError::InvalidOperation);

    // Cập nhật thông tin bảo mật
    security.wallet = multisig.key();
    security.password_hash = password_hash;
    // Loại bỏ security.created_at = current_time;
    // Loại bỏ security.last_updated = current_time;
    security.bump = ctx.bumps.security;

    msg!("Hash mật khẩu đã được lưu trữ");
    Ok(())
}