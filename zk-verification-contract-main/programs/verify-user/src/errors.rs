use anchor_lang::error_code;

#[error_code]
pub enum CustomError {
    #[msg("Invalid file name.")]
    InvalidFileName,
    #[msg("Invalid access type.")]
    InvalidAccessType,
    #[msg("Invalid time range.")]
    InvalidTimeRange,
    #[msg("Invalid time for verification.")]
    InvalidTime,
    #[msg("Verification failed.")]
    VerificationFailed,
    #[msg("Unauthorized access.")]
    Unauthorized,
}