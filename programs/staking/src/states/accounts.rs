use anchor_lang::prelude::*;
//use anchor_spl::token::spl_token::instruction;
//use anchor_spl::{mint, token::{TokenAccount, Mint, Token}};
use crate::states::errors::{VaultError, DistributionError};
//use crate::staking;



/* We first Need a Vault Account with fields like: Total Staked tokens and rewards in this vault, 
rewardRate, the vault's id, and a bump

.............VAULT ACCOUNT ............... */


#[account]
#[derive(InitSpace)]
pub struct VaultAccount {
    pub vault_id: u64,// space = 8
    pub total_staked: u64,// space = 8
    pub total_rewards: u64,// space = 8
    pub reward_rate: f32,// space = 8
    pub bump: u8,// space = 1
}


#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct InitializeVaultAccount<'info> {
    #[account(
        init,
        payer = vault_admin,
        seeds = [vault_id.to_le_bytes().as_ref()],
        space = 8 + VaultAccount::INIT_SPACE,
        bump
    )]
    pub vault_account: Account<'info, VaultAccount>,
    
    #[account(
        mut, 
        signer,
        constraint = vault_admin.key == expected_admin.key @ VaultError::UnauthorizedVaultAdmin
    )]
    pub vault_admin: Signer<'info>,
    /// CHECK: This is safe because we only compare the vault_admin key to this expected_admin just to be sure
    pub expected_admin: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}





/* Now, We need to store details for users who stake their tokens inside an account struct.
Contains publickey of user, stakeAmount, staking time ( to aid in reward calculation ), claimed_rewards */
#[account]
#[derive(InitSpace)]
pub struct StakerAccount {
    pub staker_address: Pubkey,// space = 32
    pub stake_amount: u64,// space = 8 
    pub stake_time: i64,// space = 8
    pub claimed_rewards: u64,// space = 8
    pub is_claimed: bool, // space = 1
}


#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct StakerTokens<'info> {
    #[account(
        init_if_needed,
        payer = staker_address,
        seeds = [staker_address.key().as_ref(), vault_id.to_le_bytes().as_ref()],
        space = 8 + StakerAccount::INIT_SPACE,
        bump,
    )]
    pub staker_account: Account<'info, StakerAccount>,
    #[account(mut)]
    pub staker_address: Signer<'info>, 
    #[account(mut, seeds = [vault_id.to_le_bytes().as_ref()], bump)]
    pub vault_account: Account<'info, VaultAccount>,
    pub system_program: Program<'info, System>
}



/* Now, let's create the Reward Distribution Struct */
#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct DistributeRewards<'info> {
    #[account(
        mut, 
        signer,
        constraint = rewarder_authority.key == expected_distributer.key @ DistributionError::OnlyRewardAuthority
    )]
    pub rewarder_authority: Signer<'info>,
    /// CHECK: This is safe because we are only comparing the signer or caller to the expected Distributer, which is the reward Authority
    pub expected_distributer: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [vault_id.to_le_bytes().as_ref()],
        bump
    )]
    pub vault_account: Account<'info, VaultAccount>,
    pub system_program: Program<'info, System>
}



/* Let's create the Structure for the ClaimRewards 
1. The caller will claim from a vault
2. There should be a reference to the staker's and vault's accounts*/
#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        has_one = staker_address,
        seeds = [staker_address.key.as_ref(), vault_id.to_le_bytes().as_ref()],
        bump
    )]
    pub staker_account: Account<'info, StakerAccount>,
    #[account(mut, signer)]
    pub staker_address: Signer<'info>,
    #[account(
        mut,
        seeds = [vault_id.to_le_bytes().as_ref()],
        bump
    )]
    pub vault_account: Account<'info, VaultAccount>,
    pub system_program: Program<'info, System>
}


/* Let's create the structure for the Withdraw Tokens
1. The staker is the one who is supposed to be able to withdraw staked sol
2. There should be a reference to the Staker and Vault accounts. */
#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct WithdrawTokens<'info> {
    #[account(
        mut,
        seeds = [staker_address.key.as_ref(), vault_id.to_le_bytes().as_ref()],
        bump
    )]
    pub staker_account: Account<'info, StakerAccount>,
    #[account(mut, signer)]
    pub staker_address: Signer<'info>,
    #[account(
        mut,
        seeds = [vault_id.to_le_bytes().as_ref()],
        bump
    )]
    pub vault_account: Account<'info, VaultAccount>
}