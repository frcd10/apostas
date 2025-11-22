pub mod state;
pub mod instructions;
pub mod errors;

use anchor_lang::prelude::*;

use instructions::*;

// Program ID sincronizado pelo Anchor
declare_id!("EbyYk2bRFQBWkdJdoxc6ALiVkQsGNkZxPGNMtpHR1KQz");

#[program]
mod betting_protocol {
    use super::*;

    pub fn initialize_bet(ctx: Context<InitializeBet>, arbiter: Pubkey, bet_amount: u64, lock_period: i64) -> Result<()> {
        instructions::initialize_bet::handler(ctx, arbiter, bet_amount, lock_period)
    }

    pub fn join_bet(ctx: Context<JoinBet>) -> Result<()> {
        instructions::join_bet::handler(ctx)
    }

    pub fn add_group_bet(ctx: Context<AddGroupBet>, side: bool, amount: u64) -> Result<()> {
        instructions::add_group_bet::handler(ctx, side, amount)
    }

    pub fn declare_winner(ctx: Context<DeclareWinner>, winner: Pubkey) -> Result<()> {
        instructions::declare_winner::handler(ctx, winner)
    }

    pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
        instructions::claim_prize::handler(ctx)
    }

    pub fn emergency_cancel(ctx: Context<EmergencyCancel>) -> Result<()> {
        instructions::emergency_cancel::handler(ctx)
    }
}
