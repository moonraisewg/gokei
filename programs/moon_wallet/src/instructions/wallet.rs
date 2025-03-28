use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_lang::solana_program;
use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
use crate::state::*;
use crate::errors::*;
use sha2::{Sha256, Digest};


#[derive(Accounts)]
#[instruction(threshold: u8, recovery_hash: [u8; 32], credential_id: Vec<u8>)]
pub struct InitializeMultisig<'info> {
    #[account(
        init,
        payer = fee_payer,
        space = 8 + 
               32 + 
               1 +  
               1 +  
               33 + 
               4 + credential_id.len() + 
               1 +  
               32 + 
               32 + 
               16 + 
               8 +  
               1 +  
               8 +  
               8,  
        seeds = [b"multisig", credential_id.as_slice()],
        bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    /// CHECK: Đây là địa chỉ chủ của ví, được sử dụng để lưu trữ và không cần validate
    pub owner: AccountInfo<'info>,
    
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}


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


pub fn initialize_multisig(
    ctx: Context<InitializeMultisig>,
    threshold: u8,
    recovery_hash: [u8; 32],
    credential_id: Vec<u8>,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let owner = &ctx.accounts.owner;
    
    require!(threshold > 0, WalletError::InvalidConfig);
    
    multisig.owner = owner.key();
    multisig.threshold = threshold;
    multisig.has_webauthn = false;
    multisig.webauthn_pubkey = [0; 33];
    multisig.credential_id = credential_id;
    multisig.guardian_count = 0;

    multisig.recovery_hash = recovery_hash;
    multisig.recovery_salt = [0; 16];
    multisig.recovery_nonce = 0;
    multisig.bump = ctx.bumps.multisig;
    
    multisig.transaction_nonce = 0;
    multisig.last_transaction_timestamp = 0;
    
    msg!("Đã khởi tạo ví MultiSign thành công");
    msg!("Hãy thêm owner làm guardian đầu tiên để thiết lập recovery hash");
    Ok(())
}


pub fn configure_webauthn(
    ctx: Context<ConfigureWebAuthn>,
    webauthn_pubkey: [u8; 33],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    require!(multisig.owner == ctx.accounts.owner.key(), WalletError::InvalidOperation);

    multisig.webauthn_pubkey = webauthn_pubkey;
    multisig.has_webauthn = true;

    Ok(())
}


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


#[derive(Accounts)]
pub struct RecoverAccess<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    /// CHECK: Đây là địa chỉ chủ mới sau khi khôi phục, sẽ được lưu vào ví
    pub new_owner: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}


pub fn store_recovery_hash(
    ctx: Context<StoreRecoveryHash>,
    recovery_hash_intermediate: [u8; 32], 
    recovery_salt: [u8; 16],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let owner = &ctx.accounts.owner;

    require!(multisig.owner == owner.key(), WalletError::InvalidOperation);

    let mut hasher = Sha256::new();
    hasher.update(recovery_hash_intermediate);
    let final_hash: [u8; 32] = hasher.finalize().into();

    multisig.recovery_hash = final_hash;
    multisig.recovery_salt = recovery_salt;
    multisig.recovery_nonce += 1;

    msg!("Recovery hash và salt đã được lưu trữ");
    Ok(())
}


pub fn recover_access(
    ctx: Context<RecoverAccess>,
    recovery_hash_intermediate: [u8; 32],
    new_webauthn_pubkey: [u8; 33],
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let new_owner = &ctx.accounts.new_owner;
    
    let mut hasher = Sha256::new();
    hasher.update(recovery_hash_intermediate);
    let final_hash: [u8; 32] = hasher.finalize().into();
    
    require!(multisig.recovery_hash == final_hash, WalletError::InvalidRecoveryKey);
    
    multisig.owner = new_owner.key();
    
    multisig.webauthn_pubkey = new_webauthn_pubkey;
    multisig.has_webauthn = true;
    
    multisig.recovery_nonce += 1;
    
    msg!("Quyền truy cập đã được khôi phục thành công");
    Ok(())
}


#[derive(Accounts)]
pub struct VerifyAndExecute<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    pub clock: Sysvar<'info, Clock>,
    
    /// CHECK: Đây là tài khoản sysvar chứa thông tin về các instruction trong transaction, được sử dụng để xác thực quá trình ký
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instruction_sysvar: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Đây là địa chỉ đích để gửi giao dịch, được kiểm tra trong logic thực thi
    #[account(mut)]
    pub destination: AccountInfo<'info>,
}


pub fn verify_and_execute(
    ctx: Context<VerifyAndExecute>,
    action: String,
    params: ActionParams,
    nonce: u64,
    timestamp: i64,
    message: Vec<u8>
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let clock = &ctx.accounts.clock;
    
    require!(
        nonce == multisig.transaction_nonce + 1,
        WalletError::InvalidNonce
    );
    
    require!(
        timestamp <= clock.unix_timestamp + 60, 
        WalletError::FutureTimestamp
    );
    
    require!(
        timestamp >= multisig.last_transaction_timestamp,
        WalletError::OutdatedTimestamp
    );
    
    require!(
        timestamp >= clock.unix_timestamp - 300,
        WalletError::ExpiredTimestamp
    );
    
    let instruction_sysvar = &ctx.accounts.instruction_sysvar;
    require!(
        !instruction_sysvar.data_is_empty(),
        WalletError::InstructionMissing
    );
    
    let secp_ix = load_instruction_at_checked(0, instruction_sysvar)?;
    
    let secp256r1_verify_id = Pubkey::from_str("Secp256r1SigVerify1111111111111111111111111").unwrap();
    
    require!(
        secp_ix.program_id == secp256r1_verify_id,
        WalletError::InvalidSignatureVerification
    );
    
    let pk_in_ix = extract_public_key_from_secp_instruction(&secp_ix.data)?;
    require!(
        pk_in_ix == multisig.webauthn_pubkey,
        WalletError::PublicKeyMismatch
    );
    
    let msg_in_ix = extract_message_from_secp_instruction(&secp_ix.data)?;
    require!(
        msg_in_ix == message,
        WalletError::MessageMismatch
    );
    
    let mut expected_message = String::new();
   
    match action.as_str() {
        "transfer" => {
            let amount = params.amount.ok_or(WalletError::InvalidOperation)?;
            let destination = params.destination.ok_or(WalletError::InvalidOperation)?;
            
            expected_message = format!(
                "transfer:{}_SOL_to_{},nonce:{},timestamp:{}",
                amount as f64 / 1_000_000_000.0, 
                destination.to_string(),
                nonce,
                timestamp
            );
        },
        _ => return Err(WalletError::UnsupportedAction.into())
    }
    
    require!(
        message == expected_message.as_bytes(),
        WalletError::MessageMismatch
    );
    
    multisig.transaction_nonce = nonce;
    multisig.last_transaction_timestamp = timestamp;
    
    match action.as_str() {
        "transfer" => execute_transfer(ctx, &params),
        _ => Err(WalletError::UnsupportedAction.into())
    }
}


fn extract_public_key_from_secp_instruction(data: &[u8]) -> Result<[u8; 33]> {
    if data.len() < 16 {
        return Err(WalletError::InvalidInstructionData.into());
    }
    
    let num_signatures = data[0] as usize;
    if num_signatures != 1 {
        return Err(WalletError::InvalidSignatureCount.into());
    }
    
    let public_key_offset = u16::from_le_bytes([data[6], data[7]]) as usize;
    
    let mut pk = [0u8; 33];
    if data.len() < public_key_offset + 33 {
        return Err(WalletError::InvalidInstructionData.into());
    }
    
    pk.copy_from_slice(&data[public_key_offset..public_key_offset + 33]);
    Ok(pk)
}


fn extract_message_from_secp_instruction(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 16 {
        return Err(WalletError::InvalidInstructionData.into());
    }
    
    let message_offset = u16::from_le_bytes([data[10], data[11]]) as usize;
    let message_size = u16::from_le_bytes([data[12], data[13]]) as usize;
    
    if data.len() < message_offset + message_size {
        return Err(WalletError::InvalidInstructionData.into());
    }
    
    Ok(data[message_offset..message_offset + message_size].to_vec())
}


fn execute_transfer(ctx: Context<VerifyAndExecute>, params: &ActionParams) -> Result<()> {
    let amount = params.amount.ok_or(WalletError::InvalidOperation)?;
    
    let destination = ctx.accounts.destination.key();
    
    let params_destination = params.destination.ok_or(WalletError::InvalidOperation)?;
    require!(
        params_destination == destination,
        WalletError::InvalidOperation
    );
    
    msg!("Thực hiện chuyển {} SOL đến {}", amount as f64 / 1_000_000_000.0, destination);
    
    let wallet_address = ctx.accounts.multisig.key();
    let seeds = &[
        b"multisig",
        ctx.accounts.multisig.credential_id.as_slice(),
        &[ctx.accounts.multisig.bump]
    ];
    
    let ix = solana_program::system_instruction::transfer(
        &wallet_address,
        &destination,
        amount
    );
    
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.multisig.to_account_info(),
            ctx.accounts.destination.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[seeds]
    )?;
    
    msg!("Chuyển tiền thành công");
    Ok(())
}

