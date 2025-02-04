use anchor_lang::prelude::*;
use crate::errors::LotteryError;
use crate::state::lottery::LotteryType;
use sha2::{Sha256, Digest};

pub const MAX_TICKETS_PER_TX: u8 = 5;
pub const CLAIM_WINDOW: i64 = 14 * 24 * 60 * 60; // 14 days
pub const PRIZE_TIERS: [u8; 4] = [60, 25, 10, 5]; // Percentages for 6, 5, 4, 3 matching digits

#[inline]
pub fn get_current_timestamp() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp)
}

// Split random number generation into smaller functions
fn hash_data(data: &[u8], time: i64, slot: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&data[..24]);
    hasher.update(time.to_le_bytes());
    hasher.update(slot.to_le_bytes());
    hasher.finalize().into()
}

fn convert_hash_to_numbers(hash: &[u8; 32]) -> [u8; 6] {
    let mut numbers = [0u8; 6];
    for i in 0..6 {
        numbers[i] = (hash[i] % 10) as u8;
    }
    numbers
}

pub fn generate_random_number(price_feed: &AccountInfo) -> Result<[u8; 6]> {
    let current_time = Clock::get()?.unix_timestamp;
    let current_slot = Clock::get()?.slot;
    
    let data = price_feed.try_borrow_data()?;
    let hash = hash_data(&data, current_time, current_slot);
    Ok(convert_hash_to_numbers(&hash))
}

#[inline]
pub fn calculate_prize_amount(matching_digits: u8, total_pool: u64) -> Result<u64> {
    require!(matching_digits >= 3 && matching_digits <= 6, LotteryError::InvalidPrizeDistribution);
    
    let tier_index = (6 - matching_digits) as usize;
    let percentage = PRIZE_TIERS[tier_index];
    
    let amount = (total_pool as u128)
        .checked_mul(percentage as u128)
        .ok_or(LotteryError::ArithmeticError)?
        .checked_div(100)
        .ok_or(LotteryError::ArithmeticError)? as u64;
    
    Ok(amount)
}

#[inline]
pub fn validate_ticket_purchase(num_tickets: u8) -> Result<()> {
    require!(
        num_tickets > 0 && num_tickets <= MAX_TICKETS_PER_TX,
        LotteryError::ExceededTicketLimit
    );
    Ok(())
}

#[inline]
pub fn validate_lottery_type(lottery_type: LotteryType, ticket_price: u64) -> Result<()> {
    let expected_price = match lottery_type {
        LotteryType::Daily => 1_000_000, // 1 USDC
        LotteryType::Weekly => 5_000_000, // 5 USDC
        LotteryType::Monthly => 10_000_000, // 10 USDC
    };
    
    require!(ticket_price == expected_price, LotteryError::InvalidTicketPrice);
    Ok(())
}

#[inline]
pub fn validate_min_pool(lottery_type: LotteryType, current_pool: u64) -> Result<()> {
    let min_pool = match lottery_type {
        LotteryType::Daily => 100_000_000, // 100 USDC
        LotteryType::Weekly => 500_000_000, // 500 USDC
        LotteryType::Monthly => 1_000_000_000, // 1000 USDC
    };
    
    require!(current_pool >= min_pool, LotteryError::MinPoolNotReached);
    Ok(())
}

#[inline]
pub fn count_matching_digits(user_numbers: &[u8; 6], winning_numbers: &[u8; 6]) -> u8 {
    user_numbers.iter()
        .zip(winning_numbers.iter())
        .filter(|(a, b)| a == b)
        .count() as u8
}

#[inline]
pub fn is_claim_window_expired(draw_timestamp: i64) -> Result<bool> {
    let current_time = get_current_timestamp()?;
    Ok(current_time > draw_timestamp + CLAIM_WINDOW)
}
