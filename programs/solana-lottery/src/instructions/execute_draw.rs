use anchor_lang::prelude::*;
use crate::state::lottery::{Lottery, LotteryState};
use crate::errors::LotteryError;
use crate::utils;

#[event]
pub struct DrawExecuted {
    pub lottery_id: u64,
    pub winning_numbers: [u8; 6],
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct ExecuteDraw<'info> {
    #[account(
        mut,
        seeds = [b"lottery", lottery.lottery_type.discriminant().to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = lottery.state == LotteryState::Drawing @ LotteryError::InvalidLotteryState
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK: Validated in handler
    pub pyth_price_feed: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ExecuteDraw>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let clock = &ctx.accounts.clock;
    
    // Generate winning numbers using Pyth price feed
    lottery.winning_numbers = utils::generate_random_number(&ctx.accounts.pyth_price_feed)?;
    
    // Update lottery state
    lottery.state = LotteryState::Completed;
    lottery.timing.last_draw_timestamp = clock.unix_timestamp;
    
    // Calculate prize amount (90% of pool)
    let total_pool = lottery.state_data.current_pool_amount;
    lottery.state_data.prize_amount = (total_pool * 90) / 100;
    lottery.state_data.treasury_fee = total_pool - lottery.state_data.prize_amount;
    
    // Emit draw executed event
    emit!(DrawExecuted {
        lottery_id: lottery.id,
        winning_numbers: lottery.winning_numbers,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}