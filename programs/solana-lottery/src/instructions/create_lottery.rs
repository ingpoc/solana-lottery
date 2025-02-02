use anchor_lang::prelude::*;
use crate::state::lottery::{Lottery, LotteryState, LotteryType, LotteryConfig, LotteryStateData, LotteryTiming};
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
    lottery.state = LotteryState::Created;
    lottery.bump = ctx.bumps.lottery;
    
    // Initialize config
    lottery.config = LotteryConfig {
        ticket_price: lottery.get_ticket_price(),
        min_pool_amount: lottery.get_min_pool_amount(),
        pyth_price_account: ctx.accounts.pyth_price_feed.key(),
    };
    
    // Initialize timing
    lottery.timing = LotteryTiming {
        start_time: clock.unix_timestamp,
        end_time: clock.unix_timestamp + lottery.get_duration(),
        last_draw_timestamp: 0,
    };
    
    // Initialize state data
    lottery.state_data = LotteryStateData {
        total_tickets: 0,
        current_pool_amount: 0,
        prize_amount: 0,
        treasury_fee: 0,
    };
    
    // Validate lottery parameters
    utils::validate_lottery_type(lottery_type, lottery.config.ticket_price)?;
    
    Ok(())
}