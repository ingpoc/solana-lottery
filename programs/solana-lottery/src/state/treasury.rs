use anchor_lang::prelude::*;
use crate::errors::LotteryError;

#[derive(Accounts)]
pub struct WithdrawTreasury<'info> {
    #[account(
        mut,
        has_one = authority @ LotteryError::Unauthorized,
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Treasury {
    pub balance: u64,
    pub fee_bps: u16,     // 250 for 2.5%
    pub authority: Pubkey,
    pub token_account: Pubkey,
    pub total_fees_collected: u64,
    pub last_withdrawal: i64,
    pub time_locked: i64,
    pub bump: u8,
}

impl Treasury {
    pub const SPACE: usize = 8 + // discriminator
        8 + // balance
        2 + // fee_bps
        32 + // authority
        32 + // token_account
        8 + // total_fees_collected
        8 + // last_withdrawal
        8 + // time_locked
        1; // bump

    pub fn collect_fees(&mut self, amount: u64) -> Result<u64> {
        let fee = (amount as u128)
            .checked_mul(self.fee_bps as u128)
            .ok_or(LotteryError::ArithmeticError)?
            .checked_div(10000)
            .ok_or(LotteryError::ArithmeticError)? as u64;

        self.balance = self.balance
            .checked_add(fee)
            .ok_or(LotteryError::ArithmeticError)?;

        self.total_fees_collected = self.total_fees_collected
            .checked_add(fee)
            .ok_or(LotteryError::ArithmeticError)?;

        Ok(fee)
    }

    pub fn withdraw(&mut self, amount: u64, clock: &Clock) -> Result<()> {
        require!(amount <= self.balance, LotteryError::InsufficientFunds);
        
        self.balance = self.balance
            .checked_sub(amount)
            .ok_or(LotteryError::ArithmeticError)?;
        
        self.last_withdrawal = clock.unix_timestamp;

        emit!(TreasuryWithdrawal {
            amount,
            authority: self.authority,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    pub fn is_authorized_signer(&self, signer: &Signer) -> bool {
        self.authority == signer.key()
    }
}

#[event]
pub struct TreasuryWithdrawal {
    pub amount: u64,
    pub authority: Pubkey,
    pub timestamp: i64,
}