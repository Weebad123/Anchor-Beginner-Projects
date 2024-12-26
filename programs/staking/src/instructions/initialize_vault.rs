

use anchor_lang::prelude::*;

use crate::states::{accounts::InitializeVaultAccount, errors::VaultError, events::InitializeVaultEvent};


/* Some Checklist.
1. Only the vault admin can initialize a vault
2. The vault cannot be initialized twice
3.  */
pub fn initialize_vault(ctx: Context<InitializeVaultAccount>, vault_id: u64) -> Result<()> {
    // Get a mutable borrow of the vault
    let vault = &mut ctx.accounts.vault_account;
    // i'd like to ensure that the vault Id is within a range of 1  to 128
    if !(1..=100).contains(&vault_id) {
        return Err(error!(VaultError::VaultIdOutOfBounds));
    }
    // let's set the parameters
    //require!(vault.is_initialized == false, VaultError::VaultAlreadyInitialized);
    vault.vault_id = vault_id;
    vault.total_staked = 0;
    vault.total_rewards = 0;
    vault.bump = ctx.bumps.vault_account;

    /* Let's get the Unix Timestamp */
    let clock = Clock::get()?;

    // Let's emit the InitializeVaultEvent
    let message = format!("The vault with vault_id of {} has been been initialized", vault_id);
    emit!(InitializeVaultEvent {
        vault_id: vault_id,
        message: message,
        initializing_time: clock.unix_timestamp,
    });
    Ok(())
}