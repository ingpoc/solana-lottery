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

// Split into smaller functions to reduce stack usage
fn calculate_prize_distribution(total_pool: u64) -> Result<(u64, u64)> {
    let prize_amount = (total_pool * 90) / 100;
    let treasury_fee = total_pool.checked_sub(prize_amount)
        .ok_or(LotteryError::ArithmeticError)?;
    Ok((prize_amount, treasury_fee))
}

pub fn handler(ctx: Context<ExecuteDraw>) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let lottery = &mut ctx.accounts.lottery;
    
    // Generate winning numbers using Pyth price feed
    let winning_numbers = utils::generate_random_number(&ctx.accounts.pyth_price_feed)?;
    lottery.winning_numbers = winning_numbers;
    
    // Update lottery state
    lottery.state = LotteryState::Completed;
    lottery.timing.last_draw_timestamp = clock.unix_timestamp;
    
    // Calculate prize distribution
    let total_pool = lottery.state_data.current_pool_amount;
    let (prize_amount, treasury_fee) = calculate_prize_distribution(total_pool)?;
    lottery.state_data.prize_amount = prize_amount;
    lottery.state_data.treasury_fee = treasury_fee;
    
    // Emit draw executed event
    emit!(DrawExecuted {
        lottery_id: lottery.id,
        winning_numbers,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}