use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::lottery::{Lottery, LotteryState, LotteryType};
use crate::state::treasury::Treasury;
use crate::errors::LotteryError;
use crate::utils;

#[event]
pub struct LotteryRecycled {
    pub lottery_id: u64,
    pub unclaimed_amount: u64,
    pub new_end_time: i64,
    pub timestamp: i64,
    pub lottery_type: LotteryType,
}

#[derive(Accounts)]
pub struct RecycleUnclaimed<'info> {
    #[account(
        mut,
        seeds = [b"lottery", lottery.lottery_type.discriminant().to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = lottery.state == LotteryState::Completed @ LotteryError::InvalidLotteryState,
        constraint = !lottery.prize_claimed @ LotteryError::PrizeAlreadyClaimed,
        constraint = utils::is_claim_window_expired(lottery.last_draw_timestamp)? @ LotteryError::ClaimWindowExpired
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

pub fn handler(ctx: Context<RecycleUnclaimed>) -> Result<()> {
    let clock = &ctx.accounts.clock;
    
    // Calculate unclaimed amount before mutating lottery
    let unclaimed_amount = {
        let lottery = &ctx.accounts.lottery;
        lottery.current_pool_amount
    };
    
    // Transfer unclaimed funds to treasury
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.lottery_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.lottery.to_account_info(),
            },
        ),
        unclaimed_amount
    )?;
    
    // Update lottery state after transfer
    let lottery = &mut ctx.accounts.lottery;
    let _treasury = &mut ctx.accounts.treasury;
    
    // Reset lottery for new round
    lottery.state = LotteryState::Created;
    lottery.total_tickets = 0;
    lottery.winner = None;
    lottery.winner_ticket = None;
    lottery.prize_claimed = false;
    lottery.prize_amount = 0;
    lottery.treasury_fee = 0;
    lottery.current_pool_amount = 0;
    lottery.start_time = clock.unix_timestamp;
    lottery.end_time = clock.unix_timestamp + lottery.get_duration();
    
    // Emit recycle event
    emit!(LotteryRecycled {
        lottery_id: lottery.id,
        unclaimed_amount,
        new_end_time: lottery.end_time,
        timestamp: clock.unix_timestamp,
        lottery_type: lottery.lottery_type,
    });

    Ok(())
}
