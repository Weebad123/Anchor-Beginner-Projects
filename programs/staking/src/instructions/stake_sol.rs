use anchor_lang::{prelude::*, solana_program::program::invoke_signed};

//use anchor_spl::token::{self, Token};
use anchor_lang::solana_program::system_instruction;

use crate::states::{accounts::StakerTokens, errors::StakerError, events::StakeSolEvent};



pub fn stake_sol(ctx: Context<StakerTokens>, vault_id: u64, stake_amount: u64) -> Result<()> {
    // Check to ensure amount to stake is non-zero
    require!(stake_amount > 0, StakerError::ZeroStakeAmount);
    // let's check that the specified vault_id is within range
    //require!((1..=100).contains(&vault_id), VaultError::VaultIdOutOfBounds); Not necessary: an outside Id will be caught with Account Not Initialized
    // Get the staker accounts and vault accounts
    let staker = &mut ctx.accounts.staker_account;

    let vault = &mut ctx.accounts.vault_account;


    let clock = Clock::get()?;

    /* Let's ensure the staker_address, the signer has sufficient funds */
    let staker_address = &mut ctx.accounts.staker_address.to_account_info();
    require!(**staker_address.to_account_info().lamports.borrow() >= stake_amount, StakerError::InsufficientFunds);
    // let's set the staker details
    staker.staker_address = *ctx.accounts.staker_address.key;
    // staker rewards is initially set to 0
    //staker.accumulated_rewards = 0;
    // Staker can continuously stake, so we gotta accumulate the amounts
    staker.stake_amount += stake_amount;
    // set staker start time to current timestamp
    staker.stake_time = clock.unix_timestamp;
    // Set staker's is_claimed to false
    staker.is_claimed = false;

    // let's increment the vault's total staked with the user's stake amount
    vault.total_staked += stake_amount;

    // Let's make the transfers below using the System transfer instruction
    let transfer_instruction = system_instruction::transfer(
        &staker_address.key,
        &vault.key(),
        stake_amount
    );

    let account_infos = &[
        staker_address.to_account_info(),
        vault.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ];

    invoke_signed(
        &transfer_instruction,
        account_infos,
        &[&[&vault_id.to_be_bytes(), &[vault.bump]]]
    )?;
    

    /* Let's emit the Stake Sol Event Listener */
    emit!(StakeSolEvent{
        staker: staker.staker_address,
        vault_id: vault_id,
        amount: stake_amount,
        staking_time: clock.unix_timestamp,
    });

    Ok(())
}