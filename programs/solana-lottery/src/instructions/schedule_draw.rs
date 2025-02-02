use anchor_lang::prelude::*;
use crate::state::lottery::{Lottery, LotteryState};
use crate::errors::LotteryError;

#[event]
pub struct DrawScheduled {
    pub lottery_id: u64,
    pub total_tickets: u64,
    pub timestamp: i64,
    pub min_pool_reached: bool,
}

#[derive(Accounts)]
pub struct ScheduleDraw<'info> {
    #[account(
        mut,
        seeds = [b"lottery", lottery.lottery_type.discriminant().to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = lottery.state == LotteryState::Open @ LotteryError::InvalidLotteryState,
        constraint = Clock::get()?.unix_timestamp >= lottery.end_time @ LotteryError::InvalidTimeRange,
        constraint = lottery.total_tickets > 0 @ LotteryError::MinPoolNotReached
    )]
    pub lottery: Account<'info, Lottery>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ScheduleDraw>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let clock = &ctx.accounts.clock;
    
    // Check if minimum pool amount is reached
    let min_pool_reached = lottery.current_pool_amount >= lottery.min_pool_amount;
    require!(min_pool_reached, LotteryError::MinPoolNotReached);
    
    // Update lottery state
    lottery.state = LotteryState::Drawing;
    lottery.last_draw_timestamp = clock.unix_timestamp;

    // Emit draw scheduled event
    emit!(DrawScheduled {
        lottery_id: lottery.id,
        total_tickets: lottery.total_tickets,
        timestamp: lottery.last_draw_timestamp,
        min_pool_reached,
    });

    Ok(())
} 