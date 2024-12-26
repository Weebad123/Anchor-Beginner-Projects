Program Architecture

1. Accounts

   StakerAccount: Stores information specific to the staker.
   staker: Pubkey - Wallet address of the staker.
   stake_amount: u64 - Amount of tokens staked.
   stake_time: i64 - UNIX timestamp when staking began.
   claimed_rewards: u64 - Calculated rewards claimed at each distribution epoch

   VaultAccount: Centralized account to manage staked sol of users, and receives rewards to be distributed to users
   vault_id: u64 - Distinguishes many vaults initialized by the vault Admin from each other
   total_staked: u64 - Total SOL staked in the program.
   reward_rate: u64 - Reward rate (distributed rewards from the rewarder authority per 30 days reward duration).
   bump: u8 - PDA bump for the vault.
   NB: There can be many vaults initialized by the Vault Administrator...

2. Instructions

   InitializeVault
   Initializes the VaultAccount.
   Sets the reward rate and vault PDA.
   Callable only by the vault Admin

   StakeSol
   Allows a user to stake SOL into the vault.
   Initializes a StakerAccount for the user with the stake details.
   Transfers the user's staked SOL from the user's account to the specified vault account.

   DistributeRewards
   The Rewarder Authority uses this to distributes reward amounts into a specific vault, and then
   calculates the reward rate for the vault using the distributed rewards per 30-day REWARD DURATION.

   ClaimRewards
   Allows a staker to claim his or her rewards based on his or her stake proportion in the vault and REWARDER_MULTIPLIER, and only
   when rewards have been distributed to the specified vault by the Rewarder Authority.

   - Because this project is beginner-friendly, I didn't want to utilize time-based reward calculations yet, which
     is why i utilize the REWARDER_MULTIPLIER

   WithdrawSol
   Transfers the staked sol back to the staker.
   Updates the VaultAccount and the corresponding StakerAccount.

   - Even if the User Withdraws All His Stake, his StakerAccount is not close because, the user can always
     come back and stake in that same vault in a future period...

Program Flow

    Setup
        Deploy the program and initialize the VaultAccount using InitializeVault.

    Staking
        Users invoke StakeSol to stake their SOL.
        SOL are transferred to the program's vault, and their staking information is recorded.

    Reward Calculation and Claiming
        Users can call ClaimRewards to claim their accumulated rewards in a vault.

    Withdrawal
        Users invoke WithdrawSol to reclaim staked SOL permissionlessly.

Security Considerations

    Stake and Rewards Ownership
        Use PDAs for secure Stake account ownership to prevent unauthorized access.
