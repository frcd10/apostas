use anchor_lang::prelude::*;

use crate::errors::BetError;
use crate::state::{BetState, BettingPool, GroupBet, ParticipantPosition};

#[derive(Accounts)]
pub struct AddGroupBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub betting_pool: Account<'info, BettingPool>,
    #[account(
        init,
        payer = user,
        seeds = [b"group_bet", betting_pool.key().as_ref()],
        bump,
        space = 8 + GroupBet::MAX_SIZE,
    )]
    pub group_bet: Account<'info, GroupBet>,
    #[account(
        init,
        payer = user,
        seeds = [b"position", group_bet.key().as_ref(), user.key().as_ref()],
        bump,
        space = 8 + ParticipantPosition::MAX_SIZE,
    )]
    pub position: Account<'info, ParticipantPosition>,
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

pub fn handler(ctx: Context<AddGroupBet>, side: bool, amount: u64) -> Result<()> {
    let pool = &ctx.accounts.betting_pool;
    require!(pool.state == BetState::Active || pool.state == BetState::Created, BetError::InvalidState);

    let group = &mut ctx.accounts.group_bet;
    let position = &mut ctx.accounts.position;

    group.pool = pool.key();

    // Transfer group bet stake into the vault
    if amount == 0 {
        return Err(BetError::InsufficientContribution.into());
    }

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &ctx.accounts.vault.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    if side {
        group.total_on_a = group.total_on_a.saturating_add(amount);
        position.amount_on_a = position.amount_on_a.saturating_add(amount);
    } else {
        group.total_on_b = group.total_on_b.saturating_add(amount);
        position.amount_on_b = position.amount_on_b.saturating_add(amount);
    }

    position.owner = ctx.accounts.user.key();
    position.group_bet = group.key();

    ctx.accounts.betting_pool.total_pool = ctx
        .accounts
        .betting_pool
        .total_pool
        .checked_add(amount)
        .ok_or(crate::errors::BetError::InsufficientContribution)?;

    Ok(())
}
