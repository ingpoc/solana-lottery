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
        constraint = Clock::get()?.unix_timestamp >= lottery.timing.end_time @ LotteryError::InvalidTimeRange,
        constraint = lottery.state_data.total_tickets > 0 @ LotteryError::MinPoolNotReached
    )]
    pub lottery: Account<'info, Lottery>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ScheduleDraw>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let clock = &ctx.accounts.clock;
    
    // Update lottery state
    lottery.state = LotteryState::Drawing;
    
    // Emit draw scheduled event
    emit!(DrawScheduled {
        lottery_id: lottery.id,
        total_tickets: lottery.state_data.total_tickets,
        timestamp: clock.unix_timestamp,
        min_pool_reached: lottery.state_data.current_pool_amount >= lottery.config.min_pool_amount,
    });
    
    Ok(())
} 