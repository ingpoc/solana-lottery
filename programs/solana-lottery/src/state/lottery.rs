use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum LotteryState {
    #[default]
    Created,
    Open,
    Drawing,
    Completed,
    Expired
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum LotteryType {
    #[default]
    Daily,   // 1 USDC, 24h, min 100 USDC
    Weekly,  // 5 USDC, 7d, min 500 USDC
    Monthly  // 10 USDC, 30d, min 1000 USDC
}

// Split into smaller components to reduce stack usage
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LotteryConfig {
    pub ticket_price: u64,
    pub min_pool_amount: u64,
    pub pyth_price_account: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LotteryStateData {
    pub total_tickets: u64,
    pub current_pool_amount: u64,
    pub prize_amount: u64,
    pub treasury_fee: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LotteryTiming {
    pub start_time: i64,
    pub end_time: i64,
    pub last_draw_timestamp: i64,
}

#[account]
#[derive(Default)]
pub struct Lottery {
    pub id: u64,
    pub lottery_type: LotteryType,
    pub state: LotteryState,
    pub bump: u8,
    pub config: LotteryConfig,
    pub timing: LotteryTiming,
    pub state_data: LotteryStateData,
    pub winner_ticket: Option<u64>,
    pub winner: Option<Pubkey>,
    pub prize_claimed: bool,
    pub winning_numbers: [u8; 6],
}

impl Lottery {
    pub const SPACE: usize = 8 + // discriminator
        8 + // id
        1 + // lottery_type
        1 + // state
        1 + // bump
        (8 + 8 + 32) + // config
        (8 + 8 + 8) + // timing
        (8 + 8 + 8 + 8) + // state_data
        9 + // winner_ticket (Option)
        33 + // winner (Option<Pubkey>)
        1 + // prize_claimed
        6; // winning_numbers

    pub fn get_min_pool_amount(&self) -> u64 {
        match self.lottery_type {
            LotteryType::Daily => 100_000_000, // 100 USDC
            LotteryType::Weekly => 500_000_000, // 500 USDC
            LotteryType::Monthly => 1_000_000_000, // 1000 USDC
        }
    }

    pub fn get_ticket_price(&self) -> u64 {
        match self.lottery_type {
            LotteryType::Daily => 1_000_000, // 1 USDC
            LotteryType::Weekly => 5_000_000, // 5 USDC
            LotteryType::Monthly => 10_000_000, // 10 USDC
        }
    }

    pub fn get_duration(&self) -> i64 {
        match self.lottery_type {
            LotteryType::Daily => 24 * 60 * 60, // 24 hours
            LotteryType::Weekly => 7 * 24 * 60 * 60, // 7 days
            LotteryType::Monthly => 30 * 24 * 60 * 60, // 30 days
        }
    }
}