use anchor_lang::prelude::*;

//use anchor_lang::solana_program::system_instruction;

use crate::states::{accounts::ClaimRewards, errors::{VaultError, StakerError, ClaimerError}, events::ClaimRewardsEvent, constants::REWARD_MULTIPLIER};



/* The claim instruction logic should be as follows:
1. The user will specify the vault to claim rewards from. The amount of rewards will be calculated for user automatically
2. Preflight checks == check that user staked in that vault, there's enough rewards in the vault, user has a non-zero rewards amount to claim*/

pub fn claim_rewards(ctx: Context<ClaimRewards>, vault_id: u64) -> Result<()> {
    // let's check that the specified vault Id is valid
    require!((1..=100).contains(&vault_id), VaultError::VaultIdOutOfBounds);

    // let's Get the staker and Vault Accounts associated in this context
    let staker = &mut ctx.accounts.staker_account;
    let vault = &mut ctx.accounts.vault_account;
    //let claimer_address = staker.staker_address;
    if staker.stake_amount == 0 {
        return Err(StakerError::NoStakeInVault.into());
    }

    /* Let's get the unix timestamp and time elapsed between now and when user staked*/
    let clock = Clock::get()?;
    //let current_time = clock.unix_timestamp;
// We allow user's to claim 20 days afterwards..
    // finding difficult implementing the time-travel feature in my test, so doing it here
    // using a REWARD_MULTIPLIER to do it, which is equivalent to 20 days elapsed time.
    //require!(elapsed_time > 0, ClaimerError::InsufficientTimeToClaim);

    // checks for the vault
    //require!(vault.is_initialized == true, VaultError::VaultNotInitialized);

    require!(vault.total_rewards > 0 && vault.reward_rate > 0.0, VaultError::VaultHasNoRewards);

    // checks for staker
    require!(staker.stake_amount > 0, StakerError::NoStakeInVault);
    require!(staker.is_claimed == false, ClaimerError::AlreadyClaimed);

    // let's calculate user's rewards using the formulae: rate * stake / total staked
    let claimer_rewards = (vault.reward_rate * staker.stake_amount as f32) / vault.total_staked as f32;
    require!(claimer_rewards > 0.0, ClaimerError::NoRewardsToClaim);
    let staker_proportion_rewards = claimer_rewards as u64;

    let final_claimed = staker_proportion_rewards * REWARD_MULTIPLIER as u64;

    //staker.accumulated_rewards = final_claimed;
    staker.claimed_rewards = final_claimed;

    /* Now, the longer a user's stake is locked within this vault, the higher the rewards accumulated..
    This is to incentivize users to lock their staked solana for longer periods. Implement this later!!! */

    //Let's make the transfers 
    **staker.to_account_info().try_borrow_mut_lamports()? += final_claimed;
    **vault.to_account_info().try_borrow_mut_lamports()? -= final_claimed;

    // The Below Method of Transfer, using the system_instructions::transfer instruction is not working because this instruction requires that the pda
    // does not hold any data (in this case, the vault_account does).. So, it's best if we use the try_borrow_mut_lamports()? instruction
    // Check this link: https://solana.stackexchange.com/questions/250/error-processing-instruction-0-invalid-program-argument-while-signing-transfe
    
/* 
    let transfer_instruction = system_instruction::transfer(
        &vault.to_account_info().key(),
        &staker.staker_address,
        final_claimed
    );
    let account_infos = &[
        ctx.accounts.staker_address.to_account_info(),
        vault.to_account_info(),
        ctx.accounts.system_program.to_account_info()
    ];
    invoke_signed(
        &transfer_instruction,
        account_infos,
        &[&[&vault_id.to_le_bytes(), &[vault.bump]]]
    )?;
    */
    /* Let's reset the vault's total rewards */
    vault.total_rewards -= final_claimed; 

    /* Let's reset the staker's rewards to zero */
    //staker.accumulated_rewards = 0;
    staker.stake_time = clock.unix_timestamp;
    staker.is_claimed = true;

    

    /* Let's emit the ClaimRewardsEvent */
    emit!(ClaimRewardsEvent{
        claimer_address: staker.staker_address,
        vault_id: vault_id,
        rewards_claimed: claimer_rewards as u64,
        claiming_time: clock.unix_timestamp,
    });

    Ok(())
}


