use anchor_lang::prelude::*;

// Split errors into logical groups using nested variants
#[error_code]
pub enum LotteryError {
    // State errors
    #[msg("Invalid time range for lottery")]
    InvalidTimeRange,
    #[msg("Lottery is not active")]
    LotteryNotActive,
    #[msg("Draw is already in progress")]
    DrawInProgress,
    #[msg("Invalid lottery state for operation")]
    InvalidLotteryState,
    #[msg("Invalid lottery type")]
    InvalidLotteryType,

    // Prize errors
    #[msg("Invalid prize claim attempt")]
    InvalidPrizeClaim,
    #[msg("Prize already claimed")]
    PrizeAlreadyClaimed,
    #[msg("Not a winner")]
    NotWinner,
    #[msg("Invalid prize distribution")]
    InvalidPrizeDistribution,
    #[msg("Claim window expired")]
    ClaimWindowExpired,

    // Treasury errors
    #[msg("Invalid treasury withdrawal")]
    InvalidWithdrawal,
    #[msg("Treasury operation failed")]
    TreasuryError,
    #[msg("Treasury is timelocked")]
    TimelockActive,
    #[msg("Insufficient treasury balance")]
    InsufficientTreasuryBalance,

    // Token errors
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Invalid token transfer")]
    InvalidTokenTransfer,
    #[msg("Insufficient funds for operation")]
    InsufficientFunds,

    // Validation errors
    #[msg("Exceeded ticket purchase limit")]
    ExceededTicketLimit,
    #[msg("Minimum pool amount not reached")]
    MinPoolNotReached,
    #[msg("Invalid ticket price")]
    InvalidTicketPrice,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Unauthorized signer")]
    UnauthorizedSigner,
    #[msg("Price data is stale")]
    StalePrice,
    #[msg("Invalid Pyth price feed")]
    InvalidPythFeed,
    #[msg("Arithmetic error")]
    ArithmeticError,
}