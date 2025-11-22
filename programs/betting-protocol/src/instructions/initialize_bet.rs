use anchor_lang::prelude::*;

use crate::state::{BetState, BettingPool};

#[derive(Accounts)]
pub struct InitializeBet<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    /// CHECK: arbiter is validated as Pubkey only
    pub arbiter: UncheckedAccount<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + BettingPool::MAX_SIZE,
    )]
    pub betting_pool: Account<'info, BettingPool>,
    /// Vault PDA that will hold all SOL from this pool
    #[account(
        mut,
        seeds = [b"vault", betting_pool.key().as_ref()],
        bump,
    )]
    /// CHECK: vault PDA only holds SOL via system program
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<InitializeBet>, arbiter: Pubkey, bet_amount: u64, lock_period: i64) -> Result<()> {
    let pool = &mut ctx.accounts.betting_pool;
    pool.creator = ctx.accounts.creator.key();
    pool.arbiter = arbiter;
    pool.participant_a = ctx.accounts.creator.key();
    pool.participant_b = Pubkey::default();
    pool.bet_amount = bet_amount;
    pool.total_pool = 0;
    pool.start_time = ctx.accounts.clock.unix_timestamp;
    pool.lock_period = lock_period;
    pool.state = BetState::Created;
    pool.winner = None;
    pool.arbiter_fee_bps = 100; // 1%
    // Por enquanto não usamos o bump de forma dinâmica
    pool.vault_bump = 0;

    // Transfer creator's initial stake into the vault
    if bet_amount > 0 {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.creator.key(),
            &ctx.accounts.vault.key(),
            bet_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        pool.total_pool = pool
            .total_pool
            .checked_add(bet_amount)
            .ok_or(crate::errors::BetError::InsufficientContribution)?;
    }
    Ok(())
}
