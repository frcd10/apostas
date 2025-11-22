use anchor_lang::prelude::*;

use crate::errors::BetError;
use crate::state::{validate_lock_expired, BetState, BettingPool};

#[derive(Accounts)]
pub struct DeclareWinner<'info> {
    pub arbiter: Signer<'info>,
    #[account(mut)]
    pub betting_pool: Account<'info, BettingPool>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<DeclareWinner>, winner: Pubkey) -> Result<()> {
    let pool = &mut ctx.accounts.betting_pool;
    require!(ctx.accounts.arbiter.key() == pool.arbiter, BetError::UnauthorizedArbiter);
    require!(pool.state == BetState::Active, BetError::InvalidState);

    let now = ctx.accounts.clock.unix_timestamp;
    validate_lock_expired(pool.start_time, pool.lock_period, now)?;

    pool.winner = Some(winner);
    pool.state = BetState::Decided;

    Ok(())
}
