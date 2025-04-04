use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use anchor_lang::solana_program::hash::hash;


use crate::instructions::wallet::process_credential_id_seed;


#[derive(Accounts)]
#[instruction(guardian_id: u64)]
pub struct AddGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig".as_ref(), &process_credential_id_seed(&multisig.credential_id)],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + 
               32 +  
               8 + 
               4 + 32 + 
               1 +  
               32 +  
               1 +   
               1 + 33 + 
               1,    
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &guardian_id.to_le_bytes()],
        bump
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Không còn cần thiết nhưng giữ lại để tương thích
    pub guardian_pubkey: AccountInfo<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn add_guardian(
    ctx: Context<AddGuardian>,
    guardian_id: u64,
    guardian_name: String,
    recovery_hash_intermediate: [u8; 32],
    is_owner: bool,
    webauthn_pubkey: Option<[u8; 33]>,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let guardian = &mut ctx.accounts.guardian;
    
    require!(multisig.guardian_count < 8, WalletError::LimitExceeded);
    require!(guardian_name.len() <= 32, WalletError::NameTooLong);
    
    
    if is_owner {
    
    }
    
    
    if is_owner {
        require!(webauthn_pubkey.is_some(), WalletError::WebAuthnNotConfigured);
    }
    
    let hash_result = hash(&recovery_hash_intermediate);
    let final_hash: [u8; 32] = hash_result.to_bytes();

    guardian.wallet = multisig.key();
    guardian.guardian_id = guardian_id;
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
#[instruction(guardian_id: u64, owner_guardian_id: u64)]
pub struct RemoveGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig".as_ref(), &process_credential_id_seed(&multisig.credential_id)],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &guardian_id.to_le_bytes()],
        bump = guardian.bump,
        close = owner
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Không còn cần thiết nhưng giữ lại để tương thích
    pub guardian_pubkey: AccountInfo<'info>,
    
   
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &owner_guardian_id.to_le_bytes()],
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
    
   
    require!(multisig.guardian_count > 0, WalletError::GuardianError);
    multisig.guardian_count -= 1;

    msg!("Guardian đã được xóa");
    Ok(())
}

// Thêm context cho việc cập nhật trạng thái guardian
#[derive(Accounts)]
#[instruction(guardian_id: u64, owner_guardian_id: u64, is_active: bool)]
pub struct UpdateGuardianStatus<'info> {
    #[account(
        mut,
        seeds = [b"multisig".as_ref(), &process_credential_id_seed(&multisig.credential_id)],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &guardian_id.to_le_bytes()],
        bump = guardian.bump
    )]
    pub guardian: Account<'info, Guardian>,
    
    /// CHECK: Không còn cần thiết nhưng giữ lại để tương thích
    pub guardian_pubkey: AccountInfo<'info>,
    
    /// Tài khoản guardian của người gọi, phải là owner
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &owner_guardian_id.to_le_bytes()],
        constraint = owner_guardian.is_owner == true,
        bump = owner_guardian.bump
    )]
    pub owner_guardian: Account<'info, Guardian>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}


pub fn update_guardian_status(
    ctx: Context<UpdateGuardianStatus>,
    is_active: bool,
) -> Result<()> {
    let guardian = &mut ctx.accounts.guardian;
    
  
    require!(ctx.accounts.owner_guardian.is_owner, WalletError::InvalidOperation);
    
    // Cập nhật trạng thái
    guardian.is_active = is_active;

    msg!("Trạng thái guardian đã được cập nhật");
    Ok(())
}

#[derive(Accounts)]
#[instruction(old_guardian_id: u64, new_guardian_id: u64, recovery_hash_intermediate: [u8; 32])]
pub struct RecoverAccessByGuardian<'info> {
    #[account(
        mut,
        seeds = [b"multisig".as_ref(), &process_credential_id_seed(&multisig.credential_id)],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    

    #[account(
        mut,
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &old_guardian_id.to_le_bytes()],
        bump = old_guardian.bump,
        constraint = old_guardian.is_owner == true
    )]
    pub old_guardian: Account<'info, Guardian>,
    
    /// CHECK: Không còn cần thiết nhưng giữ lại để tương thích
    pub old_guardian_pubkey: AccountInfo<'info>,
    

    #[account(
        mut,
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &new_guardian_id.to_le_bytes()],
        bump = new_guardian.bump
    )]
    pub new_guardian: Account<'info, Guardian>,
    
    /// CHECK: Không còn cần thiết nhưng giữ lại để tương thích
    pub new_guardian_pubkey: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}


pub fn recover_access_by_guardian(
    ctx: Context<RecoverAccessByGuardian>,
    recovery_hash_intermediate: [u8; 32],
    new_webauthn_pubkey: [u8; 33],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let old_guardian = &mut ctx.accounts.old_guardian;
    let new_guardian = &mut ctx.accounts.new_guardian;
    

    require!(old_guardian.wallet == multisig.key(), WalletError::InvalidGuardian);
    require!(old_guardian.is_active, WalletError::InactiveGuardian);
    
  
    let hash_result = hash(&recovery_hash_intermediate);
    let final_hash: [u8; 32] = hash_result.to_bytes();
    
    require!(old_guardian.recovery_hash == final_hash, WalletError::InvalidRecoveryKey);
    
    
    old_guardian.is_owner = false;
    old_guardian.webauthn_pubkey = None;
    
    new_guardian.is_owner = true;
    new_guardian.webauthn_pubkey = Some(new_webauthn_pubkey);
    

    multisig.recovery_nonce += 1;
    
    msg!("Quyền truy cập đã được khôi phục thành công thông qua guardian");
    Ok(())
}