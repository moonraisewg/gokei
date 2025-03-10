use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

// Context cho việc thêm guardian
#[derive(Accounts)]
pub struct AddGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig", owner.key().as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        init,
        payer = owner,
        // Tối ưu hơn: Giảm kích thước tên xuống tối đa 16 ký tự
        space = 8 + 32 + 32 + 4 + 16 + 1 + 1, 
        seeds = [b"guardian", multisig.key().as_ref(), guardian_pubkey.key().as_ref()],
        bump
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Chỉ lưu trữ public key
    pub guardian_pubkey: AccountInfo<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Hàm thêm guardian
pub fn add_guardian(
    ctx: Context<AddGuardian>,
    guardian_pubkey: Pubkey,
    guardian_name: String,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let guardian = &mut ctx.accounts.guardian;
    let owner = &ctx.accounts.owner;

    // Kiểm tra quyền sở hữu
    require!(multisig.owner == owner.key(), WalletError::InvalidOperation);
    
    // Kiểm tra số lượng guardian
    require!(multisig.guardian_count < 8, WalletError::LimitExceeded);
    
    // Giới hạn độ dài tên guardian
    require!(guardian_name.len() <= 32, WalletError::NameTooLong);

    // Khởi tạo dữ liệu guardian
    guardian.wallet = multisig.key();
    guardian.pubkey = guardian_pubkey;
    guardian.name = guardian_name;
    guardian.is_active = true;
    // Loại bỏ guardian.created_at = Clock::get()?.unix_timestamp;
    guardian.bump = ctx.bumps.guardian;

    // Cập nhật số lượng guardian
    multisig.guardian_count += 1;

    msg!("Guardian đã được thêm");
    Ok(())
}

// Thêm context cho việc xóa guardian
#[derive(Accounts)]
pub struct RemoveGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig", owner.key().as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [b"guardian", multisig.key().as_ref(), guardian_pubkey.key().as_ref()],
        bump = guardian.bump,
        close = owner
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Chỉ sử dụng để tìm account
    pub guardian_pubkey: AccountInfo<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Hàm xóa guardian
pub fn remove_guardian(ctx: Context<RemoveGuardian>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let owner = &ctx.accounts.owner;

    // Kiểm tra quyền sở hữu
    require!(multisig.owner == owner.key(), WalletError::InvalidOperation);
    
    // Giảm số lượng guardian
    require!(multisig.guardian_count > 0, WalletError::GuardianError);
    multisig.guardian_count -= 1;

    msg!("Guardian đã được xóa");
    Ok(())
}

// Thêm context cho việc cập nhật trạng thái guardian
#[derive(Accounts)]
pub struct UpdateGuardianStatus<'info> {
    #[account(
        seeds = [b"multisig", owner.key().as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [b"guardian", multisig.key().as_ref(), guardian_pubkey.key().as_ref()],
        bump = guardian.bump
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Chỉ sử dụng để tìm account
    pub guardian_pubkey: AccountInfo<'info>,
    
    pub owner: Signer<'info>,
}

// Hàm cập nhật trạng thái guardian
pub fn update_guardian_status(
    ctx: Context<UpdateGuardianStatus>,
    is_active: bool,
) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let guardian = &mut ctx.accounts.guardian;
    let owner = &ctx.accounts.owner;

    // Kiểm tra quyền sở hữu
    require!(multisig.owner == owner.key(), WalletError::InvalidOperation);
    
    // Cập nhật trạng thái
    guardian.is_active = is_active;

    msg!("Trạng thái guardian đã được cập nhật");
    Ok(())
}