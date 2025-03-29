use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use anchor_lang::solana_program::hash::hash;

// Context cho việc thêm guardian
#[derive(Accounts)]
pub struct AddGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig".as_ref(), b"seed_for_pda".as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + 
               32 +  // wallet
               32 +  // pubkey
               4 + 32 + // name (string có max 32 bytes)
               1 +   // is_active
               32 +  // recovery_hash
               1 +   // is_owner
               1 + 33 + // Optional webauthn_pubkey (1 byte discriminator + 33 bytes)
               1,    // bump
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), guardian_pubkey.key().as_ref()],
        bump
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Chỉ lưu trữ public key
    pub guardian_pubkey: AccountInfo<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Hàm thêm guardian
pub fn add_guardian(
    ctx: Context<AddGuardian>,
    guardian_pubkey: Pubkey,
    guardian_name: String,
    recovery_hash_intermediate: [u8; 32],
    is_owner: bool,
    webauthn_pubkey: Option<[u8; 33]>,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let guardian = &mut ctx.accounts.guardian;
    
    require!(multisig.guardian_count < 8, WalletError::LimitExceeded);
    require!(guardian_name.len() <= 32, WalletError::NameTooLong);
    
    // Xác thực thêm: chỉ có thể có một guardian là owner
    if is_owner {
        // Kiểm tra xem đã có guardian nào là owner chưa
        // (Cần thêm logic để kiểm tra nếu đã có guardian là owner)
        // ...
    }
    
    // Nếu là owner, phải cung cấp webauthn_pubkey
    if is_owner {
        require!(webauthn_pubkey.is_some(), WalletError::WebAuthnNotConfigured);
    }
    
    let hash_result = hash(&recovery_hash_intermediate);
    let final_hash: [u8; 32] = hash_result.to_bytes();

    guardian.wallet = multisig.key();
    guardian.guardian_id = guardian_pubkey;
    guardian.name = guardian_name;
    guardian.is_active = true;
    guardian.recovery_hash = final_hash; 
    guardian.is_owner = is_owner;
    guardian.webauthn_pubkey = webauthn_pubkey;
    guardian.bump = ctx.bumps.guardian;

    multisig.guardian_count += 1;
    
    msg!("Guardian đã được thêm thành công");
    if is_owner {
        msg!("Guardian này là owner của ví");
    }
    
    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct RemoveGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig".as_ref(), b"seed_for_pda".as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), guardian_pubkey.key().as_ref()],
        bump = guardian.bump,
        close = owner
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: find account
    pub guardian_pubkey: AccountInfo<'info>,
    
    /// Yêu cầu phải được ký bởi một guardian có quyền owner
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), owner.key().as_ref()],
        constraint = owner_guardian.is_owner == true,
        bump = owner_guardian.bump
    )]
    pub owner_guardian: Account<'info, Guardian>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn remove_guardian(ctx: Context<RemoveGuardian>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    
    // Chỉ kiểm tra guardian_count, không kiểm tra owner nữa vì nay nó nằm trong guardian
    require!(multisig.guardian_count > 0, WalletError::GuardianError);
    multisig.guardian_count -= 1;

    msg!("Guardian đã được xóa");
    Ok(())
}

// Thêm context cho việc cập nhật trạng thái guardian
#[derive(Accounts)]
pub struct UpdateGuardianStatus<'info> {
    #[account(
        seeds = [b"multisig".as_ref(), b"seed_for_pda".as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), guardian_pubkey.key().as_ref()],
        bump = guardian.bump
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Chỉ sử dụng để tìm account
    pub guardian_pubkey: AccountInfo<'info>,
    
    /// Tài khoản guardian của người gọi, phải là owner
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), owner.key().as_ref()],
        constraint = owner_guardian.is_owner == true,
        bump = owner_guardian.bump
    )]
    pub owner_guardian: Account<'info, Guardian>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}

// Hàm cập nhật trạng thái guardian
pub fn update_guardian_status(
    ctx: Context<UpdateGuardianStatus>,
    is_active: bool,
) -> Result<()> {
    let guardian = &mut ctx.accounts.guardian;
    
    // Kiểm tra xem người gọi có phải là guardian owner không
    // Thực hiện điều này bằng cách yêu cầu tài khoản owner_guardian trong struct Context
    require!(ctx.accounts.owner_guardian.is_owner, WalletError::InvalidOperation);
    
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
        seeds = [b"multisig".as_ref(), b"seed_for_pda".as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    /// Guardian cũ (không phải owner nữa)
    #[account(
        mut,
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), old_guardian_pubkey.key().as_ref()],
        bump = old_guardian.bump,
        constraint = old_guardian.is_owner == true
    )]
    pub old_guardian: Account<'info, Guardian>,
    
    /// CHECK: Chỉ sử dụng để tìm account
    pub old_guardian_pubkey: AccountInfo<'info>,
    
    /// Guardian mới (sẽ trở thành owner)
    #[account(
        mut,
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), new_guardian_pubkey.key().as_ref()],
        bump = new_guardian.bump
    )]
    pub new_guardian: Account<'info, Guardian>,
    
    /// CHECK: Chỉ sử dụng để tìm account
    pub new_guardian_pubkey: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

// Hàm khôi phục quyền truy cập bằng Guardian Recovery Hash
pub fn recover_access_by_guardian(
    ctx: Context<RecoverAccessByGuardian>,
    recovery_hash_intermediate: [u8; 32],
    new_webauthn_pubkey: [u8; 33],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let old_guardian = &mut ctx.accounts.old_guardian;
    let new_guardian = &mut ctx.accounts.new_guardian;
    
    // Xác minh hash khôi phục cho old_guardian
    require!(old_guardian.wallet == multisig.key(), WalletError::InvalidGuardian);
    require!(old_guardian.is_active, WalletError::InactiveGuardian);
    
    // Xác minh hash khôi phục cho guardian cũ
    let hash_result = hash(&recovery_hash_intermediate);
    let final_hash: [u8; 32] = hash_result.to_bytes();
    
    require!(old_guardian.recovery_hash == final_hash, WalletError::InvalidRecoveryKey);
    
    // Chuyển quyền owner từ guardian cũ sang guardian mới
    old_guardian.is_owner = false;
    old_guardian.webauthn_pubkey = None;
    
    new_guardian.is_owner = true;
    new_guardian.webauthn_pubkey = Some(new_webauthn_pubkey);
    
    // Tăng nonce để ngăn chặn tấn công phát lại
    multisig.recovery_nonce += 1;
    
    msg!("Quyền truy cập đã được khôi phục thành công thông qua guardian");
    Ok(())
}