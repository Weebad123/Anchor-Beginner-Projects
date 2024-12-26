
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

use anchor_lang::prelude::*;

declare_id!("AKeY1R1UbGMVYTCCaeMtaUzin9AMW6Ng6uxWbkcjrqN1");

#[program]
pub mod staking {
    use super::*;

        /*  ........ VAULT   INITIALIZATION    */
    pub fn initialize(ctx: Context<InitializeVaultAccount>, vault_id: u64) -> Result<()> {
        //msg!("Greetings from: {:?}", ctx.program_id);
        initialize_vault(ctx, vault_id)?;
        Ok(())
    }


        /*  ..........USER       STAKING   ........  */
    pub fn stake(ctx: Context<StakerTokens>, vault_id: u64, stake_amount: u64) -> Result<()> {
        stake_sol(ctx, vault_id, stake_amount)?;
        Ok(())
    }


        /*  .........      REWARDS     DISTRIBUTION   ......... */
    pub fn distribute(ctx: Context<DistributeRewards>, vault_id: u64, reward_amount: u64) -> Result<()> {
        distribute_rewards(ctx, vault_id, reward_amount)?;
        Ok(())
    }


        /*  ..........   REWARDS       CLAIMING      ..........  */
    pub fn claim(ctx: Context<ClaimRewards>, vault_id: u64) -> Result<()> {
        claim_rewards(ctx, vault_id)?;
        Ok(())
    }


        /*  ............    USER      WITHDRAWING          */
    pub fn withdraw(ctx: Context<WithdrawTokens>, vault_id: u64, withdraw_amount: u64) -> Result<()> {
        withdraw_tokens(ctx, vault_id, withdraw_amount)?;
        Ok(())
    }
}

