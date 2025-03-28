use anchor_lang::prelude::*;


#[error_code]
pub enum WalletError {
    #[msg("Không có quyền hoặc dữ liệu không hợp lệ")]
    InvalidOperation,
    
    #[msg("Giới hạn đã đạt tối đa")]
    LimitExceeded,
    
    #[msg("Guardian không hợp lệ")]
    GuardianError,
    
    #[msg("Cấu hình không hợp lệ")]
    InvalidConfig,
    
    #[msg("Recovery không hợp lệ")]
    InvalidRecovery,
    
    #[msg("Ngưỡng không hợp lệ")]
    InvalidThreshold,
    
    #[msg("WebAuthn chưa được cấu hình")]
    WebAuthnNotConfigured,
    
    #[msg("Tên ví không được vượt quá 32 ký tự")]
    NameTooLong,
    
    #[msg("Recovery key không hợp lệ")]
    InvalidRecoveryKey,
    
    #[msg("Không có guardian nào để xóa")]
    NoGuardians,
    
    #[msg("Nonce không hợp lệ")]
    InvalidNonce,
    
    #[msg("Timestamp thuộc về tương lai")]
    FutureTimestamp,
    
    #[msg("Timestamp quá cũ")]
    OutdatedTimestamp,
    
    #[msg("Timestamp đã hết hạn")]
    ExpiredTimestamp,
    
    #[msg("Instruction xác thực chữ ký bị thiếu")]
    InstructionMissing,
    
    #[msg("Xác thực chữ ký không hợp lệ")]
    InvalidSignatureVerification,
    
    #[msg("Public key không khớp với wallet")]
    PublicKeyMismatch,
    
    #[msg("Message không khớp")]
    MessageMismatch,
    
    #[msg("Dữ liệu instruction không hợp lệ")]
    InvalidInstructionData,
    
    #[msg("Số lượng chữ ký không hợp lệ")]
    InvalidSignatureCount,
    
    #[msg("Hành động không được hỗ trợ")]
    UnsupportedAction,
    
    #[msg("Guardian không hợp lệ hoặc không được tìm thấy")]
    InvalidGuardian,
    
    #[msg("Guardian đang không hoạt động")]
    InactiveGuardian,
}