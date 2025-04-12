use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
use crate::state::*;
use crate::errors::*;
use anchor_lang::solana_program::hash::hash;


#[derive(Accounts)]
#[instruction(threshold: u8, credential_id: String)]
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
               8 +  // last_transaction_timestamp
               32 + // owner
               4 + credential_id.len(), // credential_id với 4 bytes cho độ dài
        seeds = [b"multisig".as_ref(), &process_credential_id_seed(&credential_id)],
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
    credential_id: String,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    
    require!(threshold > 0, WalletError::InvalidConfig);
    require!(credential_id.len() > 0, WalletError::InvalidConfig);
    require!(credential_id.len() <= 64, WalletError::NameTooLong);
    
    multisig.threshold = threshold;
    multisig.guardian_count = 0;
    multisig.recovery_nonce = 0;
    multisig.bump = ctx.bumps.multisig;
    multisig.transaction_nonce = 0;
    multisig.last_transaction_timestamp = 0;
    multisig.owner = ctx.accounts.fee_payer.key();
    multisig.credential_id = credential_id;

    Ok(())
}

#[derive(Accounts)]
pub struct VerifyAndExecute<'info> {
    #[account(
        mut,
        seeds = [b"multisig".as_ref(), &process_credential_id_seed(&multisig.credential_id)],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &guardian.guardian_id.to_le_bytes()],
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

/// Hàm chuẩn hóa public key trước khi tính hash
/// Đảm bảo format của public key đồng nhất trước khi hash
fn standardize_pubkey(pubkey: &[u8; 33]) -> [u8; 33] {
    // Đảm bảo public key chỉ được xử lý thống nhất
    // Hiện tại chỉ trả về pubkey gốc, có thể mở rộng xử lý trong tương lai
    msg!("Standardizing pubkey: {}", to_hex(pubkey));
    
    // Chỉ trả về pubkey gốc, đảm bảo xử lý giống nhau giữa các chức năng
    *pubkey
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
    
    // Chuẩn hóa public key trước khi hash - THAY ĐỔI Ở ĐÂY
    let standardized_pubkey = standardize_pubkey(&webauthn_pubkey);
    msg!("Standardized public key: {}", to_hex(&standardized_pubkey));

    let pubkey_hash = hash(&standardized_pubkey).to_bytes();
    let pubkey_hash_hex = to_hex(&pubkey_hash[0..6]);
    
    msg!("Public key hash after standardization: {}", pubkey_hash_hex);
    
    let expected_message = match action.as_str() {
        "transfer" => {
            let amount = params.amount.ok_or(WalletError::InvalidOperation)?;
            let destination = params.destination.ok_or(WalletError::InvalidOperation)?;
            
            format!(
                "transfer:{}_SOL_to_{},nonce:{},timestamp:{},pubkey:{}",
                amount as f64 / 1_000_000_000.0, 
                destination.to_string(),
                nonce,
                timestamp,
                pubkey_hash_hex
            )
        },
        _ => return Err(WalletError::UnsupportedAction.into())
    };
    
    msg!("Expected message: {}", expected_message);
   
    // Debug chi tiết để phát hiện sự khác biệt
    msg!("Expected message length: {}", expected_message.len());
    msg!("Received message length: {}", message.len());
    
    // Log từng byte của message để so sánh
    msg!("Expected message bytes:");
    for (i, byte) in expected_message.as_bytes().iter().enumerate() {
        msg!("  [{}] {} ({})", i, byte, char::from(*byte));
    }
    
    msg!("Received message bytes:");
    for (i, byte) in message.iter().enumerate() {
        msg!("  [{}] {} ({})", i, byte, char::from(*byte));
    }
    
    // So sánh các byte khác nhau
    if expected_message.as_bytes().len() == message.len() {
        for (i, (exp, rec)) in expected_message.as_bytes().iter().zip(message.iter()).enumerate() {
            if exp != rec {
                msg!("Khác biệt tại vị trí [{}]: Expected {} ({}), Received {} ({})", 
                    i, exp, char::from(*exp), rec, char::from(*rec));
            }
        }
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

fn execute_transfer(ctx: Context<VerifyAndExecute>, params: &ActionParams) -> Result<()> {
    let amount = params.amount.ok_or(WalletError::InvalidOperation)?;
    
    let destination = ctx.accounts.destination.key();
    
    let params_destination = params.destination.ok_or(WalletError::InvalidOperation)?;
    require!(
        params_destination == destination,
        WalletError::InvalidOperation
    );
    
    msg!("Thực hiện chuyển {} SOL đến {}", amount as f64 / 1_000_000_000.0, destination);
    
    let multisig_info = ctx.accounts.multisig.to_account_info();
    let credential_id_bytes = process_credential_id_seed(&ctx.accounts.multisig.credential_id);
    let _seeds = &[
        b"multisig".as_ref(),
        &credential_id_bytes,
        &[ctx.accounts.multisig.bump]
    ];
    
    let dest_starting_lamports = ctx.accounts.destination.lamports();
    **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports.checked_add(amount)
        .ok_or(WalletError::ArithmeticOverflow)?;
    
    let multisig_starting_lamports = multisig_info.lamports();
    **multisig_info.lamports.borrow_mut() = multisig_starting_lamports.checked_sub(amount)
        .ok_or(WalletError::InsufficientFunds)?;
    
    msg!("Chuyển {} SOL đến {} thành công", amount as f64 / 1_000_000_000.0, destination);
    
    Ok(())
}

pub fn process_credential_id_seed(credential_id: &str) -> [u8; 24] {
    msg!("CONTRACT - process_credential_id_seed");
    msg!("Input credential ID: {}", credential_id);
    
    let credential_bytes = credential_id.as_bytes();
    msg!("Credential bytes length: {}", credential_bytes.len());
    
    // Debug từng byte
    let bytes_hex = to_hex(credential_bytes);
    msg!("Credential bytes (hex): {}", bytes_hex);
    
    // Seed tối đa cho PDA là 32 bytes, trừ đi "multisig" (8 bytes) còn 24 bytes
    let mut result = [0u8; 24];
    
    if credential_bytes.len() > 24 {
        msg!("Credential ID dài quá 24 bytes, thực hiện hash");
        
        // Hash credential ID nếu quá dài để đảm bảo tính đồng nhất
        // Sử dụng thuật toán XOR đơn giản
        // Khác với phiên bản trước: Hash trực tiếp vào 24 bytes thay vì 32 bytes
        for (i, byte) in credential_bytes.iter().enumerate() {
            result[i % 24] ^= *byte;
        }
        
        // Debug kết quả hash
        let result_hex = to_hex(&result);
        msg!("Seed sau khi hash (hex): {}", result_hex);
    } else {
        // Copy bytes từ credential ID, padding với 0 nếu cần
        let len = credential_bytes.len();
        result[..len].copy_from_slice(credential_bytes);
        
        // Debug kết quả
        let result_hex = to_hex(&result);
        msg!("Seed không hash (hex, padded): {}", result_hex);
    }
    
    result
}

/// Chuyển đổi mảng bytes thành chuỗi hex
fn to_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Format mỗi byte thành 2 ký tự hex
        let hex = format!("{:02x}", byte);
        result.push_str(&hex);
    }
    result
}

