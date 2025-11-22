use anchor_lang::prelude::*;

use crate::errors::BetError;
use crate::state::{BetState, BettingPool};

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub winner: Signer<'info>,
    #[account(mut)]
    pub betting_pool: Account<'info, BettingPool>,
    /// Vault PDA holding the SOL
    #[account(
        mut,
        seeds = [b"vault", betting_pool.key().as_ref()],
        bump = betting_pool.vault_bump,
    )]
    /// CHECK: vault PDA only holds SOL via system program
    pub vault: UncheckedAccount<'info>,
    /// CHECK: arbiter fee recipient (must match pool.arbiter)
    #[account(mut)]
    pub arbiter: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimPrize>) -> Result<()> {
    let pool = &mut ctx.accounts.betting_pool;
    require!(pool.state == BetState::Decided, BetError::InvalidState);
    require!(pool.winner == Some(ctx.accounts.winner.key()), BetError::NotWinner);

    // Fetch vault balance
    let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
    require!(vault_lamports > 0, BetError::InsufficientContribution);

    // Calculate arbiter fee and winner amount
    let fee = vault_lamports
        .checked_mul(pool.arbiter_fee_bps as u64)
        .and_then(|v| v.checked_div(10_000))
        .ok_or(crate::errors::BetError::InsufficientContribution)?;
    let winner_amount = vault_lamports
        .checked_sub(fee)
        .ok_or(crate::errors::BetError::InsufficientContribution)?;

    // Pay arbiter
    if fee > 0 {
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= fee;
        **ctx.accounts.arbiter.to_account_info().try_borrow_mut_lamports()? += fee;
    }

    // Pay winner
    **ctx
        .accounts
        .vault
        .to_account_info()
        .try_borrow_mut_lamports()? -= winner_amount;
    **ctx
        .accounts
        .winner
        .to_account_info()
        .try_borrow_mut_lamports()? += winner_amount;

    pool.state = BetState::Claimed;

    Ok(())
}
