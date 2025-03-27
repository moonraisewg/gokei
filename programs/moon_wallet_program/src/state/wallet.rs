use anchor_lang::prelude::*;


#[account]
pub struct MultiSigWallet {
    pub owner: Pubkey,               // 32 bytes
    pub threshold: u8,               // 1 byte
    pub has_webauthn: bool,          // 1 byte
    pub webauthn_pubkey: [u8; 65],   // 65 bytes - lưu ở định dạng đúng với Secp256r1SigVerify
    pub credential_id: Vec<u8>,      // Thêm credential_id (kích thước thay đổi)
    pub guardian_count: u8,          // 1 byte
    pub recovery_hash: [u8; 32],     // 32 bytes - hash SHA-256 của hash secp256r1
    pub recovery_salt: [u8; 16],     // 16 bytes - salt ngẫu nhiên
    pub recovery_nonce: u64,         // 8 bytes - nonce tránh replay attack cho recovery
    pub bump: u8,                    // 1 byte
    pub transaction_nonce: u64,      // 8 bytes - nonce cho giao dịch, ngăn replay attack
    pub last_transaction_timestamp: i64, // 8 bytes - timestamp giao dịch cuối cùng
}

// Cấu trúc dữ liệu cho các tham số hành động
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ActionParams {
    // Các trường chung cho tất cả các hành động
    pub amount: Option<u64>,         // Số lượng (nếu cần)
    pub destination: Option<Pubkey>, // Địa chỉ đích (nếu cần)
    pub token_mint: Option<Pubkey>,  // Địa chỉ mint token (nếu cần)
    // Thêm các trường khác nếu cần
}