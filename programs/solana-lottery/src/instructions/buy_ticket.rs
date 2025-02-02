use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::lottery::{Lottery, LotteryState};
use crate::errors::LotteryError;
use crate::utils;

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"lottery", lottery.lottery_type.discriminant().to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = lottery.state == LotteryState::Open @ LotteryError::LotteryNotActive,
        constraint = Clock::get()?.unix_timestamp < lottery.end_time @ LotteryError::LotteryNotActive
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key(),
        constraint = buyer_token_account.mint == lottery_token_account.mint
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = lottery_token_account.owner == lottery.key()
    )]
    pub lottery_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<BuyTicket>, amount: u8) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    
    // Validate ticket purchase
    utils::validate_ticket_purchase(amount)?;
    
    // Calculate total cost
    let total_cost = (lottery.ticket_price as u128)
        .checked_mul(amount as u128)
        .ok_or(LotteryError::ArithmeticError)? as u64;
    
    // Transfer USDC tokens
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_token_account.to_account_info(),
                to: ctx.accounts.lottery_token_account.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        total_cost
    )?;
    
    // Update lottery state
    lottery.total_tickets = lottery.total_tickets
        .checked_add(amount as u64)
        .ok_or(LotteryError::ArithmeticError)?;
    
    lottery.current_pool_amount = lottery.current_pool_amount
        .checked_add(total_cost)
        .ok_or(LotteryError::ArithmeticError)?;

    Ok(())
}