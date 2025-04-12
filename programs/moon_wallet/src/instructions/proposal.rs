use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::instructions::wallet::process_credential_id_seed;
use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
use anchor_lang::solana_program::hash::hash;
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(proposal_id: u64, description: String, proposer_guardian_id: u64)]
pub struct CreateProposal<'info> {
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
                32 +  // multisig
                8 +   // proposal_id
                32 +  // proposer
                4 + description.len() + // description 
                4 + 10 +  // action (max 10 chars)
                1 + 8 + 32 + 32 + // ActionParams (với các Option)
                1 +   // status
                8 +   // created_at
                1 + 8 + // executed_at Option<i64>
                1 +   // signatures_count
                1 +   // required_signatures
                1,    // bump
        seeds = [
            b"proposal".as_ref(), 
            multisig.key().as_ref(), 
            &proposal_id.to_le_bytes()
        ],
        bump
    )]
    pub proposal: Account<'info, TransactionProposal>,
    
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &proposer_guardian_id.to_le_bytes()],
        bump = proposer_guardian.bump
    )]
    pub proposer_guardian: Account<'info, Guardian>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

pub fn create_proposal(
    ctx: Context<CreateProposal>, 
    proposal_id: u64,
    description: String,
    _proposer_guardian_id: u64,
    action: String,
    params: ActionParams
) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let proposal = &mut ctx.accounts.proposal;
    let guardian = &ctx.accounts.proposer_guardian;
    let clock = &ctx.accounts.clock;
    
    // Kiểm tra guardian có hoạt động không
    require!(guardian.is_active, WalletError::InactiveGuardian);
    
    // Kiểm tra độ dài mô tả
    require!(description.len() <= 100, WalletError::NameTooLong);
    
    // Kiểm tra hành động hợp lệ (hiện tại chỉ hỗ trợ "transfer")
    require!(
        action == "transfer", 
        WalletError::UnsupportedAction
    );
    
    // Thiết lập thông tin đề xuất
    proposal.multisig = multisig.key();
    proposal.proposal_id = proposal_id;
    proposal.proposer = ctx.accounts.payer.key();
    proposal.description = description;
    proposal.action = action;
    proposal.params = params;
    proposal.status = ProposalStatus::Pending;
    proposal.created_at = clock.unix_timestamp;
    proposal.executed_at = None;
    proposal.signatures_count = 0; // Bắt đầu với 0 chữ ký
    proposal.required_signatures = multisig.threshold;
    proposal.bump = ctx.bumps.proposal;
    
    msg!("Đã tạo đề xuất giao dịch thành công với ID: {}", proposal_id);
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(proposal_id: u64, guardian_id: u64, timestamp: i64)]
pub struct ApproveProposal<'info> {
    #[account(mut)]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [
            b"proposal".as_ref(), 
            multisig.key().as_ref(), 
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Pending @ WalletError::InvalidOperation,
        constraint = *multisig.to_account_info().key == proposal.multisig @ WalletError::MultisigMismatch
    )]
    pub proposal: Account<'info, TransactionProposal>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + 
                32 +  // proposal
                8 +   // guardian_id
                8 +   // signature_time
                1,    // bump
        seeds = [
            b"signature".as_ref(),
            proposal.key().as_ref(),
            &guardian_id.to_le_bytes()
        ],
        bump
    )]
    pub signature: Account<'info, ProposalSignature>,
    
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &guardian_id.to_le_bytes()],
        bump = guardian.bump,
        constraint = guardian.is_active @ WalletError::InactiveGuardian
    )]
    pub guardian: Account<'info, Guardian>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Đây là tài khoản sysvar chứa thông tin về các instruction trong transaction
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instruction_sysvar: AccountInfo<'info>,
    
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

/// Hàm chuẩn hóa public key trước khi tính hash
/// Đảm bảo format của public key đồng nhất trước khi hash
fn standardize_pubkey(pubkey: &[u8; 33]) -> [u8; 33] {
    // Đảm bảo public key chỉ được xử lý thống nhất
    // Hiện tại chỉ trả về pubkey gốc, có thể mở rộng xử lý trong tương lai
    msg!("Standardizing pubkey: {}", to_hex(pubkey));
    
    // Chỉ trả về pubkey gốc, đảm bảo xử lý giống hệt ở wallet.rs
    *pubkey
}

pub fn approve_proposal(
    ctx: Context<ApproveProposal>, 
    proposal_id: u64,
    guardian_id: u64,
    timestamp: i64,
    message: Vec<u8>
) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let proposal = &mut ctx.accounts.proposal;
    let signature = &mut ctx.accounts.signature;
    let guardian = &ctx.accounts.guardian;
    let clock = &ctx.accounts.clock;
    
    // Thêm logs để debug
    msg!("Bắt đầu phê duyệt đề xuất với ID: {}", proposal_id);
    msg!("Địa chỉ multisig: {}", multisig.key());
    msg!("Guardian ID: {}", guardian_id);
    msg!("Timestamp: {}", timestamp);
    
    // Thêm kiểm tra chủ sở hữu của tài khoản multisig
    require!(
        *multisig.to_account_info().owner == crate::ID,
        WalletError::InvalidOwner
    );
    
    // 1. Kiểm tra timestemp
    require!(
        timestamp <= clock.unix_timestamp + 60, 
        WalletError::FutureTimestamp
    );
    
    require!(
        timestamp >= clock.unix_timestamp - 300,
        WalletError::ExpiredTimestamp
    );
    
    // 2. Xác thực WebAuthn nếu guardian có webauthn_pubkey
    if let Some(webauthn_pubkey) = guardian.webauthn_pubkey {
        msg!("Guardian có WebAuthn public key: {}", to_hex(&webauthn_pubkey));
        
        // Kiểm tra instruction sysvar
        let instruction_sysvar = &ctx.accounts.instruction_sysvar;
        require!(
            !instruction_sysvar.data_is_empty(),
            WalletError::InstructionMissing
        );
        
        // Tải instruction Secp256r1
        let secp_ix = load_instruction_at_checked(0, instruction_sysvar)?;
        
        // Kiểm tra ID
        let secp256r1_verify_id = Pubkey::from_str("Secp256r1SigVerify1111111111111111111111111").unwrap();
        require!(
            secp_ix.program_id == secp256r1_verify_id,
            WalletError::InvalidSignatureVerification
        );
        
        // Trích xuất và kiểm tra public key
        let pk_in_ix = extract_public_key_from_secp_instruction(&secp_ix.data)?;
        
        msg!("Public key từ instruction: {}", to_hex(&pk_in_ix));
        
        require!(
            pk_in_ix == webauthn_pubkey,
            WalletError::PublicKeyMismatch
        );
        
        // Chuẩn hóa public key trước khi hash - THAY ĐỔI Ở ĐÂY
        let standardized_pubkey = standardize_pubkey(&webauthn_pubkey);
        msg!("Standardized public key: {}", to_hex(&standardized_pubkey));
        
        // Tính hash của webauthn public key
        let pubkey_hash = hash(&standardized_pubkey).to_bytes();
        let pubkey_hash_hex = to_hex(&pubkey_hash[0..6]);
        msg!("Public key hash after standardization: {}", pubkey_hash_hex);
        
        // Tạo message kỳ vọng
        let expected_message = format!(
            "approve:proposal_{},guardian_{},timestamp:{},pubkey:{}",
            proposal_id,
            guardian_id,
            timestamp,
            pubkey_hash_hex
        );
        
        // In thông báo message để debug
        msg!("Expected message: {}", expected_message);
        msg!("Received message length: {}", message.len());
        msg!("Received message: {}", String::from_utf8_lossy(&message));
        
        // So sánh từng byte để debug
        if expected_message.as_bytes().len() == message.len() {
            for (i, (exp, rec)) in expected_message.as_bytes().iter().zip(message.iter()).enumerate() {
                if exp != rec {
                    msg!("Khác biệt tại vị trí [{}]: Expected {} ({}), Received {} ({})", 
                        i, exp, char::from(*exp), rec, char::from(*rec));
                }
            }
        }
        
        // Kiểm tra message
        require!(
            message == expected_message.as_bytes(),
            WalletError::MessageMismatch
        );
    }
    
    // Ghi nhận thông tin chữ ký
    signature.proposal = proposal.key();
    signature.guardian_id = guardian_id;
    signature.signature_time = clock.unix_timestamp;
    signature.bump = ctx.bumps.signature;
    
    // Tăng số lượng chữ ký
    proposal.signatures_count += 1;
    
    msg!("Guardian {} đã phê duyệt đề xuất {}", guardian_id, proposal_id);
    
    Ok(())
}

/// Trích xuất public key từ instruction Secp256r1
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

/// Chuyển đổi mảng bytes thành chuỗi hex
fn to_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let hex = format!("{:02x}", byte);
        result.push_str(&hex);
    }
    result
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [
            b"proposal".as_ref(), 
            multisig.key().as_ref(), 
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Pending @ WalletError::InvalidOperation,
        constraint = proposal.signatures_count >= proposal.required_signatures @ WalletError::InvalidOperation,
        constraint = *multisig.to_account_info().key == proposal.multisig @ WalletError::MultisigMismatch
    )]
    pub proposal: Account<'info, TransactionProposal>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Đây là địa chỉ đích được kiểm tra trong hàm xử lý
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

pub fn execute_proposal(
    ctx: Context<ExecuteProposal>,
    proposal_id: u64
) -> Result<()> {
    // Lưu các thông tin cần thiết trước khi mượn multisig dưới dạng mutable
    let multisig_credential_id = ctx.accounts.multisig.credential_id.clone();
    let multisig_bump = ctx.accounts.multisig.bump;
    
    // Bây giờ mượn multisig dưới dạng mutable
    let multisig = &mut ctx.accounts.multisig;
    let proposal = &mut ctx.accounts.proposal;
    let clock = &ctx.accounts.clock;
    
    // Kiểm tra chủ sở hữu của tài khoản multisig là program
    require!(
        *multisig.to_account_info().owner == crate::ID,
        WalletError::InvalidOwner
    );
    
    // Kiểm tra đề xuất đã đủ chữ ký
    require!(
        proposal.signatures_count >= proposal.required_signatures,
        WalletError::InvalidOperation
    );
    
    // In log để debug
    msg!("Thực thi đề xuất với ID: {}", proposal_id);
    msg!("Địa chỉ multisig: {}", multisig.key());
    msg!("Số chữ ký hiện tại: {}/{}", proposal.signatures_count, proposal.required_signatures);
    
    // Thực hiện hành động dựa trên loại
    match proposal.action.as_str() {
        "transfer" => {
            // Kiểm tra thông tin đích đến
            let destination = ctx.accounts.destination.key();
            let params_destination = proposal.params.destination.ok_or(WalletError::InvalidOperation)?;
            
            require!(
                params_destination == destination,
                WalletError::InvalidOperation
            );
            
            // Lấy số lượng lamports cần chuyển
            let amount = proposal.params.amount.ok_or(WalletError::InvalidOperation)?;
            
            // Chuẩn bị thông tin cho transfer
            let multisig_info = multisig.to_account_info();
            let credential_id_bytes = process_credential_id_seed(&multisig_credential_id);
            let _seeds = &[
                b"multisig".as_ref(),
                &credential_id_bytes,
                &[multisig_bump]
            ];
            
            // Thực hiện chuyển lamports
            let dest_starting_lamports = ctx.accounts.destination.lamports();
            **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
                .checked_add(amount)
                .ok_or(WalletError::ArithmeticOverflow)?;
            
            let multisig_starting_lamports = multisig_info.lamports();
            **multisig_info.lamports.borrow_mut() = multisig_starting_lamports
                .checked_sub(amount)
                .ok_or(WalletError::InsufficientFunds)?;
            
            msg!("Đã chuyển {} SOL đến {}", amount as f64 / 1_000_000_000.0, destination);
        },
        _ => return Err(WalletError::UnsupportedAction.into())
    }
    
    // Cập nhật trạng thái đề xuất
    proposal.status = ProposalStatus::Executed;
    proposal.executed_at = Some(clock.unix_timestamp);
    
    // Tăng transaction_nonce của ví
    multisig.transaction_nonce += 1;
    multisig.last_transaction_timestamp = clock.unix_timestamp;
    
    msg!("Đã thực thi đề xuất {} thành công", proposal_id);
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(proposal_id: u64, guardian_id: u64, timestamp: i64)]
pub struct RejectProposal<'info> {
    // Bỏ seeds và bump cho multisig
    #[account(mut)]
    pub multisig: Account<'info, MultiSigWallet>,
    
    #[account(
        mut,
        seeds = [
            b"proposal".as_ref(), 
            multisig.key().as_ref(), 
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Pending @ WalletError::InvalidOperation,
        // Thêm ràng buộc đảm bảo multisig truyền vào khớp với multisig lưu trong đề xuất
        constraint = *multisig.to_account_info().key == proposal.multisig @ WalletError::MultisigMismatch
    )]
    pub proposal: Account<'info, TransactionProposal>,
    
    #[account(
        seeds = [b"guardian".as_ref(), multisig.key().as_ref(), &guardian_id.to_le_bytes()],
        bump = guardian.bump,
        constraint = guardian.is_active @ WalletError::InactiveGuardian
    )]
    pub guardian: Account<'info, Guardian>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Đây là tài khoản sysvar chứa thông tin về các instruction trong transaction
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instruction_sysvar: AccountInfo<'info>,
    
    pub clock: Sysvar<'info, Clock>,
}

pub fn reject_proposal(
    ctx: Context<RejectProposal>,
    proposal_id: u64,
    guardian_id: u64,
    timestamp: i64,
    message: Vec<u8>
) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let proposal = &mut ctx.accounts.proposal;
    let guardian = &ctx.accounts.guardian;
    let clock = &ctx.accounts.clock;
    
    // Thêm logs để debug
    msg!("Bắt đầu từ chối đề xuất với ID: {}", proposal_id);
    msg!("Địa chỉ multisig: {}", multisig.key());
    msg!("Guardian ID: {}", guardian_id);
    msg!("Timestamp: {}", timestamp);
    
    // Thêm kiểm tra chủ sở hữu của tài khoản multisig
    require!(
        *multisig.to_account_info().owner == crate::ID,
        WalletError::InvalidOwner
    );
    
    // Kiểm tra timestamp
    require!(
        timestamp <= clock.unix_timestamp + 60, 
        WalletError::FutureTimestamp
    );
    
    require!(
        timestamp >= clock.unix_timestamp - 300,
        WalletError::ExpiredTimestamp
    );
    
    // Xác thực WebAuthn nếu guardian có webauthn_pubkey
    if let Some(webauthn_pubkey) = guardian.webauthn_pubkey {
        msg!("Guardian có WebAuthn public key: {}", to_hex(&webauthn_pubkey));
        
        // Kiểm tra instruction sysvar
        let instruction_sysvar = &ctx.accounts.instruction_sysvar;
        require!(
            !instruction_sysvar.data_is_empty(),
            WalletError::InstructionMissing
        );
        
        // Tải instruction Secp256r1
        let secp_ix = load_instruction_at_checked(0, instruction_sysvar)?;
        
        // Kiểm tra ID
        let secp256r1_verify_id = Pubkey::from_str("Secp256r1SigVerify1111111111111111111111111").unwrap();
        require!(
            secp_ix.program_id == secp256r1_verify_id,
            WalletError::InvalidSignatureVerification
        );
        
        // Trích xuất và kiểm tra public key
        let pk_in_ix = extract_public_key_from_secp_instruction(&secp_ix.data)?;
        
        msg!("Public key từ instruction: {}", to_hex(&pk_in_ix));
        
        require!(
            pk_in_ix == webauthn_pubkey,
            WalletError::PublicKeyMismatch
        );
        
        // Tính hash của webauthn public key
        let pubkey_hash = hash(&webauthn_pubkey).to_bytes();
        let pubkey_hash_hex = to_hex(&pubkey_hash[0..6]);
        
        // Tạo message kỳ vọng
        let expected_message = format!(
            "reject:proposal_{},guardian_{},timestamp:{},pubkey:{}",
            proposal_id,
            guardian_id,
            timestamp,
            pubkey_hash_hex
        );
        
        // In thông báo message để debug
        msg!("Expected message: {}", expected_message);
        msg!("Received message length: {}", message.len());
        msg!("Received message: {}", String::from_utf8_lossy(&message));
        
        // So sánh từng byte để debug
        if expected_message.as_bytes().len() == message.len() {
            for (i, (exp, rec)) in expected_message.as_bytes().iter().zip(message.iter()).enumerate() {
                if exp != rec {
                    msg!("Khác biệt tại vị trí [{}]: Expected {} ({}), Received {} ({})", 
                        i, exp, char::from(*exp), rec, char::from(*rec));
                }
            }
        }
        
        // Kiểm tra message
        require!(
            message == expected_message.as_bytes(),
            WalletError::MessageMismatch
        );
    }
    
    // Cập nhật trạng thái đề xuất thành Rejected
    proposal.status = ProposalStatus::Rejected;
    
    msg!("Guardian {} đã từ chối đề xuất {}", guardian_id, proposal_id);
    
    Ok(())
} 