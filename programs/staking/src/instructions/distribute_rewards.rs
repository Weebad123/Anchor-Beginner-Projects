use anchor_lang::{prelude::*,solana_program::program::invoke_signed};

use anchor_lang::solana_program::system_instruction;


use crate::states::{accounts::DistributeRewards, errors::DistributionError, constants::REWARD_DURATION, events::DistributeRewardsEvent};


pub fn distribute_rewards(ctx: Context<DistributeRewards>, vault_id: u64, reward_amount: u64) -> Result<()> {
    // check to ensure a non-zero reward_amount
    require!(reward_amount > 0, DistributionError::ZeroRewardAmount);
    // Check that vault_id is valid
    //require!((1..=100).contains(&vault_id), VaultError::VaultIdOutOfBounds); Not necessary: Will be caught with Vault Account Not Initialized

    let vault = &mut ctx.accounts.vault_account;

    // let's attempt to calculate the Reward Rate here, and then set it to be the vault's rate
    let reward_rate = reward_amount as f32 / REWARD_DURATION as f32;
    // let's set this reward rate to that of the vault
    vault.reward_rate = reward_rate;

    // Now, let's ensure that the rewarder authority has enough funds to transfer to this vault, before the actual transfer
    let rewarder_authority = &mut ctx.accounts.rewarder_authority;
    require!(**rewarder_authority.to_account_info().lamports.borrow() >= reward_amount, DistributionError::NotEnoughFunds);

    /* Let's make the actual transfer from The Rewarder Authority to the Vault Using The system_instruction's transfer  */
    
    let transfer_instruction = system_instruction::transfer(
        &rewarder_authority.key,
        &vault.key(),
        reward_amount
    );

    let account_infos = &[
        rewarder_authority.to_account_info(),
        vault.to_account_info(),
        ctx.accounts.system_program.to_account_info()
    ];

    invoke_signed(
        &transfer_instruction,
        account_infos,
        &[&[&vault_id.to_le_bytes(), &[vault.bump]]]
    )?;

    // let's update the totalRewards associated with this vault
    vault.total_rewards += reward_amount;

    /* Let's get the unix timestamp */
    let clock = Clock::get()?;

    /* Let's emit the distribute Rewards Event Listener */
    emit!(DistributeRewardsEvent{
        distributer: *rewarder_authority.key,
        vault_id: vault_id,
        reward_amount: reward_amount,
        distribution_time: clock.unix_timestamp,
    });

    Ok(())
}