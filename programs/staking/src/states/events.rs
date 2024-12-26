use anchor_lang::prelude::*;

#[event]
pub struct InitializeVaultEvent {
    pub vault_id: u64,
    pub message: String,
    pub initializing_time: i64,
}

/* Let's emit the Stake Sol Event */
#[event]
pub struct StakeSolEvent {
    pub staker: Pubkey,
    pub vault_id: u64,
    pub amount: u64,
    pub staking_time: i64,
}


/* Let's emit the Distribute Rewards Event */
#[event]
pub struct DistributeRewardsEvent {
    pub distributer: Pubkey,
    pub vault_id: u64,
    pub reward_amount: u64,
    pub distribution_time: i64,
}


/* Let's emit the Claim Rewards Event */
#[event]
pub struct ClaimRewardsEvent {
    pub claimer_address: Pubkey,
    pub vault_id: u64,
    pub rewards_claimed: u64,
    pub claiming_time: i64,
}

/* Let's emit the Withdraw Sol Event */
#[event]
pub struct WithdrawSolEvent {
    pub withdrawer: Pubkey,
    pub vault_id: u64,
    pub withdrawal_amount: u64,
    pub withdrawal_time: i64,
}