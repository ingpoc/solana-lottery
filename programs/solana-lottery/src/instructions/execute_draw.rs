use anchor_lang::prelude::*;
use crate::state::lottery::{Lottery, LotteryState};
use crate::state::treasury::Treasury;
use crate::errors::LotteryError;
use crate::utils;

#[event]
pub struct DrawExecuted {
    pub lottery_id: u64,
    pub timestamp: i64,
    pub participants: u64,
    pub prize_amount: u64,
    pub treasury_fee: u64,
    pub winning_numbers: [u8; 6],
}

#[derive(Accounts)]
pub struct ExecuteDraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"lottery", lottery.lottery_type.discriminant().to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = lottery.state == LotteryState::Drawing @ LotteryError::InvalidLotteryState
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK: Used for randomness source
    #[account(
        constraint = pyth_price_feed.key() == lottery.pyth_price_account @ LotteryError::InvalidPythFeed
    )]
    pub pyth_price_feed: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ExecuteDraw>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let treasury = &mut ctx.accounts.treasury;
    let clock = &ctx.accounts.clock;
    
    // Generate winning numbers using Pyth price feed
    let winning_numbers = utils::generate_random_number(&ctx.accounts.pyth_price_feed)?;
    
    // Calculate prize pool and fees
    let prize_pool = lottery.current_pool_amount;
    let fee = treasury.collect_fees(prize_pool)?;
    let prize_amount = prize_pool.checked_sub(fee).ok_or(LotteryError::ArithmeticError)?;
    
    // Update lottery state
    lottery.state = LotteryState::Completed;
    lottery.prize_amount = prize_amount;
    lottery.treasury_fee = fee;
    lottery.winning_numbers = winning_numbers;
    
    // Emit draw executed event
    emit!(DrawExecuted {
        lottery_id: lottery.id,
        timestamp: clock.unix_timestamp,
        participants: lottery.total_tickets,
        prize_amount,
        treasury_fee: fee,
        winning_numbers,
    });

    Ok(())
}