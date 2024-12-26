//process.env.ANCHOR_TEST_CLOCK_DRIFT = "1";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { SystemProgram, Keypair, PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";

describe("staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  /* Writing Airdrop Function Below*/
  async function airdropSol(provider, publicKey, amountInSol) {
    const airdropSig = await provider.connection.requestAirdrop(
      publicKey,
      amountInSol * anchor.web3.LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction(airdropSig);
  }

  const program = anchor.workspace.Staking as Program<Staking>;

  /* Let's set my Actors*/
  const vaultAdmin = provider.wallet;
  const rewardDistributer = anchor.web3.Keypair.generate();
  const user1Keypair = anchor.web3.Keypair.generate();
  const user2Keypair = anchor.web3.Keypair.generate();
  const user3Keypair = anchor.web3.Keypair.generate();

  /* Let' set Up the users for airdropping */
  async function setUpUsers(provider, users, initialBalanceSol) {
    for (const user of users) {
      await airdropSol(provider, user.publicKey, initialBalanceSol);
    }
  }

  /* Let's do the actual Airdropping*/
  before(async () => {
    // Airdrop 2 SOL to the vaultAdmin
    await airdropSol(provider, vaultAdmin.publicKey, 2);

    // Airdrop 33 SOL to the rewardDistributer, but will Only Distribute 30 SOL.
    await airdropSol(provider, rewardDistributer.publicKey, 33);

    // Setup users with 10 SOL each
    await setUpUsers(provider, [user1Keypair, user2Keypair, user3Keypair], 10);
  });

  it("Correct Operation ::: Vault (ID = 1) Is Being Initialized!!!", async () => {
    // Let's call the initialize method
    await program.methods
      .initialize(new anchor.BN(1))
      .accounts({
        vaultAdmin: vaultAdmin.publicKey,
        expectedAdmin: vaultAdmin.publicKey,
      })
      .signers([])
      .rpc();

    /* Now, we are using the vault_id as seeds to find the vault address with ID =1  */
    const [vaultAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    /* Let's fetch the vault Account using the generated PDA (vaultAddress) */
    const vaultAccountIs = await program.account.vaultAccount.fetch(
      vaultAddress
    );
    // Let's log the fetched Vault Account
    //console.log(vaultAccountIs);

    // Let's Make Our Assertions
    expect(vaultAccountIs.vaultId.toNumber()).to.equal(1);
    expect(vaultAccountIs.rewardRate).to.equal(0);
    expect(vaultAccountIs.totalRewards.toNumber()).to.equal(0);
    expect(vaultAccountIs.totalStaked.toNumber()).to.equal(0);
    expect(Boolean(vaultAccountIs.isInitialized)).to.equal(true);
  });

  it("Unhappy Scenario:  =====>>>>   Vault Can Only Be Initialized By The Vault Admin", async () => {
    try {
      await program.methods
        .initialize(new anchor.BN(2))
        .accounts({
          vaultAdmin: user1Keypair.publicKey,
          expectedAdmin: provider.wallet.publicKey,
        })
        .signers([user1Keypair])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "UnauthorizedVaultAdmin");
    }
  });

  it("Unhappy Scenario: ===>>>    Vault ID Can Not Be Reinitialized Again!!", async () => {
    try {
      await program.methods
        .initialize(new anchor.BN(1))
        .accounts({
          vaultAdmin: vaultAdmin.publicKey,
          expectedAdmin: provider.wallet.publicKey,
        })
        .signers([])
        .rpc();
      assert.fail("Reinitialization Did not Throw an error as expected!");
    } catch (_err) {
      const errorString = _err.toString();
      assert(
        errorString.includes("already in use"),
        "Vault Id is Already In Use"
      );
    }
  });

  it("Correct Operation :::: User Is Staking Sol Into Vault!!", async () => {
    const userBalance = await provider.connection.getBalance(
      user1Keypair.publicKey
    );
    // Asserting That Each User Has Indeed 10 SOL before staking
    expect(userBalance).to.equal(10 * anchor.web3.LAMPORTS_PER_SOL);
    // USER 1 Amount To Stake
    const user1AmountToStake = 4 * anchor.web3.LAMPORTS_PER_SOL;
    // USER 1st Staking of 4 SOL to the vault 1
    await program.methods
      .stake(new anchor.BN(1), new anchor.BN(user1AmountToStake))
      .accounts({
        stakerAddress: user1Keypair.publicKey,
      })
      .signers([user1Keypair])
      .rpc();
    // USER 2nd staking of 4 SOL to the same vault 1 in a different transaction
    await program.methods
      .stake(new anchor.BN(1), new anchor.BN(user1AmountToStake))
      .accounts({
        stakerAddress: user1Keypair.publicKey,
      })
      .signers([user1Keypair])
      .rpc();

    // USER 2 staking 6 SOL to vault 1
    await program.methods
      .stake(new anchor.BN(1), new anchor.BN(6 * anchor.web3.LAMPORTS_PER_SOL))
      .accounts({
        stakerAddress: user2Keypair.publicKey,
      })
      .signers([user2Keypair])
      .rpc();

    // USER 3 staking 5 SOL to vault 1
    await program.methods
      .stake(new anchor.BN(1), new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL))
      .accounts({
        stakerAddress: user3Keypair.publicKey,
      })
      .signers([user3Keypair])
      .rpc();

    // Let's Get The Vault PDA
    const [vaultAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    // let's Get the Stakers PDAs
    const [staker1Address] = PublicKey.findProgramAddressSync(
      [
        user1Keypair.publicKey.toBuffer(),
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const [staker2Address] = PublicKey.findProgramAddressSync(
      [
        user2Keypair.publicKey.toBuffer(),
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const [staker3Address] = PublicKey.findProgramAddressSync(
      [
        user3Keypair.publicKey.toBuffer(),
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    /* Fetch Accounts Data for the Vault and The Various Stakers */
    const vaultAccountData = await program.account.vaultAccount.fetch(
      vaultAddress
    );

    const staker1AccountData = await program.account.stakerAccount.fetch(
      staker1Address
    );

    const staker2AccountData = await program.account.stakerAccount.fetch(
      staker2Address
    );

    const staker3AccountData = await program.account.stakerAccount.fetch(
      staker3Address
    );

    // Let's make our assertions to ascertain that the stake was successful
    expect(vaultAccountData.totalStaked.toNumber()).to.equal(
      19 * anchor.web3.LAMPORTS_PER_SOL
    );
    expect(staker1AccountData.stakeAmount.toNumber()).to.equal(
      8 * anchor.web3.LAMPORTS_PER_SOL
    );
    expect(staker2AccountData.stakeAmount.toNumber()).to.equal(
      6 * anchor.web3.LAMPORTS_PER_SOL
    );
    expect(staker3AccountData.stakeAmount.toNumber()).to.equal(
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
  });

  it("Unhappy Scenario:  ===>>>>   Staking Can Not Be Done Into An Uninitialized Vault Account 2", async () => {
    // Vault ID =2 hasn't been initialized
    try {
      await program.methods
        .stake(
          new anchor.BN(2),
          new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL)
        )
        .accounts({
          stakerAddress: user1Keypair.publicKey,
        })
        .signers([user1Keypair])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "AccountNotInitialized");
    }
  });

  it("Unhappy Scenario:   =====>>>>>   Staking Can Not Be Done With A Zero Stake Amount!", async () => {
    try {
      await program.methods
        .stake(new anchor.BN(1), new anchor.BN(0))
        .accounts({
          stakerAddress: user2Keypair.publicKey,
        })
        .signers([user2Keypair])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "ZeroStakeAmount");
    }
  });

  it("Correct Operation::: Rewards Are Being Distributed Into The Vault!!!", async () => {
    // REWARD AUTHORITY distributing 30 SOL to vault 1
    await program.methods
      .distribute(
        new anchor.BN(1),
        new anchor.BN(30 * anchor.web3.LAMPORTS_PER_SOL)
      )
      .accounts({
        rewarderAuthority: rewardDistributer.publicKey,
        expectedDistributer: rewardDistributer.publicKey,
      })
      .signers([rewardDistributer])
      .rpc();

    // Let's Get The Vault PDA
    const [vaultAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    // Let's pull up the Vault Account using the PDA
    const vaultAccountData = await program.account.vaultAccount.fetch(
      vaultAddress
    );

    // Let's ensure the rewards associated with the Vault is = to what was transferred from the reward authority
    expect(vaultAccountData.totalRewards.toNumber()).to.equal(
      30 * anchor.web3.LAMPORTS_PER_SOL
    );
    // Let's ensure the rewards rate was calculated and is not zero
    expect(vaultAccountData.rewardRate).greaterThan(0);
  });

  it("Unhappy Scenario:  =====>>>>> Only Reward Authority Can Distribute Rewards", async () => {
    try {
      await program.methods
        .distribute(
          new anchor.BN(1),
          new anchor.BN(3 * anchor.web3.LAMPORTS_PER_SOL)
        )
        .accounts({
          rewarderAuthority: user3Keypair.publicKey,
          expectedDistributer: rewardDistributer.publicKey,
        })
        .signers([user3Keypair])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "OnlyRewardAuthority");
    }
  });

  it("Unhappy Scenario:   ====>>>> Reward Authority Cannot Distribute Zero Reward Amount!", async () => {
    try {
      await program.methods
        .distribute(new anchor.BN(1), new anchor.BN(0))
        .accounts({
          rewarderAuthority: rewardDistributer.publicKey,
          expectedDistributer: rewardDistributer.publicKey,
        })
        .signers([rewardDistributer])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "ZeroRewardAmount");
    }
  });

  it("Correct Operation::: User Is Claiming Rewards From The Vault!!!", async () => {
    // Now, Let USER 2 claim rewards from the vault
    await program.methods
      .claim(new anchor.BN(1))
      .accounts({
        stakerAddress: user2Keypair.publicKey,
      })
      .signers([user2Keypair])
      .rpc();

    // Let USER 1 also claim rewards
    await program.methods
      .claim(new anchor.BN(1))
      .accounts({
        stakerAddress: user1Keypair.publicKey,
      })
      .signers([user1Keypair])
      .rpc();

    // Now, let's get the vault and staker PDAs
    const [vaultAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const [staker2Address] = PublicKey.findProgramAddressSync(
      [
        user2Keypair.publicKey.toBuffer(),
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const [staker1Address] = PublicKey.findProgramAddressSync(
      [
        user1Keypair.publicKey.toBuffer(),
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    // Let's Get The Staker And Vault Data to make some assertions
    const staker1AccountData = await program.account.stakerAccount.fetch(
      staker1Address
    );
    const staker2AccountData = await program.account.stakerAccount.fetch(
      staker2Address
    );
    const vaultAccountData = await program.account.vaultAccount.fetch(
      vaultAddress
    );
    // Checking That The claimed Rewards of the stakers is Non-Zero indicates That They Receive The Rewards
    // Let's make some assertions
    const remainingRewards =
      30 * anchor.web3.LAMPORTS_PER_SOL -
      (staker1AccountData.claimedRewards.toNumber() +
        staker2AccountData.claimedRewards.toNumber());
    expect(vaultAccountData.totalRewards.toNumber()).to.equal(remainingRewards);
    expect(staker1AccountData.claimedRewards.toNumber()).greaterThan(0);
    expect(staker2AccountData.claimedRewards.toNumber()).greaterThan(0);
  });

  it("Unhappy Scenario:  ===>>>>>>  User Cannot Claim Twice Within The Same Distribution!", async () => {
    // Let User3 First Claim, The 2nd claim of User3 in a different txn will fail with AlreadyClaimed error
    await program.methods
      .claim(new anchor.BN(1))
      .accounts({
        stakerAddress: user3Keypair.publicKey,
      })
      .signers([user3Keypair])
      .rpc();

    // Let User 3 attempt a second claim, it will fail
    try {
      await program.methods
        .claim(new anchor.BN(1))
        .accounts({
          stakerAddress: user3Keypair.publicKey,
        })
        .signers([user3Keypair])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "AlreadyClaimed");
    }
  });

  it("Unhappy Scenario: ====>>>>> User Who Haven't Staked Into The Vault Cannot Claim!", async () => {
    const userNotStaker = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .claim(new anchor.BN(1))
        .accounts({
          stakerAddress: userNotStaker.publicKey,
        })
        .signers([userNotStaker])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      //console.log("Errors logged Are:", err);
      assert.strictEqual(err.error.errorCode.code, "AccountNotInitialized");
    }
  });

  it("Correct Operation::: User Is Withdrawing Staked Sol From The Vault!!", async () => {
    const withdrawal_amount = 3 * anchor.web3.LAMPORTS_PER_SOL;
    // USER 3 staked 5 SOL, now he's withdrawing 3 SOL, so remaining should be 2 SOL
    await program.methods
      .withdraw(new anchor.BN(1), new anchor.BN(withdrawal_amount))
      .accounts({
        stakerAddress: user3Keypair.publicKey,
      })
      .signers([user3Keypair])
      .rpc();

    // USER 1 staked 8 SOL. but withdrawing 5 SOL, so remaining should be 3 SOL
    await program.methods
      .withdraw(
        new anchor.BN(1),
        new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL)
      )
      .accounts({
        stakerAddress: user1Keypair.publicKey,
      })
      .signers([user1Keypair])
      .rpc();

    const [user3Address] = PublicKey.findProgramAddressSync(
      [
        user3Keypair.publicKey.toBuffer(),
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const [user1address] = PublicKey.findProgramAddressSync(
      [
        user1Keypair.publicKey.toBuffer(),
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const [vaultAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const vaultAccount = await program.account.vaultAccount.fetch(vaultAddress);
    const user3Account = await program.account.stakerAccount.fetch(
      user3Address
    );
    const user1Account = await program.account.stakerAccount.fetch(
      user1address
    );

    // Let's make our assertions here with regards to stakes
    expect(user1Account.stakeAmount.toNumber()).to.equal(
      3 * anchor.web3.LAMPORTS_PER_SOL
    );
    expect(user3Account.stakeAmount.toNumber()).to.equal(
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    expect(vaultAccount.totalStaked.toNumber()).to.equal(
      11 * anchor.web3.LAMPORTS_PER_SOL
    );
  });

  it("Unhappy Scenario: ===>>> User Cannot Withdraw More Than Staked Amount In The Vault", async () => {
    // Testing That User 2 who has staked 6 SOL cannot withdraw 7 SOL
    try {
      await program.methods
        .withdraw(
          new anchor.BN(1),
          new anchor.BN(7 * anchor.web3.LAMPORTS_PER_SOL)
        )
        .accounts({
          stakerAddress: user2Keypair.publicKey,
        })
        .signers([user2Keypair])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(
        err.error.errorCode.code,
        "InsufficientAmountToWithdraw"
      );
    }
  });

  it("Unhappy Scenario: ====>>>>> User Cannot Withdraw Zero Amount!", async () => {
    // Testing that 0 amount withdrawals is not possible
    try {
      await program.methods
        .withdraw(new anchor.BN(1), new anchor.BN(0))
        .accounts({
          stakerAddress: user2Keypair.publicKey,
        })
        .signers([user2Keypair])
        .rpc();
    } catch (_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "ZeroWithdrawAmount");
    }
  });
});
