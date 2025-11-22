use anchor_lang::prelude::*;

use crate::errors::BetError;

#[account]
pub struct BettingPool {
    pub creator: Pubkey,
    pub arbiter: Pubkey,
    pub participant_a: Pubkey,
    pub participant_b: Pubkey,
    pub bet_amount: u64,
    pub total_pool: u64,
    pub start_time: i64,
    pub lock_period: i64,
    pub state: BetState,
    pub winner: Option<Pubkey>,
    pub arbiter_fee_bps: u16,
    pub vault_bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BetState {
    Created,
    Active,
    Decided,
    Claimed,
    Cancelled,
}

#[account]
pub struct GroupBet {
    pub pool: Pubkey,
    pub total_on_a: u64,
    pub total_on_b: u64,
    pub bump: u8,
}

#[account]
pub struct ParticipantPosition {
    pub owner: Pubkey,
    pub group_bet: Pubkey,
    pub amount_on_a: u64,
    pub amount_on_b: u64,
}

impl BettingPool {
    pub const MAX_SIZE: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1 + 1 + 32 + 2 + 1;
}

impl GroupBet {
    pub const MAX_SIZE: usize = 32 + 8 + 8 + 1;
}

impl ParticipantPosition {
    pub const MAX_SIZE: usize = 32 + 32 + 8 + 8;
}

pub fn validate_lock_expired(start_time: i64, lock_period: i64, now: i64) -> Result<()> {
    require!(now >= start_time + lock_period, BetError::LockPeriodNotExpired);
    Ok(())
}
