use anchor_lang::prelude::*;


use crate::states::{accounts::*, errors::*, events::WithdrawSolEvent};


/* Now, let's write the instruction for the withdraw Logic
1. Arguments include: specify vault_id, staked amount to withdraw*/

pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, vault_id: u64, withdraw_amount: u64) -> Result<()> {
    // let's check for valid vault_id and non-zero withdraw amount
    require!((1..=100).contains(&vault_id), VaultError::VaultIdOutOfBounds);
    require!(withdraw_amount > 0, WithdrawError::ZeroWithdrawAmount);

    // let's get the staker and vault accounts
    let staker = &mut ctx.accounts.staker_account;
    let vault = &mut ctx.accounts.vault_account;

    /* Validate Sufficient Staked amount in both Vault and User's  */
    require!(staker.stake_amount >= withdraw_amount, WithdrawError::InsufficientAmountToWithdraw);
    require!(vault.total_staked >= withdraw_amount, VaultError::InsufficientVaultFunds);
    
    /* update Vault and Staker state */
    staker.stake_amount -= withdraw_amount;
    //staker.stake_amount = staker.stake_amount.checked_sub(withdraw_amount).ok_or(StakerError::InsufficientFunds)?;
    vault.total_staked = vault.total_staked.checked_sub(withdraw_amount).ok_or(VaultError::InsufficientVaultFunds)?;

    /* Now, let's make the actual transfers from the vault to the Staker */
    **staker.to_account_info().try_borrow_mut_lamports()? += withdraw_amount;
    **vault.to_account_info().try_borrow_mut_lamports()? -= withdraw_amount;

    /* let's get the unix timestamp */
    let clock = Clock::get()?;

    /* Let's emit the Withdraw Sol Event */
    emit!(WithdrawSolEvent {
        withdrawer: *ctx.accounts.staker_address.key,
        vault_id: vault_id,
        withdrawal_amount: withdraw_amount,
        withdrawal_time: clock.unix_timestamp,
    });

    Ok(())
}