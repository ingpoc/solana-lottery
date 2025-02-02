use anchor_lang::prelude::*;

declare_id!("DRmPDrBUrF1R4Y7tdKRfjFKQPsdQdtvTEbQY5Qp9GzqY");

pub mod state;
pub mod errors;
pub mod utils;
pub mod instructions;


#[program]
pub mod solana_lottery {
    use super::*;

    pub fn create_lottery(ctx: Context<CreateLottery>, lottery_type: LotteryType) -> Result<()> {
        instructions::create_lottery::handler(ctx, lottery_type)
    }

    pub fn buy_ticket(
        ctx: Context<BuyTicket>,
        amount: u8
    ) -> Result<()> {
        instructions::buy_ticket::handler(ctx, amount)
    }

    pub fn schedule_draw(ctx: Context<ScheduleDraw>) -> Result<()> {
        instructions::schedule_draw::handler(ctx)
    }

    pub fn execute_draw(ctx: Context<ExecuteDraw>) -> Result<()> {
        instructions::execute_draw::handler(ctx)
    }

    pub fn claim_prize(ctx: Context<ClaimPrize>, user_numbers: [u8; 6]) -> Result<()> {
        instructions::claim_prize::handler(ctx, user_numbers)
    }

    pub fn distribute_prize(ctx: Context<DistributePrize>) -> Result<()> {
        instructions::distribute_prize::handler(ctx)
    }

    pub fn recycle_unclaimed(ctx: Context<RecycleUnclaimed>) -> Result<()> {
        instructions::recycle_unclaimed::handler(ctx)
    }

    pub fn withdraw_treasury(ctx: Context<instructions::withdraw_treasury::WithdrawTreasury>, amount: u64) -> Result<()> {
        withdraw_treasury_handler(ctx, amount)
    }

}

#[derive(Accounts)]
pub struct Initialize {}
