use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use sha2::{Sha256, Digest};

// Context cho việc thêm guardian
#[derive(Accounts)]
pub struct AddGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        init,
        payer = owner,
        // Thêm 32 bytes cho recovery_hash
        space = 8 + 32 + 32 + 4 + 16 + 1 + 32 + 1, 
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
    recovery_hash_intermediate: [u8; 32], // hash secp256r1 từ frontend
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

    // Hash SHA-256 onchain trên kết quả hash secp256r1 từ frontend
    let mut hasher = Sha256::new();
    hasher.update(recovery_hash_intermediate);
    let final_hash: [u8; 32] = hasher.finalize().into();

    // Khởi tạo dữ liệu guardian
    guardian.wallet = multisig.key();
    guardian.pubkey = guardian_pubkey;
    guardian.name = guardian_name;
    guardian.is_active = true;
    guardian.recovery_hash = final_hash; // Lưu hash recovery key
    guardian.bump = ctx.bumps.guardian;

    // Cập nhật số lượng guardian
    multisig.guardian_count += 1;
    
    // Nếu đây là guardian đầu tiên, hãy cập nhật recovery hash cho wallet
    if multisig.guardian_count == 1 {
        multisig.recovery_hash = final_hash;
    }

    msg!("Guardian đã được thêm với recovery hash");
    Ok(())
}

// Thêm context cho việc xóa guardian
#[derive(Accounts)]
pub struct RemoveGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
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
        seeds = [b"multisig", multisig.credential_id.as_slice()],
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

// Context cho việc khôi phục quyền truy cập bằng Guardian Recovery Hash
#[derive(Accounts)]
pub struct RecoverAccessByGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        seeds = [b"guardian", multisig.key().as_ref(), guardian_pubkey.key().as_ref()],
        bump = guardian.bump
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Chỉ sử dụng để tìm account
    pub guardian_pubkey: AccountInfo<'info>,
    
    /// CHECK: Đây là pubkey của chủ sở hữu mới sau khi khôi phục
    pub new_owner: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

// Hàm khôi phục quyền truy cập bằng Guardian Recovery Hash
pub fn recover_access_by_guardian(
    ctx: Context<RecoverAccessByGuardian>,
    recovery_hash_intermediate: [u8; 32],
    new_webauthn_pubkey: [u8; 65],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let guardian = &ctx.accounts.guardian;
    let new_owner = &ctx.accounts.new_owner;
    
    // Kiểm tra xem guardian có active không
    require!(guardian.is_active, WalletError::GuardianError);
    
    // Hash SHA-256 onchain trên kết quả hash secp256r1 từ frontend
    let mut hasher = Sha256::new();
    hasher.update(recovery_hash_intermediate);
    let final_hash: [u8; 32] = hasher.finalize().into();
    
    // Xác minh hash đã được tính toán khớp với hash của guardian
    require!(guardian.recovery_hash == final_hash, WalletError::InvalidRecoveryKey);
    
    // Cập nhật thông tin owner mới
    multisig.owner = new_owner.key();
    
    // Cập nhật WebAuthn mới
    multisig.webauthn_pubkey = new_webauthn_pubkey;
    multisig.has_webauthn = true;
    
    // Tăng nonce để tránh replay attack
    multisig.recovery_nonce += 1;
    
    msg!("Quyền truy cập đã được khôi phục thành công bằng Guardian Recovery");
    Ok(())
}