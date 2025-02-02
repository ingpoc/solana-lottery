use anchor_lang::prelude::*;
use crate::state::treasury::Treasury;
use crate::errors::LotteryError;
use crate::utils::get_current_timestamp;

#[derive(Accounts)]
pub struct WithdrawTreasury<'info> {
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
        constraint = treasury.is_authorized_signer(&additional_signer) @ LotteryError::UnauthorizedSigner,
        constraint = get_current_timestamp()? > treasury.time_locked @ LotteryError::TimelockActive
    )]
    pub treasury: Account<'info, Treasury>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Safe because we're transferring to this account
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
    
    /// Required signers for multisig
    pub additional_signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
    require!(
        amount <= ctx.accounts.treasury.balance,
        LotteryError::InsufficientTreasuryBalance
    );
    
    let treasury = &mut ctx.accounts.treasury;
    let destination = &mut ctx.accounts.destination;
    
    // Update timelock for next withdrawal
    treasury.time_locked = get_current_timestamp()? + 24 * 60 * 60; // 24 hours
    
    // Transfer lamports
    **treasury.to_account_info().try_borrow_mut_lamports()? = treasury
        .to_account_info()
        .lamports()
        .checked_sub(amount)
        .ok_or(LotteryError::ArithmeticError)?;

    **destination.to_account_info().try_borrow_mut_lamports()? = destination
        .to_account_info()
        .lamports()
        .checked_add(amount)
        .ok_or(LotteryError::ArithmeticError)?;

    // Update treasury state
    treasury.balance = treasury.balance
        .checked_sub(amount)
        .ok_or(LotteryError::ArithmeticError)?;
    treasury.last_withdrawal = get_current_timestamp()?;
    
    // Emit withdrawal event
    emit!(TreasuryWithdrawal {
        amount,
        authority: ctx.accounts.authority.key(),
        timestamp: treasury.last_withdrawal
    });
    
    Ok(())
}

#[event]
pub struct TreasuryWithdrawal {
    amount: u64,
    authority: Pubkey,
    timestamp: i64,
}
