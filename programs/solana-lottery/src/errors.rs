use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryError {
    #[msg("Invalid time range for lottery")]
    InvalidTimeRange,
    #[msg("Insufficient funds for operation")]
    InsufficientFunds,
    #[msg("Lottery is not active")]
    LotteryNotActive,
    #[msg("Draw is already in progress")]
    DrawInProgress,
    #[msg("Price data is stale")]
    StalePrice,
    #[msg("Invalid prize claim attempt")]
    InvalidPrizeClaim,
    #[msg("Exceeded ticket purchase limit")]
    ExceededTicketLimit,
    #[msg("Invalid lottery state for operation")]
    InvalidLotteryState,
    #[msg("Minimum pool amount not reached")]
    MinPoolNotReached,
    #[msg("Invalid ticket price")]
    InvalidTicketPrice,
    #[msg("Invalid lottery type")]
    InvalidLotteryType,
    #[msg("Prize already claimed")]
    PrizeAlreadyClaimed,
    #[msg("Not a winner")]
    NotWinner,
    #[msg("Invalid treasury withdrawal")]
    InvalidWithdrawal,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Arithmetic error")]
    ArithmeticError,
    #[msg("Invalid Pyth price feed")]
    InvalidPythFeed,
    #[msg("Invalid prize distribution")]
    InvalidPrizeDistribution,
    #[msg("Claim window expired")]
    ClaimWindowExpired,
    #[msg("Treasury operation failed")]
    TreasuryError,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Invalid token transfer")]
    InvalidTokenTransfer,
    #[msg("Unauthorized signer")]
    UnauthorizedSigner,
    #[msg("Treasury is timelocked")]
    TimelockActive,
    #[msg("Insufficient treasury balance")]
    InsufficientTreasuryBalance,
}