use anchor_lang::prelude::*;

#[error_code]
pub enum BetError {
    #[msg("Unauthorized arbiter action")]
    UnauthorizedArbiter,
    #[msg("Lock period has not expired")]
    LockPeriodNotExpired,
    #[msg("Bet already joined")]
    AlreadyJoined,
    #[msg("Invalid state for this operation")]
    InvalidState,
    #[msg("Only winner can claim prize")]
    NotWinner,
    #[msg("Insufficient contribution")]
    InsufficientContribution,
}
