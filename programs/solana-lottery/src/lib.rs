use anchor_lang::prelude::*;

declare_id!("DRmPDrBUrF1R4Y7tdKRfjFKQPsdQdtvTEbQY5Qp9GzqY");

pub mod state;
pub mod errors;
pub mod utils;
pub mod instructions;

use crate::{
    instructions::{
        create_lottery::{self, CreateLottery},
        buy_ticket::{self, BuyTicket},
        schedule_draw::{self, ScheduleDraw},
        execute_draw::{self, ExecuteDraw},
        claim_prize::{self, ClaimPrize},
        distribute_prize::{self, DistributePrize},
        recycle_unclaimed::{self, RecycleUnclaimed},
        withdraw_treasury::{self, WithdrawTreasury},
    },
    state::LotteryType,
};

#[program]
pub mod solana_lottery {
    use super::*;

    pub fn create_lottery(ctx: Context<CreateLottery>, lottery_type: LotteryType) -> Result<()> {
        create_lottery::handler(ctx, lottery_type)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u8) -> Result<()> {
        buy_ticket::handler(ctx, amount)
    }

    pub fn schedule_draw(ctx: Context<ScheduleDraw>) -> Result<()> {
        schedule_draw::handler(ctx)
    }

    pub fn execute_draw(ctx: Context<ExecuteDraw>) -> Result<()> {
        execute_draw::handler(ctx)
    }

    pub fn claim_prize(ctx: Context<ClaimPrize>, user_numbers: [u8; 6]) -> Result<()> {
        claim_prize::handler(ctx, user_numbers)
    }

    pub fn distribute_prize(ctx: Context<DistributePrize>) -> Result<()> {
        distribute_prize::handler(ctx)
    }

    pub fn recycle_unclaimed(ctx: Context<RecycleUnclaimed>) -> Result<()> {
        recycle_unclaimed::handler(ctx)
    }

    pub fn withdraw_treasury(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
        withdraw_treasury::handler(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
