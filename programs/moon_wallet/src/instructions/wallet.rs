use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_lang::solana_program;
use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
use crate::state::*;
use crate::errors::*;


#[derive(Accounts)]
#[instruction(threshold: u8)]
pub struct InitializeMultisig<'info> {
    #[account(
        init,
        payer = fee_payer,
        space = 8 + 
               1 +  // threshold
               1 +  // guardian_count
               8 +  // recovery_nonce
               1 +  // bump
               8 +  // transaction_nonce
               8,   // last_transaction_timestamp
        seeds = [b"multisig".as_ref(), b"seed_for_pda".as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_multisig(
    ctx: Context<InitializeMultisig>,
    threshold: u8,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    
    require!(threshold > 0, WalletError::InvalidConfig);
    
    multisig.threshold = threshold;
    multisig.guardian_count = 0;
    multisig.recovery_nonce = 0;
    multisig.bump = ctx.bumps.multisig;
    multisig.transaction_nonce = 0;
    multisig.last_transaction_timestamp = 0;
    
    msg!("Đã khởi tạo ví MultiSign thành công");
    msg!("Hãy thêm guardian đầu tiên làm owner");
    Ok(())
}

#[derive(Accounts)]
pub struct VerifyAndExecute<'info> {
    #[account(
        mut,
        seeds = [b"multisig".as_ref(), b"seed_for_pda".as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    /// Tìm guardian owner có webauthn_pubkey mà chúng ta cần xác thực
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), guardian.guardian_id.as_ref()],
        constraint = guardian.is_owner == true,
        bump = guardian.bump
    )]
    pub guardian: Account<'info, Guardian>,
    
    pub clock: Sysvar<'info, Clock>,
    
    /// CHECK: Đây là tài khoản sysvar chứa thông tin về các instruction trong transaction
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instruction_sysvar: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Đây là địa chỉ đích để gửi giao dịch
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
    let guardian = &ctx.accounts.guardian;
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
    
    let webauthn_pubkey = guardian.webauthn_pubkey.ok_or(WalletError::WebAuthnNotConfigured)?;
    
    let pk_in_ix = extract_public_key_from_secp_instruction(&secp_ix.data)?;
    require!(
        pk_in_ix == webauthn_pubkey,
        WalletError::PublicKeyMismatch
    );
    
    let msg_in_ix = extract_message_from_secp_instruction(&secp_ix.data)?;
    require!(
        msg_in_ix == message,
        WalletError::MessageMismatch
    );
    
    let expected_message = match action.as_str() {
        "transfer" => {
            let amount = params.amount.ok_or(WalletError::InvalidOperation)?;
            let destination = params.destination.ok_or(WalletError::InvalidOperation)?;
            
            format!(
                "transfer:{}_SOL_to_{},nonce:{},timestamp:{}",
                amount as f64 / 1_000_000_000.0, 
                destination.to_string(),
                nonce,
                timestamp
            )
        },
        _ => return Err(WalletError::UnsupportedAction.into())
    };
    
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
        b"multisig".as_ref(),
        b"seed_for_pda".as_ref(),
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

