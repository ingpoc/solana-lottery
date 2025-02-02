use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::lottery::{Lottery, LotteryState};
use crate::errors::LotteryError;
use crate::utils;

#[event]
pub struct PrizeClaimed {
    pub lottery_id: u64,
    pub winner: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub matching_digits: u8,
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub winner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"lottery", lottery.lottery_type.discriminant().to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = !lottery.prize_claimed @ LotteryError::PrizeAlreadyClaimed,
        constraint = lottery.state == LotteryState::Completed @ LotteryError::InvalidLotteryState,
        constraint = !utils::is_claim_window_expired(lottery.last_draw_timestamp)? @ LotteryError::ClaimWindowExpired
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        mut,
        constraint = winner_token_account.owner == winner.key(),
        constraint = winner_token_account.mint == lottery_token_account.mint
    )]
    pub winner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = lottery_token_account.owner == lottery.key()
    )]
    pub lottery_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> ClaimPrize<'info> {
    pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.lottery_token_account.to_account_info(),
                to: self.winner_token_account.to_account_info(),
                authority: self.lottery.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<ClaimPrize>, user_numbers: [u8; 6]) -> Result<()> {
    let clock = &ctx.accounts.clock;
    
    // Calculate prize amount before mutating lottery
    let prize_amount = {
        let lottery = &ctx.accounts.lottery;
        let matching_digits = utils::count_matching_digits(&user_numbers, &lottery.winning_numbers);
        require!(matching_digits >= 3, LotteryError::NotWinner);
        utils::calculate_prize_amount(matching_digits, lottery.prize_amount)?
    };
    
    // Transfer prize to winner
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.lottery_token_account.to_account_info(),
                to: ctx.accounts.winner_token_account.to_account_info(),
                authority: ctx.accounts.lottery.to_account_info(),
            },
        ),
        prize_amount
    )?;
    
    // Update lottery state after transfer
    let lottery = &mut ctx.accounts.lottery;
    lottery.prize_claimed = true;
    lottery.winner = Some(ctx.accounts.winner.key());
    
    // Emit prize claim event
    emit!(PrizeClaimed {
        lottery_id: lottery.id,
        winner: ctx.accounts.winner.key(),
        amount: prize_amount,
        timestamp: clock.unix_timestamp,
        matching_digits,
    });

    Ok(())
}
