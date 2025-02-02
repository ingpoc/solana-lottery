use anchor_lang::prelude::*;
use crate::state::lottery::{Lottery, LotteryState, LotteryType};
use crate::utils;

#[derive(Accounts)]
#[instruction(lottery_type: LotteryType)]
pub struct CreateLottery<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Lottery::SPACE,
        seeds = [b"lottery", lottery_type.discriminant().to_le_bytes().as_ref()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK: Validated in handler
    pub pyth_price_feed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl LotteryType {
    pub fn discriminant(&self) -> u64 {
        match self {
            LotteryType::Daily => 0,
            LotteryType::Weekly => 1,
            LotteryType::Monthly => 2,
        }
    }
}

pub fn handler(ctx: Context<CreateLottery>, lottery_type: LotteryType) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let clock = &ctx.accounts.clock;
    
    // Set basic lottery info
    lottery.id = clock.unix_timestamp as u64;
    lottery.lottery_type = lottery_type;
    lottery.ticket_price = lottery.get_ticket_price();
    lottery.min_pool_amount = lottery.get_min_pool_amount();
    lottery.current_pool_amount = 0;
    
    // Set time parameters
    lottery.start_time = clock.unix_timestamp;
    lottery.end_time = clock.unix_timestamp + lottery.get_duration();
    
    // Set initial state
    lottery.state = LotteryState::Created;
    lottery.bump = ctx.bumps.lottery;
    lottery.total_tickets = 0;
    lottery.pyth_price_account = ctx.accounts.pyth_price_feed.key();
    
    // Validate lottery parameters
    utils::validate_lottery_type(lottery_type, lottery.ticket_price)?;
    
    Ok(())
}