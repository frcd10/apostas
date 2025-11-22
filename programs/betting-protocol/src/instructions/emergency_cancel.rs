use anchor_lang::prelude::*;

use crate::state::{BetState, BettingPool};

#[derive(Accounts)]
pub struct EmergencyCancel<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
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
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EmergencyCancel>) -> Result<()> {
    let pool = &mut ctx.accounts.betting_pool;
    require_keys_eq!(pool.creator, ctx.accounts.creator.key());
    require!(pool.state == BetState::Created || pool.state == BetState::Active, crate::errors::BetError::InvalidState);

    // Simple refund logic: send all lamports in vault back to creator
    let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
    if vault_lamports > 0 {
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= vault_lamports;
        **ctx.accounts.creator.to_account_info().try_borrow_mut_lamports()? += vault_lamports;
    }

    pool.state = BetState::Cancelled;

    Ok(())
}
