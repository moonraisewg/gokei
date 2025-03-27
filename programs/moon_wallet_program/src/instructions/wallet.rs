use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_lang::solana_program;
use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
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
               1 +  // bump: u8
               8 +  // transaction_nonce: u64
               8,   // last_transaction_timestamp: i64
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
    
    // Khởi tạo recovery hash với giá trị mặc định
    // Recovery hash thực sự sẽ được cập nhật khi tạo guardian đầu tiên
    multisig.recovery_hash = recovery_hash;
    multisig.recovery_salt = [0; 16];
    multisig.recovery_nonce = 0;
    multisig.bump = ctx.bumps.multisig;
    
    // Khởi tạo các trường mới
    multisig.transaction_nonce = 0;
    multisig.last_transaction_timestamp = 0;
    
    msg!("Đã khởi tạo ví MultiSign thành công");
    msg!("Hãy thêm owner làm guardian đầu tiên để thiết lập recovery hash");
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
    
    /// CHECK: Đây là pubkey của chủ sở hữu mới sau khi khôi phục, không cần kiểm tra vì nó chỉ được dùng để lưu vào multisig
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

// Context cho việc xác thực và thực hiện giao dịch
#[derive(Accounts)]
pub struct VerifyAndExecute<'info> {
    #[account(
        mut,
        seeds = [b"multisig", multisig.credential_id.as_slice()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, MultiSigWallet>,
    
    // SYSVAR để lấy timestamp từ blockchain
    pub clock: Sysvar<'info, Clock>,
    
    // SYSVAR để kiểm tra instruction trước đó
    /// CHECK: Sysvar instruction data
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instruction_sysvar: AccountInfo<'info>,
    
    // System Program để thực hiện chuyển SOL
    pub system_program: Program<'info, System>,
    
    // Người ký nonce - người thực hiện giao dịch
    #[account(mut)]
    pub payer: Signer<'info>,
    
    // Tài khoản nhận SOL (mục tiêu của giao dịch chuyển tiền)
    /// CHECK: Tài khoản này được xác thực trong quá trình xử lý
    #[account(mut)]
    pub destination: AccountInfo<'info>,
}

// Hàm xác thực và thực hiện giao dịch
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
    
    // 1. Kiểm tra nonce để ngăn tấn công replay
    require!(
        nonce == multisig.transaction_nonce + 1,
        WalletError::InvalidNonce
    );
    
    // 2. Kiểm tra timestamp để ngăn tấn công replay
    // Đảm bảo timestamp hợp lý (không quá cũ và không quá tương lai)
    require!(
        timestamp <= clock.unix_timestamp + 60, // Cho phép chênh lệch 60 giây với tương lai
        WalletError::FutureTimestamp
    );
    
    require!(
        timestamp >= multisig.last_transaction_timestamp,
        WalletError::OutdatedTimestamp
    );
    
    // Kiểm tra xem thời gian không quá cũ (ví dụ: 5 phút)
    require!(
        timestamp >= clock.unix_timestamp - 300,
        WalletError::ExpiredTimestamp
    );
    
    // 3. Đảm bảo rằng transaction có ít nhất 2 instruction
    let instruction_sysvar = &ctx.accounts.instruction_sysvar;
    require!(
        !instruction_sysvar.data_is_empty(),
        WalletError::InstructionMissing
    );
    
    // 4. Kiểm tra instruction trước đó là gọi đến Secp256r1SigVerify
    let secp_ix = load_instruction_at_checked(0, instruction_sysvar)?;
    
    // Địa chỉ Program ID của Secp256r1SigVerify
    let secp256r1_verify_id = Pubkey::from_str("Secp256r1SigVerify1111111111111111111111111").unwrap();
    
    require!(
        secp_ix.program_id == secp256r1_verify_id,
        WalletError::InvalidSignatureVerification
    );
    
    // 5. Kiểm tra xem dữ liệu instruction chứa public key đúng với wallet
    let pk_in_ix = extract_public_key_from_secp_instruction(&secp_ix.data)?;
    require!(
        pk_in_ix == multisig.webauthn_pubkey,
        WalletError::PublicKeyMismatch
    );
    
    // 6. Kiểm tra message trong instruction secp256r1 trùng với message truyền vào
    let msg_in_ix = extract_message_from_secp_instruction(&secp_ix.data)?;
    require!(
        msg_in_ix == message,
        WalletError::MessageMismatch
    );
    
    // 7. Kiểm tra nội dung message khớp với thông tin giao dịch
    // Format chuẩn: "action:transaction_data,nonce:{nonce},timestamp:{timestamp}"
    // ví dụ: "transfer:0.1_SOL_to_Gy3L5tNnWaNKdVWfvk3wo9P8oF6bHy6RAY6NCgtvypAG,nonce:123,timestamp:1693547315"
    
    // Tạo expected_message để so sánh
    let mut expected_message = String::new();
    
    match action.as_str() {
        "transfer" => {
            let amount = params.amount.ok_or(WalletError::InvalidOperation)?;
            let destination = params.destination.ok_or(WalletError::InvalidOperation)?;
            
            expected_message = format!(
                "transfer:{}_SOL_to_{},nonce:{},timestamp:{}",
                amount as f64 / 1_000_000_000.0, // Chuyển lamports sang SOL
                destination.to_string(),
                nonce,
                timestamp
            );
        },
        // Thêm các action khác ở đây
        _ => return Err(WalletError::UnsupportedAction.into())
    }
    
    require!(
        message == expected_message.as_bytes(),
        WalletError::MessageMismatch
    );
    
    // 8. Cập nhật nonce và timestamp
    multisig.transaction_nonce = nonce;
    multisig.last_transaction_timestamp = timestamp;
    
    // 9. Thực hiện hành động tương ứng
    match action.as_str() {
        "transfer" => execute_transfer(ctx, &params),
        // Thêm các hành động khác ở đây
        _ => Err(WalletError::UnsupportedAction.into())
    }
}

// Hàm trích xuất public key từ instruction data của Secp256r1SigVerify
fn extract_public_key_from_secp_instruction(data: &[u8]) -> Result<[u8; 65]> {
    // Đọc offsets từ data
    if data.len() < 16 {
        return Err(WalletError::InvalidInstructionData.into());
    }
    
    let num_signatures = data[0] as usize;
    if num_signatures != 1 {
        return Err(WalletError::InvalidSignatureCount.into());
    }
    
    // Lấy public key offset từ instruction data
    let public_key_offset = u16::from_le_bytes([data[6], data[7]]) as usize;
    
    // Trích xuất và trả về public key
    let mut pk = [0u8; 65];
    if data.len() < public_key_offset + 65 {
        return Err(WalletError::InvalidInstructionData.into());
    }
    
    pk.copy_from_slice(&data[public_key_offset..public_key_offset + 65]);
    Ok(pk)
}

// Hàm trích xuất message từ instruction data của Secp256r1SigVerify
fn extract_message_from_secp_instruction(data: &[u8]) -> Result<Vec<u8>> {
    // Đọc offsets từ data
    if data.len() < 16 {
        return Err(WalletError::InvalidInstructionData.into());
    }
    
    // Lấy message offset và size từ instruction data
    let message_offset = u16::from_le_bytes([data[10], data[11]]) as usize;
    let message_size = u16::from_le_bytes([data[12], data[13]]) as usize;
    
    // Trích xuất và trả về message
    if data.len() < message_offset + message_size {
        return Err(WalletError::InvalidInstructionData.into());
    }
    
    Ok(data[message_offset..message_offset + message_size].to_vec())
}

// Hàm thực hiện chuyển tiền
fn execute_transfer(ctx: Context<VerifyAndExecute>, params: &ActionParams) -> Result<()> {
    // Lấy số tiền từ params
    let amount = params.amount.ok_or(WalletError::InvalidOperation)?;
    
    // Lấy destination từ context thay vì params
    let destination = ctx.accounts.destination.key();
    
    // Kiểm tra destination trong params khớp với destination trong context
    let params_destination = params.destination.ok_or(WalletError::InvalidOperation)?;
    require!(
        params_destination == destination,
        WalletError::InvalidOperation
    );
    
    // Thực hiện chuyển tiền
    msg!("Thực hiện chuyển {} SOL đến {}", amount as f64 / 1_000_000_000.0, destination);
    
    // Lấy tài khoản PDA của multisig wallet
    let wallet_address = ctx.accounts.multisig.key();
    let seeds = &[
        b"multisig",
        ctx.accounts.multisig.credential_id.as_slice(),
        &[ctx.accounts.multisig.bump]
    ];
    
    // Tạo transfer instruction
    let ix = solana_program::system_instruction::transfer(
        &wallet_address,
        &destination,
        amount
    );
    
    // Thực hiện CPI (Cross-Program Invocation)
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

