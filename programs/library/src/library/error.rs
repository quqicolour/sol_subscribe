use anchor_lang::prelude::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Non owner.")]
    NonOwner,
    #[msg("Non manager.")]
    NonManager,
    #[msg("Non fee receiver.")]
    NonFeeReceiver,
    #[msg("Invalid subscribe type.")]
    InvalidType,
    #[msg("Mul overflow.")]
    MulOverflow,
    #[msg("Invalid time.")]
    InvalidTime
}