use anchor_lang::prelude::*;

use crate::errors::BetError;
use crate::state::{BetState, BettingPool};

#[derive(Accounts)]
pub struct JoinBet<'info> {
    #[account(mut)]
    pub participant_b: Signer<'info>,
    #[account(mut)]
    pub betting_pool: Account<'info, BettingPool>,
    /// Vault PDA that holds all SOL from this pool
    #[account(
        mut,
        seeds = [b"vault", betting_pool.key().as_ref()],
        bump = betting_pool.vault_bump,
    )]
    /// CHECK: vault PDA only holds SOL via system program
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<JoinBet>) -> Result<()> {
    let pool = &mut ctx.accounts.betting_pool;
    require!(pool.state == BetState::Created, BetError::InvalidState);
    require!(pool.participant_b == Pubkey::default(), BetError::AlreadyJoined);

    pool.participant_b = ctx.accounts.participant_b.key();

    // Transfer participant B stake into the vault
    if pool.bet_amount > 0 {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.participant_b.key(),
            &ctx.accounts.vault.key(),
            pool.bet_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.participant_b.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        pool.total_pool = pool
            .total_pool
            .checked_add(pool.bet_amount)
            .ok_or(crate::errors::BetError::InsufficientContribution)?;
    }
    pool.state = BetState::Active;

    Ok(())
}
