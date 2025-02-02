use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::lottery::{Lottery, LotteryState};
use crate::state::treasury::Treasury;
use crate::errors::LotteryError;

#[event]
pub struct PrizeDistributed {
    pub lottery_id: u64,
    pub prize_amount: u64,
    pub treasury_fee: u64,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct DistributePrize<'info> {
    #[account(
        mut,
        seeds = [b"lottery", lottery.lottery_type.discriminant().to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = lottery.state == LotteryState::Completed @ LotteryError::InvalidLotteryState,
        constraint = lottery.prize_claimed @ LotteryError::InvalidPrizeClaim
    )]
    pub lottery: Account<'info, Lottery>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(
        mut,
        constraint = lottery_token_account.owner == lottery.key()
    )]
    pub lottery_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_token_account.owner == treasury.key()
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<DistributePrize>) -> Result<()> {
    let clock = &ctx.accounts.clock;
    
    // Calculate remaining amount before mutating lottery
    let remaining_amount = {
        let lottery = &ctx.accounts.lottery;
        lottery.state_data.current_pool_amount
            .checked_sub(lottery.state_data.prize_amount)
            .ok_or(LotteryError::ArithmeticError)?
    };
    
    // Transfer to treasury
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.lottery_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.lottery.to_account_info(),
            },
        ),
        remaining_amount
    )?;
    
    // Update lottery state after transfer
    let lottery = &mut ctx.accounts.lottery;
    lottery.state = LotteryState::Expired;
    
    // Emit distribution event
    emit!(PrizeDistributed {
        lottery_id: lottery.id,
        prize_amount: lottery.state_data.prize_amount,
        treasury_fee: lottery.state_data.treasury_fee,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
} 