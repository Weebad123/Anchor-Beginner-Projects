use anchor_lang::prelude::*;


#[error_code]
pub enum VaultError {
    #[msg("Vault ID Must Be Within the Range of 1 to 128!!")]
    VaultIdOutOfBounds,
    #[msg("Vault Has Not Been Initialized Yet")]
    VaultNotInitialized,
    #[msg("Vault With Given ID Has Already Been Initialized")]
    VaultAlreadyInitialized,
    #[msg("Vault Can Only Be Initialized By The Vault Admin")]
    UnauthorizedVaultAdmin,
    #[msg("Vault Do Not Have Any Rewards")]
    VaultHasNoRewards,
    #[msg("Insufficient Staked Sol In Vault")]
    InsufficientVaultFunds,
}

#[error_code]
pub enum StakerError {
    #[msg("Stake Amount Cannot Be Zero!")]
    ZeroStakeAmount,
    #[msg("You do Not Have Any Stake In This Vault")]
    NoStakeInVault,
    #[msg("Staker Do Not Have Sufficient Funds To Stake")]
    InsufficientFunds,
}

#[error_code]
pub enum ClaimerError {
    #[msg("No Rewards To Claim In This Vault")]
    NoRewardsToClaim,
    #[msg("Staker Has Already Claimed")]
    AlreadyClaimed,
    #[msg("Give Sufficient Time Before Claiming!")]
    InsufficientTimeToClaim,
}

#[error_code]
pub enum DistributionError {
    #[msg("Distribution Duration Not Reached Or Elapsed!")]
    DistributionPassed,
    #[msg("Cannot Distribute Zero Reward Amount")]
    ZeroRewardAmount,
    #[msg("Not Enough Rewards To Transfer To Vault")]
    NotEnoughFunds,
    #[msg("Rewards Can Only Be Distributed By The Reward Authority")]
    OnlyRewardAuthority,
}


#[error_code]
pub enum WithdrawError {
    #[msg("Withdraw Amount Cannot Be Zero!")]
    ZeroWithdrawAmount,
    #[msg("Not Enough Staked Amount to Withdraw!")]
    InsufficientAmountToWithdraw,
}