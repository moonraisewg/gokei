use anchor_lang::prelude::*;

// Định nghĩa các mã lỗi
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
}